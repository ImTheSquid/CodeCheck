use burn::{
    config::Config,
    module::Module,
    nn::{LeakyRelu, LeakyReluConfig, Linear, LinearConfig, Relu},
    prelude::Backend,
    tensor::Tensor,
};

use crate::{
    data::MAX_FEATURES,
    sequential::{Sequential, SequentialConfig, SequentialLayerConfig},
};

// A simple preprocessor to get some very basic embeddings out of tree info
#[derive(Debug, Module)]
pub struct NodeProcessor<B: Backend> {
    seq: Sequential<B>,
}

impl<B: Backend> NodeProcessor<B> {
    pub fn forward<const D: usize>(&self, x: Tensor<B, D>) -> Tensor<B, D> {
        self.seq.forward(x)
    }
}

#[derive(Debug, Config)]
pub struct NodeProcessorConfig {
    pub hidden_1_size: usize,
    pub leaky_1_slope: f64,
    pub hidden_2_size: usize,
    pub leaky_2_slope: f64,
    pub output_size: usize,
}

impl NodeProcessorConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> NodeProcessor<B> {
        NodeProcessor {
            seq: SequentialConfig::new(vec![
                SequentialLayerConfig::Linear(LinearConfig::new(MAX_FEATURES, self.hidden_1_size)),
                SequentialLayerConfig::LeakyRelu(
                    LeakyReluConfig::new().with_negative_slope(self.leaky_1_slope),
                ),
                SequentialLayerConfig::Linear(LinearConfig::new(
                    self.hidden_1_size,
                    self.hidden_2_size,
                )),
                SequentialLayerConfig::LeakyRelu(
                    LeakyReluConfig::new().with_negative_slope(self.leaky_2_slope),
                ),
                SequentialLayerConfig::Linear(LinearConfig::new(
                    self.hidden_2_size,
                    self.output_size,
                )),
                SequentialLayerConfig::Relu,
            ])
            .init(device),
        }
    }
}
