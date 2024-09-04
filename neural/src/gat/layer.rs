use burn::{
    config::Config,
    module::{Module, Param, ParamId},
    nn::{Dropout, DropoutConfig, Initializer, LeakyRelu, LeakyReluConfig, Linear, LinearConfig},
    prelude::Backend,
    tensor::{Distribution, Tensor},
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
    relu: LeakyRelu<B>,
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
                Some(Initializer::XavierUniform { gain: 1.0 }.init(
                    [if self.concat {
                        self.num_heads * self.out_features
                    } else {
                        self.num_heads
                    }],
                    device,
                ))
            } else {
                None
            },
            scoring_fn_target: Initializer::XavierUniform { gain: 1.0 }
                .init([1, self.num_heads, self.out_features], device),
            scoring_fn_source: Initializer::XavierUniform { gain: 1.0 }
                .init([1, self.num_heads, self.out_features], device),
            num_heads: self.num_heads,
            out_features: self.out_features,
        }
    }
}

impl<B: Backend> GatLayer<B> {
    /// Input:
    /// Edges: [batch_size, E * 2, 2]
    /// Features: [batch_size, N * 2, F]
    pub fn forward(
        &self,
        edges: Tensor<B, 3>,
        features: Tensor<B, 3>,
    ) -> (Tensor<B, 2>, Tensor<B, 2>) {
        let features = self.dropout.forward(features);

        let features = self.linear_projection.forward(features);

        let features: Tensor<B, 3> =
            features.reshape([-1, self.num_heads as i32, self.out_features as i32]);

        let features_proj = self.dropout.forward(features);

        let scores_source = self
            .scoring_fn_source
            .val()
            .mul(features_proj.clone())
            .sum_dim(2);
        let scores_source = scores_source.squeeze::<2>(2);
        let scores_target = self.scoring_fn_target.val().mul(features_proj).sum_dim(2);
        let scores_target = scores_target.squeeze::<2>(2);

        todo!()
    }
}
