use burn::{
    config::Config,
    module::Module,
    nn::{
        attention::{MhaInput, MultiHeadAttention, MultiHeadAttentionConfig},
        loss::{BinaryCrossEntropyLoss, BinaryCrossEntropyLossConfig},
        LinearConfig,
    },
    prelude::Backend,
    tensor::{backend::AutodiffBackend, cast::ToElement, Int, Tensor},
    train::{TrainOutput, TrainStep, ValidStep},
};

use crate::{
    data::{AstBatch, MAX_NODES, MAX_SPANS},
    gat::{Gat, GatConfig},
    leaky_gain,
    loss::{self, BatchedRegressionOutput, ModelOutput, ObjectnessOutput},
    node_process::{NodeProcessor, NodeProcessorConfig},
    sequential::{Sequential, SequentialConfig, SequentialLayerConfig},
};

#[derive(Config)]
pub struct ModelConfig {
    #[config(default = "crate::data::MAX_FEATURES")]
    pub hidden_1_size: usize,
    #[config(default = 0.01)]
    pub leaky_1_slope: f64,
    #[config(default = 0.3)]
    pub p_dropout: f64,
    #[config(default = 50)]
    pub hidden_2_size: usize,
    #[config(default = 0.01)]
    pub leaky_2_slope: f64,
    pub gat: GatConfig,
    #[config(default = 8)]
    pub attention_heads: usize,
}

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        let gat_output = *self.gat.num_features.last().unwrap();
        Model {
            node_processor: NodeProcessorConfig::new(
                self.hidden_1_size,
                self.leaky_1_slope,
                self.p_dropout,
                self.hidden_2_size,
                self.leaky_2_slope,
                self.gat.num_features[0],
            )
            .init(device),
            gat: self.gat.init(device),
            attention: MultiHeadAttentionConfig::new(gat_output, self.attention_heads).init(device),
            regression: SequentialConfig::new(vec![
                SequentialLayerConfig::Linear(
                    LinearConfig::new(MAX_NODES * gat_output, MAX_NODES / 2).with_initializer(
                        burn::nn::Initializer::KaimingNormal {
                            gain: leaky_gain(0.0),
                            fan_out_only: false,
                        },
                    ),
                ),
                SequentialLayerConfig::Relu,
                SequentialLayerConfig::Linear(
                    LinearConfig::new(MAX_NODES / 2, MAX_SPANS * 4).with_initializer(
                        burn::nn::Initializer::KaimingNormal {
                            gain: leaky_gain(0.0),
                            fan_out_only: false,
                        },
                    ),
                ),
                SequentialLayerConfig::Relu,
            ])
            .init(device),
            objectness: SequentialConfig::new(vec![
                SequentialLayerConfig::Linear(
                    LinearConfig::new(MAX_NODES * gat_output, MAX_NODES / 2).with_initializer(
                        burn::nn::Initializer::KaimingNormal {
                            gain: leaky_gain(0.0),
                            fan_out_only: false,
                        },
                    ),
                ),
                SequentialLayerConfig::Relu,
                SequentialLayerConfig::Linear(LinearConfig::new(MAX_NODES / 2, MAX_SPANS)),
            ])
            .init(device),
            bce_loss: BinaryCrossEntropyLossConfig::new()
                .with_logits(true)
                .init(device),
        }
    }
}

#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    node_processor: NodeProcessor<B>,
    gat: Gat<B>,
    attention: MultiHeadAttention<B>,
    regression: Sequential<B>,
    objectness: Sequential<B>,
    bce_loss: BinaryCrossEntropyLoss<B>,
}

pub struct ModelResult<B: Backend> {
    pub regression: Tensor<B, 3>,
    pub objectness: Tensor<B, 2>,
}

impl<B: Backend> Model<B> {
    pub fn forward(
        &self,
        features: Tensor<B, 2>,
        edges: Tensor<B, 2, Int>,
        graph_feature_indices: Tensor<B, 1, Int>,
    ) -> ModelResult<B> {
        // println!(
        //     "MODEL FORWARD: F {:?} E {:?}",
        //     features.dims(),
        //     edges.dims()
        // );
        println!("FORWARD: {features}\nE: {edges}\n");
        let features = self.node_processor.forward(features);

        println!("PROC COMPLETE: {features}\n");

        let features = self.gat.forward(edges, features);

        println!("GAT COMPLETE: {features}\n");

        println!("ABSOLUTE MAX: {}", features.clone().abs().max());

        // println!("GAT COMPLETE");

        let num_pairs = (graph_feature_indices.clone().max().into_scalar().to_i64() + 1) / 2;

        let mut expanded = Tensor::<B, 3>::zeros(
            [num_pairs as usize, MAX_NODES, features.dims()[1]],
            &features.device(),
        );
        for pair_index in 0..num_pairs {
            let first = pair_index * 2;
            let second = first + 1;

            // What elements match?
            let first_indices = graph_feature_indices
                .clone()
                .equal_elem(first)
                .nonzero()
                .into_iter()
                .next()
                .unwrap();
            let second_indices = graph_feature_indices
                .clone()
                .equal_elem(second)
                .nonzero()
                .into_iter()
                .next()
                .unwrap();
            let first_select = features.clone().select(0, first_indices);
            let second_select = features.clone().select(0, second_indices).unsqueeze();

            let feature_attention = self.attention.forward(MhaInput::new(
                first_select.unsqueeze(),
                second_select.clone(),
                second_select,
            ));

            let ctx = feature_attention.context.squeeze::<2>(0);

            expanded.inplace(|e| {
                let pair_index = pair_index as usize;
                e.slice_assign(
                    [
                        pair_index..pair_index + 1,
                        0..ctx.dims()[0],
                        0..ctx.dims()[1],
                    ],
                    ctx.unsqueeze(),
                )
            });
        }

        // let features_a = features
        //     .clone()
        //     .slice([None, Some((0, max_nodes as i64)), None]);

        // let features_b = features.slice([None, Some((max_nodes as i64, -1)), None]);

        // let feature_attention =
        //     self.attention
        //         .forward(MhaInput::new(features_a, features_b.clone(), features_b));

        // println!("ATTN COMPLETE: {}", feature_attention.context);

        // Normalize length of output to put through linear
        // let mut shape = feature_attention.context.dims();
        // let original = shape[1];
        // shape[1] = MAX_NODES;
        // let dev = feature_attention.context.device();
        // let z = Tensor::<B, 3>::zeros(shape, &dev);
        // let context = z.slice_assign(
        //     [0..shape[0], 0..original, 0..shape[2]],
        //     feature_attention.context,
        // );

        let context = expanded.flatten::<2>(1, 2);

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
        println!(
            "INPUT===================\nF: {}\nE: {}",
            item.features, item.edges
        );
        let out = self.forward(item.features, item.edges, item.graph_feature_indices);
        println!(
            "OUTPUT=================\nOBJ: {}\n\nREG: {}",
            out.objectness, out.regression
        );
        let regression_loss: Tensor<B, 1> =
            loss::GIOULoss::default().forward(out.regression.clone(), item.spans.clone());
        let objectness_spans = item
            .spans
            .clone()
            .sum_dim(2)
            .squeeze_dims::<2>(&[])
            .bool()
            .int();
        // println!("OBJ: {}\n\nTRUTH: {}", out.objectness, objectness_spans);
        let objectness_loss = self.bce_loss.forward(
            out.objectness.clone().flatten::<1>(0, 1),
            objectness_spans.clone().flatten(0, 1),
        );

        // println!(
        //     "ANY NAN? RL {} OL {}",
        //     regression_loss.contains_nan().into_scalar(),
        //     objectness_loss.contains_nan().into_scalar()
        // );
        // println!(
        //     "AVERAGE RL {} OL {}",
        //     regression_loss.clone().mean().into_scalar(),
        //     objectness_loss.clone().mean().into_scalar()
        // );
        println!("LOSS RL {} OL {}", regression_loss, objectness_loss);
        TrainOutput::new(
            self,
            (regression_loss.clone() * 5 + objectness_loss.clone()).backward(),
            loss::ModelOutput {
                loss: regression_loss.clone() * 5 + objectness_loss,
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
        self.forward(item.features, item.edges, item.graph_feature_indices)
    }
}
