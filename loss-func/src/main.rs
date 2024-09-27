use burn::tensor::Tensor;
use burn::backend::Wgpu;
use burn::prelude::Float;
use std::ops::Rem;
use burn::prelude::Int;


// mod giou_tensor;
// mod giou;

// Type alias for the backend to use.
type Backend = Wgpu;


fn loss_sum(truths: &Tensor<Backend, 2>, predicts: &Tensor<Backend, 2>) -> f64 {
    let mut sum = 0.0;
    let len = truths.dims()[0];
    println!("Length of truths matrix (num of truths): {}", len);

    for i in 0..len {
        // let mut max: f64 = loss(&truths.select(0, Tensor::<Backend, 1>::from_floats()), &predicts.select(0, i));
        let idx_0 = Tensor::<Backend, 1, Int>::from_ints([0, 1], &truths.device());
        let idx_i = Tensor::<Backend, 1, Int>::from_ints([i, i+1], &truths.device());
        let mut max: f64 = loss(&truths.select(1, idx_0), &predicts.select(0, idx_i));

        let mut max_truth_idx = 0;
        println!("Searching through truths: {:?} for better loss than loss: {}", truths, max);
        for j in 0..truths.dims()[0] {
            let idx_j = Tensor::<Backend, 1, Int>::from_ints([j, j+1], &truths.device());
            let loss = loss(&truths.select(0, idx_j), &predicts.select(0, idx_i));
            println!("Truth: {:?}, Predict: {:?}, Loss: {}", truths.select(0, idx_j), predicts.select(0, idx_i), loss);
            if loss > max {
                println!("Found best match with loss: {}", loss);
                max = loss;
                max_truth_idx = j;
            }
        }
        truths.rem(max_truth_idx);

        sum += max;
    }

    return sum / len as f64;

}

fn loss(truth: &Tensor<Backend, 2>, predict: &Tensor<Backend, 2>) -> f64 {
    // Assign values
    let t_l1 = truth.select(0, Tensor::<Backend, 1, Int>::from_ints([0 ,0+1], &truth.device()));
    let t_l2 = truth.select(0, Tensor::<Backend, 1, Int>::from_ints([1 ,1+1], &truth.device()));
    let t_r1= truth.select(0, Tensor::<Backend, 1, Int>::from_ints([ 2, 21], &truth.device()));
    let t_r2= truth.select(0, Tensor::<Backend, 1, Int>::from_ints([ 3, 31], &truth.device()));

    let p_l1 = predict.select(0, Tensor::<Backend, 1, Int>::from_ints([0 ,0+1], &truth.device()));
    let p_l2 = predict.select(0, Tensor::<Backend, 1, Int>::from_ints([1 ,1+1], &truth.device()));
    let p_r1 = predict.select(0, Tensor::<Backend, 1, Int>::from_ints([2 ,2+1], &truth.device()));
    let p_r2 = predict.select(0, Tensor::<Backend, 1, Int>::from_ints([3 ,3+1], &truth.device()));

    

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

    return loss;
}

#[allow(dead_code)]
fn giou(t1: f64, t2: f64, p1: f64, p2: f64) -> f64 {
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

    // // let truth: &mut Vec<Vec<f64>> = &mut vec![vec![27.0, 33.0, 30.0, 35.0], vec![3.0, 9.0, 4.0, 10.0]];
    // // let predict: [[f64; 4]; 3] = [[3.0, 10.5, 4.0, 10.0], [27.5, 33.01, 29.99, 34.45], [12.3, 14.2, 33.5, 33.7]];

    // let loss = loss_sum(truth, &predict);
    // println!("Loss: {}", loss);



    // let floats = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];



    // correct: Tensor is 1-Dimensional with 5 elements
    // let tensor_1 = Tensor::<Backend, 1, Float>::from_floats(floats, &device);

    let num_predictions: usize = predict.dims()[0];

    // let first_row = predict.slice(0);
    // println!("{:?}", first_row);
    // println!("length of preds (should be 3): {}", num_predictions);

    // let loss = loss_sum(truth, &predict);
    

    // println!("{:?}", tensor_1.dims().iter().product());
    // print tensor_1 values
    // println!("{:?}", tensor_1.slice([1]));
    // println!("{:?}", truth.clone().slice(1));
    // println!("{:?}", predict.clone().all());

}