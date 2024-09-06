use burn::{
    config::Config,
    module::Module,
    nn::{LeakyRelu, LeakyReluConfig, Linear, LinearConfig, Relu},
    prelude::Backend,
    tensor::Tensor,
};

// A simple preprocessor to get some very basic embeddings out of tree info
#[derive(Debug, Module)]
pub struct NodeProcessor<B: Backend> {
    linear1: Linear<B>,
    // Leaky to allow for negatives but to an extent
    activation1: LeakyRelu,
    linear2: Linear<B>,
    activation2: LeakyRelu,
    linear3: Linear<B>,
    activation: Relu,
}

impl<B: Backend> NodeProcessor<B> {
    pub fn forward(&self, x: Tensor<B, 1>) -> Tensor<B, 1> {
        let x = self.linear1.forward(x);
        let x = self.linear2.forward(self.activation1.forward(x));
        let x = self.linear3.forward(self.activation2.forward(x));
        self.activation.forward(x)
    }
}

#[derive(Debug, Config)]
pub struct NodeProcessorConfig {
    hidden_1_size: usize,
    leaky_1_slope: f64,
    hidden_2_size: usize,
    leaky_2_slope: f64,
    output_size: usize,
}

const MAX_ONE_HOT_TREE_ENCODING_LEN: usize = 5_000;

impl NodeProcessorConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> NodeProcessor<B> {
        NodeProcessor {
            // Add one for language identifier
            linear1: LinearConfig::new(MAX_ONE_HOT_TREE_ENCODING_LEN + 1, self.hidden_1_size)
                .init(device),
            activation1: LeakyReluConfig::new()
                .with_negative_slope(self.leaky_1_slope)
                .init(),
            linear2: LinearConfig::new(self.hidden_1_size, self.hidden_2_size).init(device),
            activation2: LeakyReluConfig::new()
                .with_negative_slope(self.leaky_2_slope)
                .init(),
            linear3: LinearConfig::new(self.hidden_2_size, self.output_size).init(device),
            activation: Relu::new(),
        }
    }
}
