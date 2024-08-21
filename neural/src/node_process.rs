use burn::{
    config::Config,
    module::Module,
    nn::{Linear, LinearConfig, Relu},
    prelude::Backend,
    tensor::Tensor,
};

// A simple preprocessor to get some very basic embeddings out of tree info
#[derive(Debug, Module)]
struct NodeProcessor<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    linear3: Linear<B>,
    activation: Relu,
}

impl<B: Backend> NodeProcessor<B> {
    fn forward(&self, x: Tensor<B, 1>) -> Tensor<B, 1> {
        let x = self.linear1.forward(x);
        let x = self.linear2.forward(x);
        let x = self.linear3.forward(x);
        self.activation.forward(x)
    }
}

#[derive(Debug, Config)]
struct NodeProcessorConfig {
    hidden_1_size: usize,
    hidden_2_size: usize,
    output_size: usize,
}

const MAX_ONE_HOT_TREE_ENCODING_LEN: usize = 5_000;

impl NodeProcessorConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> NodeProcessor<B> {
        NodeProcessor {
            // Add one for language identifier
            linear1: LinearConfig::new(MAX_ONE_HOT_TREE_ENCODING_LEN + 1, self.hidden_1_size)
                .init(device),
            linear2: LinearConfig::new(self.hidden_1_size, self.hidden_2_size).init(device),
            linear3: LinearConfig::new(self.hidden_2_size, self.output_size).init(device),
            activation: Relu::new(),
        }
    }
}
