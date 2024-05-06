use std::marker::PhantomData;

use burn::{config::Config, tensor::backend::Backend};

#[derive(Debug, Config)]
pub struct ContrastiveLossConfig {
    #[config(default = 0.5)]
    alpha: f32,
    #[config(default = 0.5)]
    beta: f32,
    #[config(default = 1.0)]
    margin: f32,
}

impl ContrastiveLossConfig {
    pub fn init<B: Backend>(&self) -> ContrastiveLoss<B> {
        ContrastiveLoss {
            backend: PhantomData,
            alpha: self.alpha,
            beta: self.beta,
            margin: self.margin,
        }
    }
}

pub struct ContrastiveLoss<B: Backend> {
    backend: PhantomData<B>,
    alpha: f32,
    beta: f32,
    margin: f32,
}

impl<B: Backend> ContrastiveLoss<B> {
    pub fn forward(&self, distance: f32, indicator: f32) -> f32 {
        debug_assert!((0.0..=1.0).contains(&indicator), "Indicator out of bounds!");

        self.alpha * (1.0 - indicator) * distance.powi(2)
            + self.beta * indicator * 0.0_f32.max(self.margin - distance).powi(2)
    }
}
