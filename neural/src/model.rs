use burn::{
    config::Config,
    module::Module,
    nn::{
        attention::{MhaInput, MultiHeadAttention, MultiHeadAttentionConfig},
        Linear, LinearConfig,
    },
    prelude::Backend,
    tensor::{Int, Tensor},
    train::RegressionOutput,
};

use crate::{
    data::{MAX_NODES, MAX_SPANS},
    gat::{Gat, GatConfig},
    node_process::{NodeProcessor, NodeProcessorConfig},
};

#[derive(Config)]
pub struct ModelConfig {
    pub hidden_1_size: usize,
    pub leaky_1_slope: f64,
    pub hidden_2_size: usize,
    pub leaky_2_slope: f64,
    pub gat: GatConfig,
    pub attention_heads: usize,
}

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        let gat_output = self.gat.num_features.last().unwrap() * self.gat.num_heads.last().unwrap();
        Model {
            node_processor: NodeProcessorConfig::new(
                self.hidden_1_size,
                self.leaky_1_slope,
                self.hidden_2_size,
                self.leaky_2_slope,
                self.gat.num_features[0] / 2,
            )
            .init(device),
            gat: self.gat.init(device),
            attention: MultiHeadAttentionConfig::new(gat_output, self.attention_heads).init(device),
            regression: LinearConfig::new(gat_output, MAX_SPANS * 4).init(device),
            objectness: LinearConfig::new(gat_output, MAX_SPANS).init(device),
        }
    }
}

#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    node_processor: NodeProcessor<B>,
    gat: Gat<B>,
    attention: MultiHeadAttention<B>,
    regression: Linear<B>,
    objectness: Linear<B>,
}

pub struct ModelResult<B: Backend> {
    pub regression: Tensor<B, 3>,
    pub objectness: Tensor<B, 2>,
}

impl<B: Backend> Model<B> {
    pub fn forward(&self, features: Tensor<B, 3>, edges: Tensor<B, 3, Int>) -> ModelResult<B> {
        let features = self.node_processor.forward(features);

        let features = self.gat.forward(edges, features);

        let features_a = features
            .clone()
            .slice([None, Some((0, MAX_NODES as i64)), None]);

        let features_b = features.slice([None, Some((MAX_NODES as i64, -1)), None]);

        let feature_attention =
            self.attention
                .forward(MhaInput::new(features_a, features_b.clone(), features_b));

        let context = feature_attention.context.flatten::<2>(1, 2);

        let regression =
            self.regression
                .forward(context.clone())
                .reshape([-1, MAX_SPANS as i32, 4]);

        let objectness = self.objectness.forward(context);

        ModelResult {
            regression,
            objectness,
        }
    }

    pub fn forward_regression_objectness(&self) -> RegressionOutput<B> {
        todo!()
    }
}
