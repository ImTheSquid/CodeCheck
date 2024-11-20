use core::f64;
use std::marker::PhantomData;

use burn::{
    prelude::Backend,
    tensor::{Int, Tensor},
    train::metric::{Adaptor, LossInput},
};

#[derive(Debug, Default)]
pub struct GIOULoss<B: Backend> {
    _backend: PhantomData<B>,
}

/// A minimum value to ensure area is never 0 (leading to a NaN)
pub const EPSILON_MIN: f64 = 1e-7;
const EMPTY_VALUE: f64 = -1.0;
const THRESHOLD: f64 = 0.3;
const PENALTY: f64 = 1.0;

// added this impl - basically all the same functions as main.rs, but changed all calls to be self.function_name
// I dont really have a way to test it
// TODO - clean print statements, remove unnecessary code, document code bett
impl<B: Backend> GIOULoss<B> {
    pub fn giou<const D: usize>(&self, predict: Tensor<B, D>, truth: Tensor<B, D>) -> Tensor<B, D>
    where
        [(); D + 1]:,
    {
        assert!(D >= 2, "This must be called on D >= 2, D = {D}");

        let predict = predict.unsqueeze_dim::<{ D + 1 }>(D - 1);
        let truth = truth.unsqueeze_dim::<{ D + 1 }>(D - 2);
        // println!("PRED: {predict}\nTRUE: {truth}");

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
        // println!("P {pred1} T {true1}");

        selection[D] = Some((3, 4));
        let pred3 = predict.clone().slice(selection);
        let true3 = truth.clone().slice(selection);

        let x1_inter = pred0.clone().max_pair(true0.clone());
        let y1_inter = pred1.clone().max_pair(true1.clone());
        let x2_inter = pred2.clone().min_pair(true2.clone());
        let y2_inter = pred3.clone().min_pair(true3.clone());
        // println!("=================\nX1:{x1_inter}\nY1:{y1_inter}\nX2:{x2_inter}\nY2:{y2_inter}\n=====================");

        let x1_enclose = pred0.clone().min_pair(true0.clone());
        let y1_enclose = pred1.clone().min_pair(true1.clone());
        let x2_enclose = pred2.clone().max_pair(true2.clone());
        let y2_enclose = pred3.clone().max_pair(true3.clone());

        // println!(
        //     "===================\nDIFF X {} Y {}",
        //     (x2_inter.clone() - x1_inter.clone()),
        //     (y2_inter.clone() - y1_inter.clone())
        // );
        let intersection_area =
            (x2_inter - x1_inter).clamp_min(0.0) * (y2_inter - y1_inter).clamp_min(0.0);
        // println!("==============\nINTERSECT\n:{intersection_area}\n");

        let prediction_area = (pred2 - pred0) * (pred3 - pred1);
        let pred_zero = prediction_area.zeros_like();
        let pred_zero = prediction_area.clone().is_close(pred_zero, None, None);
        let truth_area = (true2 - true0) * (true3 - true1);
        let true_zero = truth_area.zeros_like();
        let true_zero = truth_area.clone().is_close(true_zero, None, None);
        let zero = (pred_zero.int() + true_zero.int()).greater_elem(0);
        // println!("============\nPRED: {prediction_area}\n TRUE: {truth_area}");

        let union_area = prediction_area + truth_area - intersection_area.clone();
        // println!("==============\nUNION\n:{union_area}\n");
        let iou = intersection_area / union_area.clone().clamp_min(EPSILON_MIN);
        // println!("==============\nIOU\n:{iou}\n");

        let enclose_area = (x2_enclose - x1_enclose) * (y2_enclose - y1_enclose);
        // println!("==============\nENCLOSE\n:{enclose_area}\n");

        let giou = iou - (enclose_area.clone() - union_area) / enclose_area.clamp_min(EPSILON_MIN);

        let giou = giou.mask_fill(zero, 0.0);

        giou.squeeze::<D>(D)
    }

    pub fn forward<const D: usize>(
        &self,
        predict: Tensor<B, D>,
        truth: Tensor<B, D>,
    ) -> Tensor<B, 1>
    where
        [(); D + 1]:,
    {
        let empty_truth_mask = truth.clone().equal_elem(EMPTY_VALUE).all_dim(D - 1);

        let giou = self.giou(predict, truth);
        // println!("GIOU: {giou}");

        let best = giou.max_dim(D - 2);

        // Empty truth by default has no loss, so make it such that 1 - 1 = 0
        let actual_loss = best.ones_like() - best.clone().mask_fill(empty_truth_mask.clone(), 1.0);

        let penalty = best
            .lower_elem(THRESHOLD)
            .float()
            .mask_fill(empty_truth_mask, 0.0)
            * PENALTY;

        // println!("PENALTY: {}", penalty.clone().sum().into_scalar());
        actual_loss.sum() + penalty.sum()
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
        println!("FINAL LOSS: {}", self.loss);
        LossInput::new(self.loss.clone())
    }
}

#[cfg(test)]
mod tests {
    use burn::{backend::wgpu::WgpuDevice, tensor::Tensor};

    use crate::loss::GIOULoss;

    #[test]
    fn loss_function_works() {
        let device = WgpuDevice::default();
        type Backend = burn::backend::Wgpu;

        let predict = Tensor::<Backend, 2>::from_floats(
            [
                [0.5, 0.5, 1.0, 1.0],
                [-1.0, -1.0, -1.0, -1.0],
                [2.5, 2.5, 3.0, 3.0],
            ],
            &device,
        );

        let truth = Tensor::<Backend, 2>::from_floats(
            [
                [0.5, 0.5, 1.0, 1.0],
                [2.0, 2.0, 2.5, 2.5],
                [-1.0, -1.0, -1.0, -1.0],
            ],
            &device,
        );

        let test = Tensor::<Backend, 1>::from_floats([4.0], &device);

        let output = GIOULoss::default().forward(predict, truth);

        assert_eq!(
            test.dims(),
            output.dims(),
            "Tensors have different dimensions!"
        );

        assert!(
            test.clone().all_close(output.clone(), None, None),
            "Test and output diverge!\nTest: {}\nOutput: {}",
            test.into_scalar(),
            output.into_scalar()
        );
    }
}
