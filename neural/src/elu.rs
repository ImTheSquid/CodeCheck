use burn::{config::Config, module::Module, prelude::Backend, tensor::Tensor};

#[derive(Debug, Config)]
pub struct EluConfig {
    #[config(default = "0.01")]
    pub negative_slope: f64,
}

impl EluConfig {
    pub fn init(&self) -> Elu {
        Elu {
            negative_slope: self.negative_slope,
        }
    }
}

#[derive(Debug, Module, Clone, Copy)]
pub struct Elu {
    pub negative_slope: f64,
}

impl Elu {
    pub fn forward<B: Backend, const D: usize>(&self, input: Tensor<B, D>) -> Tensor<B, D> {
        let masked_high = input.clone().clamp_min(0.0);
        let masked_low = input.clamp_max(0.0);
        masked_high.add(
            masked_low
                .exp()
                .add_scalar(1.0)
                .mul_scalar(self.negative_slope),
        )
    }
}
