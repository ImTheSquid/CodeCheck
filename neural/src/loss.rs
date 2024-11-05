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
    ) -> Tensor<B, 1>
    where
        [(); D + 1]:,
    {
        let empty_truth_mask = truth.clone().equal_elem(EMPTY_VALUE).all_dim(D - 1);

        let giou = self.masked_giou(predict, truth);
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

    /// Ignores -1's
    pub fn masked_giou<const D: usize>(
        &self,
        predict: Tensor<B, D>,
        truth: Tensor<B, D>,
    ) -> Tensor<B, D>
    where
        [(); D + 1]:,
    {
        let pred_ones = predict.ones_like();
        let true_ones = truth.ones_like();
        let predict_mask = predict
            .clone()
            .is_close(pred_ones * EMPTY_VALUE, None, None)
            .all_dim(D - 1);
        let truth_mask = truth
            .clone()
            .is_close(true_ones * EMPTY_VALUE, None, None)
            .all_dim(D - 1);
        let predict = predict.mask_fill(predict_mask, f64::NAN);
        let truth = truth.mask_fill(truth_mask, f64::NAN);

        let mut pred_shape = predict.dims();
        pred_shape[D - 1] = truth.dims()[D - 2];

        let pred_zero = predict.zeros_like();
        let true_zero = truth.zeros_like();

        if predict.clone().all_close(pred_zero, None, None)
            && truth.clone().all_close(true_zero, None, None)
        {
            return Tensor::zeros(pred_shape, &predict.device());
        }

        let giou = self.giou(predict, truth);
        // println!("UNFILTERED GIOU: {giou}");

        // Check if any of the values in the row/cols should be ignored, if so then clear them out
        // is_nan is currently broken but this works instead
        let nans = giou
            .clone()
            .greater_equal_elem(f64::NEG_INFINITY)
            .bool_not();
        giou.clone().mask_fill(nans, 0.0)
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
        // println!("INPUTS:\nP\n{predict}\nT\n{truth}======================");

        let predict = predict.unsqueeze_dim::<{ D + 1 }>(D - 1);
        let truth = truth.unsqueeze_dim::<{ D + 1 }>(D - 2);

        let mut selection = [None; D + 1];
        selection[D] = Some((0, 1));
        let pred0 = predict.clone().slice(selection);
        let true0 = truth.clone().slice(selection);
        // println!("P {pred0} T {true0}");

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
        // println!("=================\nX1:{x1_inter}\nX2:{x2_inter}\nY1:{y1_inter}\nY2:{y2_inter}\n=====================");

        let x1_enclose = pred0.clone().min_pair(true0.clone());
        let x2_enclose = pred1.clone().max_pair(true1.clone());
        let y1_enclose = pred2.clone().min_pair(true2.clone());
        let y2_enclose = pred3.clone().max_pair(true3.clone());

        // println!(
        //     "===================\nDIFF X {} Y {}",
        //     (x2_inter.clone() - x1_inter.clone()),
        //     (y2_inter.clone() - y1_inter.clone())
        // );
        let intersection_area =
            (x2_inter - x1_inter).clamp_min(0.0) * (y2_inter - y1_inter).clamp_min(0.0);
        // println!("==============\nINTERSECT\n:{intersection_area}\n");

        let prediction_area = (pred1 - pred0) * (pred3 - pred2);
        let truth_area = (true1 - true0) * (true3 - true2);

        let union_area = prediction_area + truth_area - intersection_area.clone();
        // println!("==============\nUNION\n:{union_area}\n");
        let iou = intersection_area / (union_area.clone() + EPSILON_MIN);
        // println!("==============\nIOU\n:{iou}\n");

        let enclose_area = (x2_enclose - x1_enclose) * (y2_enclose - y1_enclose);
        // println!("==============\nENCLOSE\n:{enclose_area}\n");

        let giou = iou - (enclose_area.clone() - union_area) / (enclose_area + EPSILON_MIN);

        giou.squeeze::<D>(D)
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
        LossInput::new(self.loss.clone())
    }
}

#[cfg(test)]
mod tests {
    use burn::{
        backend::{wgpu::WgpuDevice, Wgpu},
        tensor::Tensor,
    };

    use crate::loss::GIOULoss;

    #[test]
    fn loss_function_works() {
        let device = WgpuDevice::default();
        type Backend = Wgpu;

        let predict = Tensor::<Backend, 2>::from_floats(
            [
                [0.5, 1.0, 0.5, 1.0],
                [-1.0, -1.0, -1.0, -1.0],
                [2.5, 3.0, 2.5, 3.0],
            ],
            &device,
        );

        let truth = Tensor::<Backend, 2>::from_floats(
            [
                [0.5, 1.0, 0.5, 1.0],
                [2.0, 2.5, 2.0, 2.5],
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
