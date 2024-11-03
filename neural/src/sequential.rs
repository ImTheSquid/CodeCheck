use burn::{
    config::Config,
    module::Module,
    nn::{LeakyRelu, LeakyReluConfig, Linear, LinearConfig, Relu, Sigmoid},
    prelude::Backend,
    tensor::Tensor,
};

#[derive(Debug, Config)]
pub enum SequentialLayerConfig {
    Linear(LinearConfig),
    Relu,
    LeakyRelu(LeakyReluConfig),
    Sigmoid,
}

#[derive(Debug, Config)]
pub struct SequentialConfig {
    pub layers: Vec<SequentialLayerConfig>,
}

impl SequentialConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Sequential<B> {
        Sequential {
            layers: self
                .layers
                .iter()
                .map(|l| match l {
                    SequentialLayerConfig::Relu => SequentialLayer::Relu(Relu),
                    SequentialLayerConfig::Sigmoid => SequentialLayer::Sigmoid(Sigmoid),
                    SequentialLayerConfig::LeakyRelu(lrc) => SequentialLayer::LeakyRelu(lrc.init()),
                    SequentialLayerConfig::Linear(l) => SequentialLayer::Linear(l.init(device)),
                })
                .collect(),
        }
    }
}

#[derive(Module, Debug)]
pub enum SequentialLayer<B: Backend> {
    Linear(Linear<B>),
    Relu(Relu),
    LeakyRelu(LeakyRelu),
    Sigmoid(Sigmoid),
}

#[derive(Module, Debug)]
pub struct Sequential<B: Backend> {
    layers: Vec<SequentialLayer<B>>,
}

impl<B: Backend> Sequential<B> {
    pub fn forward<const D: usize>(&self, mut input: Tensor<B, D>) -> Tensor<B, D> {
        for layer in &self.layers {
            input = match layer {
                SequentialLayer::Relu(r) => r.forward(input),
                SequentialLayer::Sigmoid(s) => s.forward(input),
                SequentialLayer::LeakyRelu(lr) => lr.forward(input),
                SequentialLayer::Linear(l) => l.forward(input),
            }
        }

        input
    }
}
