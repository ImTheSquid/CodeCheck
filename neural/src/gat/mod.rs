use core::range::Range;

use burn::{
    config::Config,
    module::Module,
    prelude::Backend,
    tensor::{Int, Tensor},
};
use layer::{GatLayer, GatLayerConfig};

pub mod layer;

#[derive(Debug, Config)]
pub struct GatConfig {
    // Vector of size num_layers + 1
    num_features: Vec<usize>,
    // Vector of size num_layers
    num_heads: Vec<usize>,
    add_skip_connection: bool,
    bias: bool,
    dropout: f64,
}

impl GatConfig {
    fn heads_for_layer(&self, i: usize) -> usize {
        if i == 0 {
            1
        } else {
            self.num_heads[i - 1]
        }
    }

    pub fn init<B: Backend>(&self, device: &B::Device) -> Gat<B> {
        Gat {
            layers: Range::from(0..self.num_heads.len())
                .into_iter()
                .map(|i| {
                    GatLayerConfig::new(
                        self.num_features[i] * self.heads_for_layer(i),
                        self.num_features[i + 1],
                        self.heads_for_layer(i + 1),
                        i < self.num_heads.len() - 1,
                        i < self.num_heads.len() - 1,
                        self.dropout,
                        self.add_skip_connection,
                        self.bias,
                    )
                    .init(device)
                })
                .collect(),
        }
    }
}

#[derive(Debug, Module)]
pub struct Gat<B: Backend> {
    layers: Vec<GatLayer<B>>,
}

impl<B: Backend> Gat<B> {
    pub fn forward(&self, edges: Tensor<B, 3, Int>, mut features: Tensor<B, 3>) -> Tensor<B, 3> {
        for layer in &self.layers {
            features = layer.forward(&edges, features);
        }

        features
    }
}
