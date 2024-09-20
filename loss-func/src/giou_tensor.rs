// use burn::{
//     config::Config,
//     module::{Module, Param, ParamId},
//     nn::{Dropout, DropoutConfig, Initializer, LeakyRelu, LeakyReluConfig, Linear, LinearConfig},
//     prelude::Backend,
//     tensor::{Distribution, Int, Shape, Tensor},
// };

use burn::{
    tensor::{Tensor},
};

// use burn::backend::Wgpu;


fn loss_sum(truths: &Tensor<B, 2>, predicts: &Tensor<B, 2>) -> f64 {
    let mut sum = 0.0;
    let len = truths.len();

    for i in 0..len {
        let mut max: f64 = loss(&truths[0], &predicts[i]);
        let mut max_truth_idx = 0;
        println!("Searching through truths: {:?} for better loss than loss: {}", truths, max);
        for j in 0..truths.len() {
            let loss = loss(&truths[j], &predicts[i]);
            println!("Truth: {:?}, Predict: {:?}, Loss: {}", truths[j], predicts[i], loss);
            if loss > max {
                println!("Found best match with loss: {}", loss);
                max = loss;
                max_truth_idx = j;
            }
        }
        truths.remove(max_truth_idx);

        sum += max;
    }

    return sum / len as f64;

}

fn loss(truth: &Vec<f64>, predict: &[f64; 4]) -> f64 {
    // Assign values
    let t_l1 = truth[0];
    let t_l2 = truth[1];
    let t_r1= truth[2];
    let t_r2= truth[3];

    let p_l1 = predict[0];
    let p_l2 = predict[1];
    let p_r1 = predict[2];
    let p_r2 = predict[3];

    // Calculate each loss
    let l_giou = giou(t_l1, t_l2, p_l1, p_l2); 
    let r_giou = giou(t_r1, t_r2, p_r1, p_r2);

    let loss = l_giou + r_giou; // this is where we would apply function g

    return loss;
}

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

// fn main() {
//     type MyBackend = Wgpu<f32, i32>;
//     let device = Default::default();

//     let truth_floats = [[27.0, 33.0, 30.0, 35.0], [3.0, 9.0, 4.0, 10.0]];
//     let predict_floats = [[3.0, 10.5, 4.0, 10.0], [27.5, 33.01, 29.99, 34.45], [12.3, 14.2, 33.5, 33.7]];
//     let truth = &mut Tensor::<MyBackend, 2>::from_floats(truth_floats, &device);
//     let predict = Tensor::<MyBackend, 2>::from_floats(predict_floats, &device);

//     // let truth: &mut Vec<Vec<f64>> = &mut vec![vec![27.0, 33.0, 30.0, 35.0], vec![3.0, 9.0, 4.0, 10.0]];
//     // let predict: [[f64; 4]; 3] = [[3.0, 10.5, 4.0, 10.0], [27.5, 33.01, 29.99, 34.45], [12.3, 14.2, 33.5, 33.7]];

//     let loss = loss_sum(truth, &predict);
//     println!("Loss: {}", loss);
// }