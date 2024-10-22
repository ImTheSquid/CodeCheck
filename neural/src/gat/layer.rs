use burn::{
    config::Config,
    module::{Module, Param, ParamId},
    nn::{Dropout, DropoutConfig, Initializer, LeakyRelu, LeakyReluConfig, Linear, LinearConfig},
    prelude::Backend,
    tensor::{Distribution, Int, Shape, Tensor},
};

use crate::elu::{Elu, EluConfig};

#[derive(Debug, Config)]
pub struct GatLayerConfig {
    in_features: usize,
    out_features: usize,
    num_heads: usize,
    concat: bool,
    activation: bool,
    dropout_prob: f64,
    add_skip: bool,
    bias: bool,
}

#[derive(Debug, Module)]
pub struct GatLayer<B: Backend> {
    linear_projection: Linear<B>,
    skip_projection: Option<Linear<B>>,
    dropout: Dropout,
    relu: LeakyRelu,
    concat: bool,
    activation: Option<Elu>,
    bias: Option<Param<Tensor<B, 1>>>,
    scoring_fn_target: Param<Tensor<B, 3>>,
    scoring_fn_source: Param<Tensor<B, 3>>,
    num_heads: usize,
    out_features: usize,
}

impl GatLayerConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> GatLayer<B> {
        GatLayer {
            linear_projection: LinearConfig::new(
                self.in_features,
                self.out_features * self.num_heads,
            )
            .with_bias(false)
            .init(device),
            skip_projection: if self.add_skip {
                Some(
                    LinearConfig::new(self.in_features, self.out_features * self.num_heads)
                        .with_bias(false)
                        .init(device),
                )
            } else {
                None
            },
            dropout: DropoutConfig::new(self.dropout_prob).init(),
            relu: LeakyReluConfig::new().with_negative_slope(0.2).init(),
            concat: self.concat,
            activation: if self.activation {
                Some(EluConfig::new().init())
            } else {
                None
            },
            bias: if self.bias {
                Some(Initializer::XavierUniform { gain: 1.0 }.init_with(
                    [if self.concat {
                        self.num_heads * self.out_features
                    } else {
                        self.num_heads
                    }],
                    Some(self.in_features),
                    Some(if self.concat {
                        self.num_heads * self.out_features
                    } else {
                        self.num_heads
                    }),
                    device,
                ))
            } else {
                None
            },
            scoring_fn_target: Initializer::XavierUniform { gain: 1.0 }.init_with(
                [1, self.num_heads, self.out_features],
                Some(self.num_heads),
                Some(self.out_features),
                device,
            ),
            scoring_fn_source: Initializer::XavierUniform { gain: 1.0 }.init_with(
                [1, self.num_heads, self.out_features],
                Some(self.num_heads),
                Some(self.out_features),
                device,
            ),
            num_heads: self.num_heads,
            out_features: self.out_features,
        }
    }
}

impl<B: Backend> GatLayer<B> {
    /// Input:
    /// Edges: [batch_size, E * 2, 2]
    /// Features: [batch_size, N * 2, F]
    pub fn forward(&self, edges: &Tensor<B, 3, Int>, features: Tensor<B, 3>) -> Tensor<B, 3> {
        println!(
            "LAYER FORWARD: E {:?} F {:?} LP {:?} OF {} NH {}",
            edges.dims(),
            features.dims(),
            self.linear_projection.weight.dims(),
            self.out_features,
            self.num_heads,
        );
        let in_skip = features.clone();
        // Linear projection and regularization
        let features_dims = features.dims();
        let features = self.dropout.forward(features);

        let features = self.linear_projection.forward(features);

        let features: Tensor<B, 4> =
            features.reshape([0, 0, self.num_heads as i32, self.out_features as i32]);

        let features_proj = self.dropout.forward(features);

        // Edge attention

        // Apply scoring function
        let scores_source = self
            .scoring_fn_source
            .val()
            .expand(features_proj.shape())
            .mul(features_proj.clone())
            .sum_dim(3);
        let scores_source = scores_source.squeeze_dims::<3>(&[]);
        let scores_target = self
            .scoring_fn_target
            .val()
            .expand(features_proj.shape())
            .mul(features_proj.clone())
            .sum_dim(3);
        let scores_target = scores_target.squeeze_dims::<3>(&[]);

        let (scores_source, scores_target, features_proj) =
            self.lift(scores_source, scores_target, features_proj, edges.clone());
        let scores_per_edge = self.relu.forward(scores_source + scores_target);

        let attentions_per_edge = self.neighborhood_aware_softmax(
            scores_per_edge,
            edges
                .clone()
                .slice([None, None, Some((1, -1))])
                .squeeze_dims(&[]),
            features_dims[1],
        );
        let attentions_per_edge = self.dropout.forward(attentions_per_edge);
        let attns = attentions_per_edge.clone();

        // Neighborhood aggregation
        let node_features_projected_lifted_weighted = features_proj * attentions_per_edge;
        let out_node_features = self.aggregate_neighbors(
            node_features_projected_lifted_weighted,
            edges.clone(),
            features_dims[1],
        );

        self.skip_concat_bias(attns, in_skip, out_node_features)
    }

    fn lift(
        &self,
        scores_source: Tensor<B, 3>,
        scores_target: Tensor<B, 3>,
        node_features_projected: Tensor<B, 4>,
        edge_indices: Tensor<B, 3, Int>,
    ) -> (Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 4>) {
        let source_node_indices: Tensor<B, 2, Int> = edge_indices
            .clone()
            .slice([None, None, Some((0, 1))])
            .squeeze_dims(&[]);
        let target_node_indices: Tensor<B, 2, Int> = edge_indices
            .slice([None, None, Some((1, 2))])
            .squeeze_dims(&[]);

        let source_node_dims = source_node_indices.dims();
        let target_node_dims = target_node_indices.dims();

        let scores_source: Tensor<B, 3> = Tensor::stack(
            scores_source
                .chunk(source_node_dims[0], 0)
                .into_iter()
                .zip(source_node_indices.clone().chunk(source_node_dims[0], 0))
                .map(|(c, i)| c.select(2, i.squeeze_dims(&[])))
                .collect::<Vec<_>>(),
            0,
        );
        let scores_target: Tensor<B, 3> = Tensor::stack(
            scores_target
                .chunk(target_node_dims[0], 0)
                .into_iter()
                .zip(target_node_indices.chunk(target_node_dims[0], 0))
                .map(|(c, i)| c.select(2, i.squeeze_dims(&[])))
                .collect(),
            0,
        );
        let node_features_projected: Tensor<B, 4> = Tensor::stack(
            node_features_projected
                .chunk(source_node_dims[0], 0)
                .into_iter()
                .zip(source_node_indices.chunk(source_node_dims[0], 0))
                .map(|(c, i)| c.select(2, i.squeeze_dims(&[])))
                .collect(),
            0,
        );

        (scores_source, scores_target, node_features_projected)
    }

    fn skip_concat_bias(
        &self,
        attention_coefficients: Tensor<B, 4>,
        in_node_features: Tensor<B, 3>,
        mut out_node_features: Tensor<B, 4>,
    ) -> Tensor<B, 3> {
        if let Some(skip) = &self.skip_projection {
            if out_node_features.dims()[3] == in_node_features.dims()[2] {
                out_node_features = out_node_features + in_node_features.unsqueeze_dim(2);
            } else {
                out_node_features = out_node_features
                    + skip.forward(in_node_features).reshape([
                        -1,
                        -1,
                        self.num_heads as i32,
                        self.out_features as i32,
                    ])
            }
        }

        let mut out_node_features: Tensor<B, 3> = if self.concat {
            out_node_features.reshape([-1, -1, (self.num_heads * self.out_features) as i32])
        } else {
            out_node_features.mean_dim(2).squeeze_dims::<3>(&[])
        };

        if let Some(bias) = &self.bias {
            out_node_features = out_node_features + bias.val().expand([-1, -1, -1]);
        }

        if let Some(activation) = &self.activation {
            activation.forward(out_node_features)
        } else {
            out_node_features
        }
    }

    fn aggregate_neighbors(
        &self,
        node_features_projected: Tensor<B, 4>,
        edge_indices: Tensor<B, 3, Int>,
        num_nodes: usize,
    ) -> Tensor<B, 4> {
        let mut size = node_features_projected.dims();
        size[1] = num_nodes;
        let out_node_features: Tensor<B, 4> =
            Tensor::zeros(size, &node_features_projected.device());
        let target_index_broadcasted = edge_indices
            .slice([None, None, Some((0, 1))])
            .squeeze_dims::<2>(&[])
            .expand(node_features_projected.shape());

        out_node_features.scatter(1, target_index_broadcasted, node_features_projected)
    }

    fn neighborhood_aware_softmax(
        &self,
        scores_per_edge: Tensor<B, 3>,
        target_index: Tensor<B, 2, Int>,
        num_nodes: usize,
    ) -> Tensor<B, 4> {
        let max_score = scores_per_edge.clone().max().into_scalar();
        let scores_per_edge = scores_per_edge - max_score;
        let exp_scores_per_edge = scores_per_edge.exp();

        let denom = self.sum_edge_scores_neighborhood_aware(
            exp_scores_per_edge.clone(),
            target_index,
            num_nodes,
        );

        let attentions_per_edge = exp_scores_per_edge / (denom + 1e-16);

        attentions_per_edge.unsqueeze_dims(&[-1])
    }

    fn sum_edge_scores_neighborhood_aware(
        &self,
        exp_scores_per_edge: Tensor<B, 3>,
        target_index: Tensor<B, 2, Int>,
        num_nodes: usize,
    ) -> Tensor<B, 3> {
        let target_index_broadcasted = target_index.clone().expand(exp_scores_per_edge.shape());

        let mut size = exp_scores_per_edge.dims();
        size[1] = num_nodes;
        let neighborhood_sums = Tensor::zeros(size, &exp_scores_per_edge.device());
        let neighborhood_sums =
            neighborhood_sums.scatter(1, target_index_broadcasted, exp_scores_per_edge);

        Tensor::stack(
            neighborhood_sums
                .chunk(size[0], 0)
                .into_iter()
                .zip(target_index.chunk(size[0], 0))
                .map(|(c, i)| c.select(1, i.squeeze_dims(&[])))
                .collect(),
            0,
        )
    }
}
