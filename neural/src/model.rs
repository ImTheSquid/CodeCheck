use std::ops::RangeInclusive;

use burn::{
    config::Config,
    module::Module,
    nn::{
        loss::{BinaryCrossEntropyLoss, BinaryCrossEntropyLossConfig},
        LinearConfig,
    },
    prelude::Backend,
    tensor::{Int, Tensor, Transaction},
    train::{metric::ItemLazy, ValidStep},
};

use crate::{
    data::{AstBatch, MAX_NODES, MAX_SPANS},
    gat::{Gat, GatConfig},
    leaky_gain,
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
}

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        let gat_output = *self.gat.num_features.last().unwrap();
        Model {
            gat: self.gat.init(device),
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
                SequentialLayerConfig::Sigmoid,
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
    gat: Gat<B>,
    regression: Sequential<B>,
    objectness: Sequential<B>,
    bce_loss: BinaryCrossEntropyLoss<B>,
}

pub struct ModelResult<B: Backend> {
    pub regression: Tensor<B, 3>,
    pub objectness: Tensor<B, 2>,
}

pub struct MappedTensor<B: Backend> {
    pub mapped: Tensor<B, 1>,
    pub span: RangeInclusive<usize>,
}

impl<B: Backend> Model<B> {
    pub fn forward(&self, _features: Tensor<B, 2>, _edges: Tensor<B, 2, Int>) -> ModelResult<B> {
        // println!(
        //     "MODEL FORWARD: F {:?} E {:?}",
        //     features.dims(),
        //     edges.dims()
        // );
        // println!("FORWARD: {features}\nE: {edges}\nGFI: {graph_feature_indices}");
        // let features = self.node_processor.forward(features);

        // println!("PROC COMPLETE: {features}\n");

        // let features = self.gat.forward(edges, features);

        // let mut found_spans = Vec::new();

        // Traverse the tree using postorder, feeding it through the LSTM
        // These are batched so this must be done multiple times
        // for &feature_index in graph_feature_indices
        //     .to_data()
        //     .as_slice::<i64>()
        //     .expect("valid slice")
        // {
        //     struct Info<B: Backend> {
        //         index: usize,
        //         feat: Tensor<B, 1>,
        //     }
        //     let mut stack1 = vec![Info {
        //         index: feature_index as usize,
        //         feat: features
        //             .clone()
        //             .slice([Some((feature_index, feature_index + 1)), None])
        //             .squeeze::<1>(1),
        //     }];
        //     let mut stack2 = Vec::new();
        //     while let Some(node) = stack1.pop() {
        //         let children = edges_hash.get(&node.index);
        //         stack2.push(node);

        //         for &child in children.unwrap_or(&vec![]) {
        //             let index = child;
        //             let child = child as i64;
        //             stack1.push(Info {
        //                 index,
        //                 feat: features
        //                     .clone()
        //                     .slice([Some((child, child + 1)), None])
        //                     .squeeze(1),
        //             });
        //         }
        //     }

        //     // Reverse and stack stack2
        //     let stack2: Vec<_> = stack2.into_iter().rev().collect();
        //     let mut stack2 = stack2.into_iter();

        //     let mut lstm_state = None;

        //     let first_node = stack2.next().expect("nonempty tree");
        //     let (output, lstm_state) = self
        //         .lstm
        //         .forward(first_node.feat.unsqueeze_dims(&[0, 1]), None);
        //     let mut start_idx = first_node.index;
        //     // Tracks whether the start index needs to be updated after pushing a new mapped tensor
        //     let mut needs_start_update = false;
        //     for itm in stack2 {
        //         if needs_start_update {
        //             start_idx = itm.index;
        //             needs_start_update = false;
        //         }

        //         (output, lstm_state) = self
        //             .lstm
        //             .forward(itm.feat.unsqueeze_dims(&[0, 1]), Some(lstm_state));

        //         // Feed LSTM output to output gate
        //         let output = self.output_gate.forward(output.squeeze_dims::<1>(&[0, 1]));
        //         let output = softmax(output.clone(), 0);
        //         // If argmax is 0, STOP, otherwise CONTINUE
        //         if output.argmax(0).into_scalar().to_i64() == 0 {
        //             // Grab the output of the LSTM and push it along with its location data to the queue.
        //             found_spans.push(MappedTensor {
        //                 mapped: output,
        //                 span: start_idx..=itm.index,
        //             });
        //             needs_start_update = true;
        //         }
        //     }
        // }

        // // Run HDBSCAN
        // let tensors = found_spans
        //     .into_iter()
        //     .map(|t| t.mapped.into_data().as_slice::<f64>().unwrap().to_vec())
        //     .collect::<Vec<_>>();
        // let clusterer = {
        //     // Every cluster must have at least two points
        //     let params = HdbscanHyperParams::builder().min_cluster_size(2).build();
        //     Hdbscan::new(&tensors, params)
        // };
        // // Grab cluster assignments
        // let clusters = clusterer.cluster().expect("successful cluster generation");

        // println!("GAT COMPLETE: {features}\n");

        // println!(
        //     "ABSOLUTE MAX: {} SUM: {}",
        //     features.clone().abs().max(),
        //     features.clone().sum()
        // );

        // println!("GAT COMPLETE");

        // let num_pairs = (graph_feature_indices.clone().max().into_scalar().to_i64() + 1) / 2;

        // let mut expanded = Tensor::<B, 3>::zeros(
        //     [num_pairs as usize, MAX_NODES, features.dims()[1]],
        //     &features.device(),
        // );
        // for pair_index in 0..num_pairs {
        //     let first = pair_index * 2;
        //     let second = first + 1;

        //     // What elements match?
        //     let first_indices = graph_feature_indices
        //         .clone()
        //         .equal_elem(first)
        //         .nonzero()
        //         .into_iter()
        //         .next()
        //         .unwrap_or_else(|| panic!("No indices found for first index {first}!"));
        //     let second_indices = graph_feature_indices
        //         .clone()
        //         .equal_elem(second)
        //         .nonzero()
        //         .into_iter()
        //         .next()
        //         .unwrap_or_else(|| panic!("No indices found for second index {second}!"));
        //     let first_select = features.clone().select(0, first_indices);
        //     let second_select = features.clone().select(0, second_indices).unsqueeze();

        //     let feature_attention = self.attention.forward(MhaInput::new(
        //         first_select.unsqueeze(),
        //         second_select.clone(),
        //         second_select,
        //     ));

        //     let ctx = feature_attention.context.squeeze::<2>(0);

        //     expanded.inplace(|e| {
        //         let pair_index = pair_index as usize;
        //         e.slice_assign(
        //             [
        //                 pair_index..pair_index + 1,
        //                 0..ctx.dims()[0],
        //                 0..ctx.dims()[1],
        //             ],
        //             ctx.unsqueeze(),
        //         )
        //     });
        // }

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

        // let context = expanded.flatten::<2>(1, 2);

        // let regression =
        //     self.regression
        //         .forward(context.clone())
        //         .reshape([-1, MAX_SPANS as i32, 4]);

        // let objectness = self.objectness.forward(context);

        // ModelResult {
        //     regression,
        //     objectness,
        // }

        todo!()
    }
}

// impl<B: AutodiffBackend> TrainStep<AstBatch<B>, ModelOutput<B>> for Model<B> {
//     fn step(&self, item: AstBatch<B>) -> burn::train::TrainOutput<ModelOutput<B>> {
//         // println!(
//         //     "INPUT===================\nF: {}\nE: {}",
//         //     item.features, item.edges
//         // );
//         let out = self.forward(item.features, item.edges, item.graph_feature_indices);
//         // println!(
//         //     "OUTPUT=================\nOBJ: {}\n\nREG: {}",
//         //     out.objectness, out.regression
//         // );
//         let regression_loss: Tensor<B, 1> =
//             loss::GIOULoss::default().forward(out.regression.clone(), item.spans.clone());
//         let objectness_spans = item
//             .spans
//             .clone()
//             .sum_dim(2)
//             .squeeze_dims::<2>(&[])
//             .bool()
//             .int();
//         // println!("OBJ: {}\n\nTRUTH: {}", out.objectness, objectness_spans);
//         let objectness_loss = self.bce_loss.forward(
//             out.objectness.clone().flatten::<1>(0, 1),
//             objectness_spans.clone().flatten(0, 1),
//         );

//         // println!(
//         //     "ANY NAN? RL {} OL {}",
//         //     regression_loss.contains_nan().into_scalar(),
//         //     objectness_loss.contains_nan().into_scalar()
//         // );
//         // println!(
//         //     "AVERAGE RL {} OL {}",
//         //     regression_loss.clone().mean().into_scalar(),
//         //     objectness_loss.clone().mean().into_scalar()
//         // );
//         // println!("LOSS RL {} OL {}", regression_loss, objectness_loss);
//         // let grads = GradientsParams::from_module(
//         //     &mut (regression_loss.clone() + objectness_loss.clone()).backward(),
//         //     self,
//         // );
//         // println!("{grads:?}");
//         TrainOutput::new(
//             self,
//             (regression_loss.clone() + objectness_loss.clone()).backward(),
//             loss::ModelOutput {
//                 loss: regression_loss.clone() + objectness_loss,
//                 regression: BatchedRegressionOutput {
//                     output: out.regression,
//                     targets: item.spans,
//                 },
//                 objectness: ObjectnessOutput {
//                     output: out.objectness,
//                     targets: objectness_spans,
//                 },
//             },
//         )
//     }
// }

impl<B: Backend> ValidStep<AstBatch<B>, ModelResult<B>> for Model<B> {
    fn step(&self, _item: AstBatch<B>) -> ModelResult<B> {
        // self.forward(item.features, item.edges, item.graph_feature_indices)
        todo!()
    }
}

impl<B: Backend> ItemLazy for ModelResult<B> {
    type ItemSync = ModelResult<burn::backend::NdArray>;
    fn sync(self) -> Self::ItemSync {
        let [obs, reg] = Transaction::default()
            .register(self.objectness)
            .register(self.regression)
            .execute()
            .try_into()
            .expect("right number of tensors");

        let dev = &Default::default();

        ModelResult {
            objectness: Tensor::from_data(obs, dev),
            regression: Tensor::from_data(reg, dev),
        }
    }
}
