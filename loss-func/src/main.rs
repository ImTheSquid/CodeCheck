#![feature(generic_const_exprs)]


use burn::tensor::Tensor;
use burn::backend::Wgpu;
use burn::prelude::Float;
use burn::prelude::Int;
use ndarray::prelude::*;


// mod giou_tensor;
// mod giou;

// Type alias for the backend to use.
type Backend = Wgpu;




fn loss_sum(truths: &Tensor<Backend, 2>, predicts: &Tensor<Backend, 2>) -> Tensor<Backend, 1> {
    // let mut sum = 0.0;
    let len = predicts.dims()[0];
    let shape = predicts.dims();

    // the results loss tensor will be of equal length to the number of predictions
    // for predictions made after all truths are exausted, a loss of n will be applied with n>>0
    let max_loss = 100.0;

    // let mut loss_floats = [0.0; 2];
    let mut loss_floats = vec![0.0; len];
    // let loss = Tensor::<Backend, 1, Float>::from_floats([0.0], &truths.device());

    // println!("Length of predict matrix (num of truths): {}", len);

    let mut used = <Vec<usize>>::new();
    // let mut used = <Vec<i32>>::new();

    for i in 0..len {
        // println!("Currently used is {:?} and i is {}", used, i);
        if used.len() == truths.dims()[0] {
            // println!("All truths have been used");
            loss_floats[i] = max_loss;
            break;
        }

        // let mut max: f64 = loss(&truths.select(0, Tensor::<Backend, 1>::from_floats()), &predicts.select(0, i));
        let idx_0 = Tensor::<Backend, 1, Int>::from_ints([0], &truths.device());
        let idx_i = Tensor::<Backend, 1, Int>::from_ints([i], &truths.device());
        let mut max: f64 = loss(&truths.clone().select(0, idx_0.clone()).flatten::<1>(0, 1), &predicts.clone().select(0, idx_i.clone()).flatten::<1>(0, 1));

        let mut max_truth_idx = 0;
        // println!("\nSearching through truths for better loss than loss: {}", max);
        for j in 0..truths.dims()[0] {
            if !used.contains(&j) {
                let idx_j = Tensor::<Backend, 1, Int>::from_ints([j], &truths.device());
                let loss = loss(&truths.clone().select(0, idx_j.clone()).flatten::<1>(0, 1), &predicts.clone().select(0, idx_i.clone()).flatten::<1>(0, 1));
                // println!("Truth: {:?}, Predict: {:?}, Loss: {}", truths.clone().select(0, idx_j.clone()), predicts.clone().select(0, idx_i.clone()), loss);
                if loss < max {
                    // println!("Found best match with loss: {}", loss);
                    max = loss;
                    max_truth_idx = j;
                }
            }
            loss_floats[i] = max;
        }
        
        // used.push(max_truth_idx);

        // sum += max;
    }
    // println!("loss_floats: {:?}", loss_floats);
    // return loss_floats
    return Tensor::<Backend, 1, Float>::from_floats(loss_floats.as_slice(), &truths.device());
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

    // Calculate each loss
    let l_giou = giou(t_l1, t_l2, p_l1, p_l2); 
    let r_giou = giou(t_r1, t_r2, p_r1, p_r2);

    let loss = -1.0 * (l_giou + r_giou); // this is where we would apply function g
    // println!("loss between truth: {} and prediction {} is {}", truth, predict, loss);
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


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

// fn get_index<const D: usize>(array: &Tensor<Backend, {D}, Float>, mut index: i32) -> Tensor<Backend, 1, Int> {
fn get_index<const D: usize>(array: &Tensor<Backend, {D}, Float>, mut index: i32) -> Vec<i32> {

    let mut total = array.dims().iter().product::<usize>() as i32;

    if index < 0 || index >= total {
        panic!("Index out of bounds");
    }

    let mut idx = vec![];

    for i in 0..D {
        total = (total as f64 / array.dims()[i] as f64).floor() as i32;
        idx.push((index as f64/ total as f64).floor() as i32);
        index = index % total;
    }

    // println!("Index: {:?}", idx);   
    return idx;
    // return Tensor::<Backend, 1, Int>::from_ints(idx.as_slice(), &array.device());
}


fn forward<const D: usize>(predict: &Tensor<Backend, D, Float>, truth: &Tensor<Backend, D, Float>) -> Tensor<Backend, {D - 1}, Float> {
    let mut loss_arr: Vec<Tensor<Backend, 1>> = Vec::new();

    let mut shape: [usize; D] = predict.dims();
    println!("Shape: {:?}", shape);

    if shape.len() != 3 {
        panic!("Only 3D tensors are supported");
    }

    for i in 0..shape[0] {
        println!("\ndim0 idx: {}", i);

        let mut new_truth = truth.clone().select(0, Tensor::<Backend, 1, Int>::from_ints([i], &truth.device())).squeeze::<2>(0);
        let mut new_predict = predict.clone().select(0, Tensor::<Backend, 1, Int>::from_ints([i], &truth.device())).squeeze::<2>(0);

        // println!("New truth: {}", new_truth);
        // println!("New predict: {}", new_predict);

        let loss = loss_sum(&new_truth, &new_predict);
        // println!("Loss: {:?}", loss);

        loss_arr.push(loss);

    }
    // println!("Loss array: {:?}", loss_arr);

    return Tensor::stack(loss_arr, 0);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // type MyBackend = Wgpu<f32, i32>;
    // let device = Default::default();
        
    // Get the default device
    let device = Default::default();



    // TESTING

    // equal arrays same length
    let truth_floats = [[[3.0, 10.5, 4.0, 10.0], [27.5, 33.01, 29.99, 34.45], [12.3, 14.2, 33.5, 33.7]]];
    let predict_floats = [[[3.0, 10.5, 4.0, 10.0], [27.5, 33.01, 29.99, 34.45], [12.3, 14.2, 33.5, 33.7]]];

    let truth = &mut Tensor::<Backend, 3, Float>::from_floats(truth_floats, &device);
    let predict = Tensor::<Backend, 3, Float>::from_floats(predict_floats, &device);


    let loss = forward(&predict, truth);
    println!("Loss: {}", loss);

    // equal arrays different length
    let truth_floats = [[[3.0, 10.5, 4.0, 10.0], [27.5, 33.01, 29.99, 34.45]]];
    let predict_floats = [[[3.0, 10.5, 4.0, 10.0], [27.5, 33.01, 29.99, 34.45], [27.5, 33.01, 29.99, 34.45]]];

    let truth = &mut Tensor::<Backend, 3, Float>::from_floats(truth_floats, &device);
    let predict = Tensor::<Backend, 3, Float>::from_floats(predict_floats, &device);


    let loss = forward(&predict, truth);
    println!("Loss: {}", loss);

    // unequal arrays - close
    let truth_floats = [[[3.1, 10.5, 4.3, 10.0], [27.4, 33.01, 28.99, 34.45]]];
    let predict_floats = [[[3.0, 10.5, 4.0, 9.2], [27.5, 33.01, 29.99, 34.45], [27.8, 33.22, 29.99, 34.45]]];

    let truth = &mut Tensor::<Backend, 3, Float>::from_floats(truth_floats, &device);
    let predict = Tensor::<Backend, 3, Float>::from_floats(predict_floats, &device);


    let loss = forward(&predict, truth);
    println!("Loss: {}", loss);

    // unequal arrays - far
    let truth_floats = [[[3.1, 10.5, 4.3, 10.0], [27.4, 33.01, 28.99, 34.45]]];
    let predict_floats = [[[32.0, 120.5, 24.0, 92.2], [247.5, 433.01, 219.99, 364.45], [727.8, 323.22, 219.99, 314.45]]];

    let truth = &mut Tensor::<Backend, 3, Float>::from_floats(truth_floats, &device);
    let predict = Tensor::<Backend, 3, Float>::from_floats(predict_floats, &device);


    let loss = forward(&predict, truth);
    println!("Loss: {}", loss);

    // higher dim arrays
    let truth_floats = [
        [[3.1, 10.5, 4.3, 10.0], [27.4, 33.01, 28.99, 34.45]],
        [[2.4, 22.1, 3.99, 10.45], [27.4, 33.01, 28.99, 34.45]],
        [[3.1, 10.5, 4.3, 10.0], [27.4, 33.01, 28.99, 34.45]]
    ];

    let predict_floats = [
        [[32.0, 120.5, 24.0, 92.2], [247.5, 433.01, 219.99, 364.45], [727.8, 323.22, 219.99, 314.45]],
        [[32.0, 120.5, 24.0, 92.2], [247.5, 433.01, 219.99, 364.45], [727.8, 323.22, 219.99, 314.45]],
        [[3.0, 10.5, 4.0, 9.2], [27.5, 33.01, 29.99, 34.45], [27.8, 33.22, 29.99, 34.45]]
    ];

    let truth = &mut Tensor::<Backend, 3, Float>::from_floats(truth_floats, &device);
    let predict = Tensor::<Backend, 3, Float>::from_floats(predict_floats, &device);


    let loss = forward(&predict, truth);
    println!("Loss: {}", loss);

}