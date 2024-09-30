use burn::tensor::Tensor;
use burn::backend::Wgpu;
use burn::prelude::Float;
use burn::prelude::Int;


// mod giou_tensor;
// mod giou;

// Type alias for the backend to use.
type Backend = Wgpu;


fn loss_sum(truths: &Tensor<Backend, 2>, predicts: &Tensor<Backend, 2>) -> Tensor<Backend, 1> {
    // let mut sum = 0.0;
    let len = predicts.dims()[0];

    // the results loss tensor will be of equal length to the number of predictions
    // for predictions made after all truths are exausted, a loss of n will be applied with n>>0
    let max_loss = 100.0;

    let mut loss_floats = [0.0; 3];
    // let loss = Tensor::<Backend, 1, Float>::from_floats([0.0], &truths.device());

    println!("Length of predict matrix (num of truths): {}", len);

    let mut used = <Vec<usize>>::new();
    // let mut used = <Vec<i32>>::new();

    for i in 0..len {
        if used.len() + 1 == truths.dims()[0] {
            println!("All truths have been used");
            loss_floats[i] = max_loss;
            break;
        }

        // let mut max: f64 = loss(&truths.select(0, Tensor::<Backend, 1>::from_floats()), &predicts.select(0, i));
        let idx_0 = Tensor::<Backend, 1, Int>::from_ints([0], &truths.device());
        let idx_i = Tensor::<Backend, 1, Int>::from_ints([i], &truths.device());
        let mut max: f64 = loss(&truths.clone().select(1, idx_0.clone()).flatten::<1>(0, 1), &predicts.clone().select(0, idx_i.clone()).flatten::<1>(0, 1));

        let mut max_truth_idx = 0;
        println!("Searching through truths for better loss than loss: {}", max);
        for j in 0..truths.dims()[0] {
            if !used.contains(&j) {
                let idx_j = Tensor::<Backend, 1, Int>::from_ints([j], &truths.device());
                let loss = loss(&truths.clone().select(0, idx_j.clone()).flatten::<1>(0, 1), &predicts.clone().select(0, idx_i.clone()).flatten::<1>(0, 1));
                // println!("Truth: {:?}, Predict: {:?}, Loss: {}", truths.clone().select(0, idx_j.clone()), predicts.clone().select(0, idx_i.clone()), loss);
                if loss > max {
                    println!("Found best match with loss: {}", loss);
                    max = loss;
                    max_truth_idx = j;
                }
            }
            loss_floats[i] = max;
        }
        
        used.push(max_truth_idx);

        // sum += max;
    }
    return Tensor::<Backend, 1, Float>::from_floats(loss_floats, &truths.device());
    // return sum / len as f64;

}

fn loss(truth: &Tensor<Backend, 1>, predict: &Tensor<Backend, 1>) -> f64 {
    // Assign values
    let t_l1 = truth.clone().select(0, Tensor::<Backend, 1, Int>::from_ints([0], &truth.device())).into_scalar();
    let t_l2 = truth.clone().select(0, Tensor::<Backend, 1, Int>::from_ints([1], &truth.device())).into_scalar();
    let t_r1= truth.clone().select(0, Tensor::<Backend, 1, Int>::from_ints([2], &truth.device())).into_scalar();
    let t_r2= truth.clone().select(0, Tensor::<Backend, 1, Int>::from_ints([3], &truth.device())).into_scalar();

    let p_l1 = predict.clone().select(0, Tensor::<Backend, 1, Int>::from_ints([0], &truth.device())).into_scalar();
    let p_l2 = predict.clone().select(0, Tensor::<Backend, 1, Int>::from_ints([1], &truth.device())).into_scalar();
    let p_r1 = predict.clone().select(0, Tensor::<Backend, 1, Int>::from_ints([2], &truth.device())).into_scalar();
    let p_r2 = predict.clone().select(0, Tensor::<Backend, 1, Int>::from_ints([3], &truth.device())).into_scalar();



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

// impl<B: Backend> iou_loss<B> {
//     pub forward<const D: usize>(
//         &self,
//         truth: &Tensor<B, D, Float>,
//         predict: &Tensor<B, D, Float>,
//     ) -> Tensor<B, D-1> {
//         let dim = truth.dims();

//     }
// }


#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // type MyBackend = Wgpu<f32, i32>;
    // let device = Default::default();
        
    // Get the default device
    let device = Default::default();

    let truth_floats = [[27.0, 33.0, 30.0, 35.0], [3.0, 9.0, 4.0, 10.0]];
    let truth_floats = [[3.0, 10.5, 4.0, 10.0], [27.5, 33.01, 29.99, 34.45], [12.3, 14.2, 33.5, 33.7]];

    let predict_floats = [[3.0, 10.5, 4.0, 10.0], [27.5, 33.01, 29.99, 34.45], [12.3, 14.2, 33.5, 33.7]];
    let truth = &mut Tensor::<Backend, 2, Float>::from_floats(truth_floats, &device);
    let predict = Tensor::<Backend, 2, Float>::from_floats(predict_floats, &device);


    let loss = loss_sum(truth, &predict);
    println!("Loss: {}", loss); 

    let dim_ten_floats = 
    [
        [
            [[1.3, 2.3, 1.2, 2.2], [1.0, 5.0, 10.2, 11.2]], 
            [[34.0, 39.0, 11.3, 11.7], [99.1, 100.1, 3.3, 5.5]]
        ], 
        [
            [[27.0, 33.0, 30.0, 35.0], [3.0, 9.0, 4.0, 10.0]], 
            [[27.0, 33.0, 30.0, 35.0], [3.0, 10.5, 4.0, 10.0]]
        ], 
        [
            [[34.0, 39.0, 11.3, 11.7], [12.3, 14.2, 33.5, 33.7]], 
            [[12.3, 14.2, 33.5, 33.7], [34.0, 39.0, 11.3, 11.7]]
        ]
    ]; 

    let dim_ten = Tensor::<Backend, 4, Float>::from_floats(dim_ten_floats, &device);

    // let shape = dim_ten.dims();
    // // iterate over sub-tensors
    // for i in 0..shape[0] {
    //     for j in 0..shape[1] {
    //         let sub_tensor = dim_ten.clone().select(0, Tensor::<Backend, 1, Int>::from_ints([i], &device))
    //             .select(0, Tensor::<Backend, 1, Int>::from_ints([j], &device));
    //         println!("Sub-tensor: {:?}", sub_tensor);
    //     }
    // }
    
    // println!("Shape: {:?}", shape);

    // 3 by 2 by 2 by 4 vector = dim 4

    // let floats = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    
    // print the predict tensor
    // let mut used: Vec<i32> = Vec::new();
    // used.push(1);
    // println!("{:?}", used);
    // if used.contains(&1) {
    //     println!("Found 1 in used");
    // }
    // let tensor_str = predict.flatten::<1>(0, 1).select(0, Tensor::<Backend, 1, Int>::from_ints([2], &device)).into_scalar();

    // println!("{}", tensor_str);
    // println!("{:?}", predict);
    
    // print the predict tensor flattened to 1d
    // println!("{:?}", predict.flatten::<2>(0, 1));

}