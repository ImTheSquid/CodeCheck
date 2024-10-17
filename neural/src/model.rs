use burn::{
    config::{self, Config},
    module::Module,
    nn::{
        attention::{MhaInput, MultiHeadAttention, MultiHeadAttentionConfig},
        loss::{BinaryCrossEntropyLoss, BinaryCrossEntropyLossConfig},
        Linear, LinearConfig,
    },
    prelude::Backend,
    tensor::{backend::AutodiffBackend, Int, Tensor},
    train::{RegressionOutput, TrainOutput, TrainStep, ValidStep},
};

use crate::{
    data::{AstBatch, MAX_NODES, MAX_SPANS},
    gat::{Gat, GatConfig},
    loss::{self, BatchedRegressionOutput, ModelOutput, ObjectnessOutput},
    node_process::{NodeProcessor, NodeProcessorConfig},
};

#[derive(Config)]
pub struct ModelConfig {
    #[config(default = "crate::data::MAX_FEATURES")]
    pub hidden_1_size: usize,
    #[config(default = 0.01)]
    pub leaky_1_slope: f64,
    #[config(default = 100)]
    pub hidden_2_size: usize,
    #[config(default = 0.01)]
    pub leaky_2_slope: f64,
    pub gat: GatConfig,
    #[config(default = 8)]
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
            bce_loss: BinaryCrossEntropyLossConfig::new().init(device),
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
    bce_loss: BinaryCrossEntropyLoss<B>,
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
}

impl<B: AutodiffBackend> TrainStep<AstBatch<B>, ModelOutput<B>> for Model<B> {
    fn step(&self, item: AstBatch<B>) -> burn::train::TrainOutput<ModelOutput<B>> {
        let out = self.forward(item.features, item.edges);
        let regression_loss: Tensor<B, 1> = loss::GIOULoss::default()
            .forward(&out.regression, &item.spans)
            .mean();
        let objectness_spans = item
            .spans
            .clone()
            .sum_dim(2)
            .squeeze_dims::<2>(&[])
            .bool()
            .int();
        let objectness_loss = self
            .bce_loss
            .forward(out.objectness.clone(), objectness_spans.clone());
        TrainOutput::new(
            self,
            (regression_loss.clone() * 5.0 + objectness_loss.clone()).backward(),
            loss::ModelOutput {
                loss: regression_loss * 5.0 + objectness_loss,
                regression: BatchedRegressionOutput {
                    output: out.regression,
                    targets: item.spans,
                },
                objectness: ObjectnessOutput {
                    output: out.objectness,
                    targets: objectness_spans,
                },
            },
        )
    }
}

impl<B: Backend> ValidStep<AstBatch<B>, ModelResult<B>> for Model<B> {
    fn step(&self, item: AstBatch<B>) -> ModelResult<B> {
        self.forward(item.features, item.edges)
    }
}
