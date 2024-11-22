use burn::{
    config::Config,
    module::Module,
    nn::{Dropout, DropoutConfig, LeakyRelu, LeakyReluConfig, Linear, LinearConfig, Relu, Sigmoid},
    prelude::Backend,
    tensor::Tensor,
};

#[derive(Debug, Config)]
pub enum SequentialLayerConfig {
    Linear(LinearConfig),
    Relu,
    LeakyRelu(LeakyReluConfig),
    Sigmoid,
    Dropout(DropoutConfig),
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
                    SequentialLayerConfig::Dropout(d) => SequentialLayer::Dropout(d.init()),
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
    Dropout(Dropout),
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
                SequentialLayer::Linear(l) => {
                    println!("LIN WEIGHTS: {}", l.weight.val());
                    l.forward(input)
                }
                SequentialLayer::Dropout(d) => d.forward(input),
            };
            println!(
                "AFTER {layer:?}, INPUT NOW: {input} SUM {}",
                input.clone().sum()
            );
        }

        input
    }
}
