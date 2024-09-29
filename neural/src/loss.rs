use burn::{
    prelude::Backend,
    tensor::{Int, Tensor},
    train::metric::{Adaptor, LossInput},
};

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
