use burn::tensor::Tensor;
use burn::backend::Wgpu;
use burn::prelude::Float;
use burn::prelude::Int;


// mod giou_tensor;
// mod giou;

// Type alias for the backend to use.
type Backend = Wgpu;


fn loss_sum(truths: &Tensor<Backend, 2>, predicts: &Tensor<Backend, 2>) -> f64 {
    let mut sum = 0.0;
    let len = truths.dims()[0];
    println!("Length of truths matrix (num of truths): {}", len);

    let mut used = <Vec<usize>>::new();
    // let mut used = <Vec<i32>>::new();

    for i in 0..len {
        // let mut max: f64 = loss(&truths.select(0, Tensor::<Backend, 1>::from_floats()), &predicts.select(0, i));
        let idx_0 = Tensor::<Backend, 1, Int>::from_ints([0], &truths.device());
        let idx_i = Tensor::<Backend, 1, Int>::from_ints([i], &truths.device());
        let mut max: f64 = loss(&truths.select(1, idx_0).flatten::<1>(0, 1), &predicts.select(0, idx_i).flatten::<1>(0, 1));

        let mut max_truth_idx = 0;
        println!("Searching through truths: {:?} for better loss than loss: {}", truths, max);
        for j in 0..truths.dims()[0] {
            if !used.contains(&j) {
                let idx_j = Tensor::<Backend, 1, Int>::from_ints([j], &truths.device());
                let loss = loss(&truths.select(0, idx_j).flatten::<1>(0, 1), &predicts.select(0, idx_i).flatten::<1>(0, 1));
                println!("Truth: {:?}, Predict: {:?}, Loss: {}", truths.select(0, idx_j), predicts.select(0, idx_i), loss);
                if loss > max {
                    println!("Found best match with loss: {}", loss);
                    max = loss;
                    max_truth_idx = j;
                }
            }

        }
        
        used.push(max_truth_idx);

        // truths = truths.dims().iter()
        //     .enumerate()
        //     .filter(|&(i, _)| i != max_truth_idx)
        //     .map(|(_, row)| row.clone())
        //     .collect();

        sum += max;
    }

    return sum / len as f64;

}

fn loss(truth: &Tensor<Backend, 1>, predict: &Tensor<Backend, 1>) -> f64 {
    // Assign values
    let t_l1 = truth.select(0, Tensor::<Backend, 1, Int>::from_ints([0], &truth.device())).into_scalar();
    let t_l2 = truth.select(0, Tensor::<Backend, 1, Int>::from_ints([1], &truth.device())).into_scalar();
    let t_r1= truth.select(0, Tensor::<Backend, 1, Int>::from_ints([2], &truth.device())).into_scalar();
    let t_r2= truth.select(0, Tensor::<Backend, 1, Int>::from_ints([3], &truth.device())).into_scalar();

    let p_l1 = predict.select(0, Tensor::<Backend, 1, Int>::from_ints([0], &truth.device())).into_scalar();
    let p_l2 = predict.select(0, Tensor::<Backend, 1, Int>::from_ints([1], &truth.device())).into_scalar();
    let p_r1 = predict.select(0, Tensor::<Backend, 1, Int>::from_ints([2], &truth.device())).into_scalar();
    let p_r2 = predict.select(0, Tensor::<Backend, 1, Int>::from_ints([3], &truth.device())).into_scalar();



    // let t_l1 = truth[0];
    // let t_l2 = truth[1];
    // let t_r1= truth[2];
    // let t_r2= truth[3];

    // let p_l1 = predict[0];
    // let p_l2 = predict[1];
    // let p_r1 = predict[2];
    // let p_r2 = predict[3];

    // Calculate each loss
    let l_giou = giou(t_l1, t_l2, p_l1, p_l2); 
    let r_giou = giou(t_r1, t_r2, p_r1, p_r2);

    let loss = l_giou + r_giou; // this is where we would apply function g

    return loss.into();
}

#[allow(dead_code)]
fn giou(t1: f32, t2: f32, p1: f32, p2: f32) -> f32 {
    // println!("t1: {}, t2: {}, p1: {}, p2: {}", t1, t2, p1, p2);
    let intersection = (t2.min(p2) - t1.max(p1)).max(0.0);
    // println!("Intersection: {}", intersection);
    let area = p2 - p1 + t2 - t1 - intersection;
    // println!("Area: {}", area);
    let mut entire = p2.max(p2) - t1.min(p1);
    // println!("Entire: {}", entire);

    entire = entire / area; // this is where we would apply function f(C)

    let overlap = ((intersection / area) - (entire)).into();
    // println!("Overlap: {}", overlap);
    return overlap;
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // type MyBackend = Wgpu<f32, i32>;
    // let device = Default::default();
        
    // Get the default device
    let device = Default::default();

    let truth_floats = [[27.0, 33.0, 30.0, 35.0], [3.0, 9.0, 4.0, 10.0]];
    let predict_floats = [[3.0, 10.5, 4.0, 10.0], [27.5, 33.01, 29.99, 34.45], [12.3, 14.2, 33.5, 33.7]];
    let truth = &mut Tensor::<Backend, 2, Float>::from_floats(truth_floats, &device);
    let predict = Tensor::<Backend, 2, Float>::from_floats(predict_floats, &device);


    // let floats = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    
    // print the predict tensor
    let mut used: Vec<i32> = Vec::new();
    used.push(1);
    println!("{:?}", used);
    if used.contains(&1) {
        println!("Found 1 in used");
    }
    let tensor_str = predict.flatten::<1>(0, 1).select(0, Tensor::<Backend, 1, Int>::from_ints([2], &device)).into_scalar();

    println!("{}", tensor_str);
    // println!("{:?}", predict);
    
    // print the predict tensor flattened to 1d
    // println!("{:?}", predict.flatten::<2>(0, 1));

}