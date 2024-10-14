use burn::{
    prelude::Backend,
    tensor::{Int, Tensor},
    train::metric::{Adaptor, LossInput},
};


// added this impl - basically all the same functions as main.rs, but changed all calls to be self.function_name
// I dont really have a way to test it
// TODO - clean print statements, remove unnecessary code, document code bett
impl<B: Backend> GIOULoss<B> {
    pub fn forward<const D: usize>(predict: &Tensor<Backend, D, Float>, truth: &Tensor<Backend, D, Float>) -> Tensor<Backend, {D - 1}, Float> {
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
    
            let loss = self.loss_sum(&new_truth, &new_predict);
            // println!("Loss: {:?}", loss);
    
            loss_arr.push(loss);
    
        }
        // println!("Loss array: {:?}", loss_arr);
    
        return Tensor::stack(loss_arr, 0);
    }

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
        let l_giou = self.giou(t_l1, t_l2, p_l1, p_l2); 
        let r_giou = self.giou(t_r1, t_r2, p_r1, p_r2);
    
        let loss = -1.0 * (l_giou + r_giou); // this is where we would apply function g
        // println!("loss between truth: {} and prediction {} is {}", truth, predict, loss);
        return loss.into();
    }

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
            let mut max: f64 = self.loss(&truths.clone().select(0, idx_0.clone()).flatten::<1>(0, 1), &predicts.clone().select(0, idx_i.clone()).flatten::<1>(0, 1));
    
            let mut max_truth_idx = 0;
            // println!("\nSearching through truths for better loss than loss: {}", max);
            for j in 0..truths.dims()[0] {
                if !used.contains(&j) {
                    let idx_j = Tensor::<Backend, 1, Int>::from_ints([j], &truths.device());
                    let loss = self.loss(&truths.clone().select(0, idx_j.clone()).flatten::<1>(0, 1), &predicts.clone().select(0, idx_i.clone()).flatten::<1>(0, 1));
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
}

#[derive(Debug, Clone)]
pub struct ObjectnessOutput<B: Backend> {
    pub output: Tensor<B, 2>,
    pub targets: Tensor<B, 2, Int>,
}

#[derive(Debug, Clone)]
pub struct BatchedRegressionOutput<B: Backend> {
    pub output: Tensor<B, 3>,
    pub targets: Tensor<B, 3, Int>,
}

#[derive(Debug, Clone)]
pub struct ModelOutput<B: Backend> {
    pub loss: Tensor<B, 1>,
    pub regression: BatchedRegressionOutput<B>,
    pub objectness: ObjectnessOutput<B>,
}

impl<B: Backend> Adaptor<LossInput<B>> for ModelOutput<B> {
    fn adapt(&self) -> LossInput<B> {
        LossInput::new(self.loss.clone())
    }
}
