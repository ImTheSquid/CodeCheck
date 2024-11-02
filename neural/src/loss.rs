use core::f64;
use std::{f64::EPSILON, marker::PhantomData};

use burn::{
    prelude::Backend,
    tensor::{cast::ToElement, Int, Tensor},
    train::metric::{Adaptor, LossInput},
};

#[derive(Debug, Default)]
pub struct GIOULoss<B: Backend> {
    _backend: PhantomData<B>,
}

/// A minimum value to ensure area is never 0 (leading to a NaN)
const EPSILON_MIN: f64 = 1e-7;
const EMPTY_VALUE: f64 = -1.0;
const THRESHOLD: f64 = 0.3;
const PENALTY: f64 = 1.0;

// added this impl - basically all the same functions as main.rs, but changed all calls to be self.function_name
// I dont really have a way to test it
// TODO - clean print statements, remove unnecessary code, document code bett
impl<B: Backend> GIOULoss<B> {
    pub fn forward<const D: usize>(
        &self,
        predict: Tensor<B, D>,
        truth: Tensor<B, D>,
    ) -> Tensor<B, D>
    where
        [(); D + 1]:,
    {
        let empty_truth_mask = truth.clone().equal_elem(EMPTY_VALUE).all_dim(D - 1);

        let giou = self.masked_giou(predict, truth);

        let best = giou.max_dim(D - 2);

        // Empty truth by default has no loss, so make it such that 1 - 1 = 0
        let actual_loss = best.ones_like() - best.clone().mask_fill(empty_truth_mask.clone(), 1.0);

        let penalty = best
            .lower_elem(THRESHOLD)
            .float()
            .mask_fill(empty_truth_mask, 0.0)
            * PENALTY;

        actual_loss + penalty
    }

    /// Ignores -1's
    pub fn masked_giou<const D: usize>(
        &self,
        predict: Tensor<B, D>,
        truth: Tensor<B, D>,
    ) -> Tensor<B, D>
    where
        [(); D + 1]:,
    {
        let predict_mask = predict.clone().not_equal_elem(EMPTY_VALUE).all_dim(D - 1);
        let truth_mask = truth.clone().not_equal_elem(EMPTY_VALUE).all_dim(D - 1);

        let predict = predict.mask_fill(predict_mask, 0.0);
        let truth = truth.mask_fill(truth_mask, 0.0);

        let mut pred_shape = predict.dims();
        pred_shape[D - 1] = truth.dims()[D - 2];

        let pred_zero = predict.zeros_like();
        let true_zero = truth.zeros_like();

        if predict
            .clone()
            .all_close(pred_zero, Some(EPSILON_MIN), Some(EPSILON_MIN))
            && truth
                .clone()
                .all_close(true_zero, Some(EPSILON_MIN), Some(EPSILON_MIN))
        {
            return Tensor::zeros(pred_shape, &predict.device());
        }

        let giou = self.giou(predict, truth);

        // Check if any of the values in the row/cols should be ignored, if so then clear them out

        giou
    }

    pub fn giou<const D: usize>(&self, predict: Tensor<B, D>, truth: Tensor<B, D>) -> Tensor<B, D>
    where
        [(); D + 1]:,
    {
        assert!(D >= 2, "This must be called on D >= 2, D = {D}");
        // let predict_dims = predict.dims();
        // let mut data = [0; D];
        // data[D - 1] = 4;
        // data[D - 2] = -1;
        // let predict = predict.repeat_dim(D - 1, predict_dims[D - 2]).reshape(data);
        // let [s1, e1, s2, e2]: [Tensor<B, { D - 1 }>; 4] = predict
        //     .chunk(4, D - 1)
        //     .into_iter()
        //     .map(|t| t.squeeze::<{ D - 1 }>(D - 1))
        //     .collect::<Vec<_>>()
        //     .try_into()
        //     .unwrap();

        // let truth = truth.repeat_dim(D - 2, predict_dims[D - 2]);
        // let [s1g, e1g, s2g, e2g]: [Tensor<B, { D - 1 }>; 4] = truth
        //     .chunk(4, D - 1)
        //     .into_iter()
        //     .map(|t| t.squeeze::<{ D - 1 }>(D - 1))
        //     .collect::<Vec<_>>()
        //     .try_into()
        //     .unwrap();

        // let out = (self.giou(s1, e1, s1g, e1g) + self.giou(s2, e2, s2g, e2g)).neg();

        // panic!("OUTPUT DIMENSIONS: {:?}", out.dims());

        let predict = predict.unsqueeze_dim::<{ D + 1 }>(D - 1);
        let truth = truth.unsqueeze_dim::<{ D + 1 }>(D - 2);

        let mut selection = [None; D + 1];
        selection[D] = Some((0, 1));
        let pred0 = predict.clone().slice(selection);
        let true0 = truth.clone().slice(selection);

        selection[D] = Some((1, 2));
        let pred1 = predict.clone().slice(selection);
        let true1 = truth.clone().slice(selection);

        selection[D] = Some((2, 3));
        let pred2 = predict.clone().slice(selection);
        let true2 = truth.clone().slice(selection);

        selection[D] = Some((3, 4));
        let pred3 = predict.clone().slice(selection);
        let true3 = truth.clone().slice(selection);

        let x1_inter = pred0.clone().max_pair(true0.clone());
        let x2_inter = pred1.clone().min_pair(true1.clone());
        let y1_inter = pred2.clone().max_pair(true2.clone());
        let y2_inter = pred3.clone().min_pair(true3.clone());

        let x1_enclose = pred0.clone().min_pair(true0.clone());
        let x2_enclose = pred1.clone().max_pair(true1.clone());
        let y1_enclose = pred2.clone().min_pair(true2.clone());
        let y2_enclose = pred3.clone().max_pair(true3.clone());

        let intersection_area =
            (x2_inter - x1_inter).clamp_min(0.0) * (y2_inter - y1_inter).clamp_min(0.0);

        let prediction_area = (pred1 - pred0) * (pred3 - pred2);
        let truth_area = (true1 - true0) * (true3 - true2);

        let union_area = prediction_area + truth_area - intersection_area.clone();
        let iou = intersection_area / (union_area.clone() + EPSILON_MIN);

        let enclose_area = (x2_enclose - x1_enclose) * (y2_enclose - y1_enclose);

        let giou = iou - (enclose_area.clone() - union_area) / (enclose_area + EPSILON_MIN);

        giou.squeeze::<D>(D)
    }

    // pub fn _forward<const D: usize>(
    //     &self,
    //     predict: &Tensor<B, D>,
    //     truth: &Tensor<B, D>,
    // ) -> Tensor<B, { D - 1 }> {
    //     let mut loss_arr: Vec<Tensor<B, 1>> = Vec::new();

    //     let mut shape: [usize; D] = predict.dims();
    //     // println!("Shape: {:?}", shape);

    //     if shape.len() != 3 {
    //         panic!("Only 3D tensors are supported");
    //     }

    //     for i in 0..shape[0] {
    //         // println!("\ndim0 idx: {}", i);

    //         let mut new_truth = truth
    //             .clone()
    //             .select(0, Tensor::<B, 1, Int>::from_ints([i], &truth.device()))
    //             .squeeze::<2>(0);
    //         let mut new_predict = predict
    //             .clone()
    //             .select(0, Tensor::<B, 1, Int>::from_ints([i], &truth.device()))
    //             .squeeze::<2>(0);

    //         // println!("New truth: {}", new_truth);
    //         // println!("New predict: {}", new_predict);

    //         let loss = self.loss_sum(&new_truth, &new_predict);
    //         // println!("Loss: {:?}", loss);

    //         loss_arr.push(loss);
    //     }
    //     // println!("Loss array: {:?}", loss_arr);

    //     return Tensor::stack(loss_arr, 0);
    // }

    // fn giou<const D: usize>(
    //     &self,
    //     t1: Tensor<B, D>,
    //     t2: Tensor<B, D>,
    //     p1: Tensor<B, D>,
    //     p2: Tensor<B, D>,
    // ) -> Tensor<B, D> {
    //     let intersection =
    //         (t2.clone().min_pair(p2.clone()) - t1.clone().max_pair(p1.clone())).clamp_min(0.0);
    //     let area = p2.clone() - p1.clone() + t2.clone() - t1.clone() - intersection.clone();
    //     let area = area.clamp_min(EPSILON_MIN);
    //     let entire = t2.max_pair(p2) - t1.min_pair(p1);

    //     let entire = entire / area.clone();

    //     // Overlap
    //     intersection / area - entire
    // }

    // fn _giou(&self, t1: f32, t2: f32, p1: f32, p2: f32) -> f32 {
    //     // println!("t1: {}, t2: {}, p1: {}, p2: {}", t1, t2, p1, p2);
    //     let intersection = (t2.min(p2) - t1.max(p1)).max(0.0);
    //     // println!("Intersection: {}", intersection);
    //     let area = p2 - p1 + t2 - t1 - intersection;
    //     // println!("Area: {}", area);
    //     let mut entire = p2.max(p2) - t1.min(p1);
    //     // println!("Entire: {}", entire);

    //     entire = entire / area; // this is where we would apply function f(C)

    //     let overlap = ((intersection / area) - (entire)).into();
    //     // println!("Overlap: {}", overlap);
    //     return overlap;
    // }

    // fn loss(&self, truth: &Tensor<B, 1>, predict: &Tensor<B, 1>) -> f64 {
    //     // Assign values
    //     let t_l1 = truth
    //         .clone()
    //         .select(0, Tensor::<B, 1, Int>::from_ints([0], &truth.device()))
    //         .into_scalar()
    //         .to_f32();
    //     let t_l2 = truth
    //         .clone()
    //         .select(0, Tensor::<B, 1, Int>::from_ints([1], &truth.device()))
    //         .into_scalar()
    //         .to_f32();
    //     let t_r1 = truth
    //         .clone()
    //         .select(0, Tensor::<B, 1, Int>::from_ints([2], &truth.device()))
    //         .into_scalar()
    //         .to_f32();
    //     let t_r2 = truth
    //         .clone()
    //         .select(0, Tensor::<B, 1, Int>::from_ints([3], &truth.device()))
    //         .into_scalar()
    //         .to_f32();

    //     let p_l1 = predict
    //         .clone()
    //         .select(0, Tensor::<B, 1, Int>::from_ints([0], &truth.device()))
    //         .into_scalar()
    //         .to_f32();
    //     let p_l2 = predict
    //         .clone()
    //         .select(0, Tensor::<B, 1, Int>::from_ints([1], &truth.device()))
    //         .into_scalar()
    //         .to_f32();
    //     let p_r1 = predict
    //         .clone()
    //         .select(0, Tensor::<B, 1, Int>::from_ints([2], &truth.device()))
    //         .into_scalar()
    //         .to_f32();
    //     let p_r2 = predict
    //         .clone()
    //         .select(0, Tensor::<B, 1, Int>::from_ints([3], &truth.device()))
    //         .into_scalar()
    //         .to_f32();

    //     // Calculate each loss
    //     let l_giou = self._giou(t_l1, t_l2, p_l1, p_l2);
    //     let r_giou = self._giou(t_r1, t_r2, p_r1, p_r2);

    //     let loss = -1.0 * (l_giou + r_giou); // this is where we would apply function g
    //                                          // println!("loss between truth: {} and prediction {} is {}", truth, predict, loss);
    //     return loss.into();
    // }

    // fn loss_sum(&self, truths: &Tensor<B, 2>, predicts: &Tensor<B, 2>) -> Tensor<B, 1> {
    //     // let mut sum = 0.0;
    //     let len = predicts.dims()[0];
    //     let shape = predicts.dims();

    //     // the results loss tensor will be of equal length to the number of predictions
    //     // for predictions made after all truths are exausted, a loss of n will be applied with n>>0
    //     let max_loss = 100.0;

    //     // let mut loss_floats = [0.0; 2];
    //     let mut loss_floats = vec![0.0; len];
    //     // let loss = Tensor::<Backend, 1, Float>::from_floats([0.0], &truths.device());

    //     // println!("Length of predict matrix (num of truths): {}", len);

    //     let mut used = <Vec<usize>>::new();
    //     // let mut used = <Vec<i32>>::new();

    //     for i in 0..len {
    //         // println!("Currently used is {:?} and i is {}", used, i);
    //         if used.len() == truths.dims()[0] {
    //             // println!("All truths have been used");
    //             loss_floats[i] = max_loss;
    //             break;
    //         }

    //         // let mut max: f64 = loss(&truths.select(0, Tensor::<Backend, 1>::from_floats()), &predicts.select(0, i));
    //         let idx_0 = Tensor::<B, 1, Int>::from_ints([0], &truths.device());
    //         let idx_i = Tensor::<B, 1, Int>::from_ints([i], &truths.device());
    //         let mut max: f64 = self.loss(
    //             &truths.clone().select(0, idx_0.clone()).flatten::<1>(0, 1),
    //             &predicts.clone().select(0, idx_i.clone()).flatten::<1>(0, 1),
    //         );

    //         let mut max_truth_idx = 0;
    //         // println!("\nSearching through truths for better loss than loss: {}", max);
    //         for j in 0..truths.dims()[0] {
    //             if !used.contains(&j) {
    //                 let idx_j = Tensor::<B, 1, Int>::from_ints([j], &truths.device());
    //                 let loss = self.loss(
    //                     &truths.clone().select(0, idx_j.clone()).flatten::<1>(0, 1),
    //                     &predicts.clone().select(0, idx_i.clone()).flatten::<1>(0, 1),
    //                 );
    //                 // println!("Truth: {:?}, Predict: {:?}, Loss: {}", truths.clone().select(0, idx_j.clone()), predicts.clone().select(0, idx_i.clone()), loss);
    //                 if loss < max {
    //                     // println!("Found best match with loss: {}", loss);
    //                     max = loss;
    //                     max_truth_idx = j;
    //                 }
    //             }
    //             loss_floats[i] = max;
    //         }

    //         // used.push(max_truth_idx);

    //         // sum += max;
    //     }
    //     // println!("loss_floats: {:?}", loss_floats);
    //     // return loss_floats
    //     return Tensor::<B, 1>::from_floats(loss_floats.as_slice(), &truths.device());
    //     // return sum / len as f64;
    // }
}

#[derive(Debug, Clone)]
pub struct ObjectnessOutput<B: Backend> {
    pub output: Tensor<B, 2>,
    pub targets: Tensor<B, 2, Int>,
}

#[derive(Debug, Clone)]
pub struct BatchedRegressionOutput<B: Backend> {
    pub output: Tensor<B, 3>,
    pub targets: Tensor<B, 3>,
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
