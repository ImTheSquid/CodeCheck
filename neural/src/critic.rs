// use std::collections::HashMap;

use burn::prelude::*;

use crate::{
    gat::{Gat, GatConfig},
    sequential::{Sequential, SequentialConfig},
};

#[derive(Debug, Config)]
pub struct CriticConfig {
    pub actor_output_seq: SequentialConfig,
    pub gat: GatConfig,
    pub combining_sequential: SequentialConfig,
}

impl CriticConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Critic<B> {
        Critic {
            actor_output_seq: self.actor_output_seq.init(device),
            gat: self.gat.init(device),
            combining_sequential: self.combining_sequential.init(device),
        }
    }
}

/// The critic module takes in the input data and the actor's output, returning a scalar value representing the critic's estimate of the state-action value.
#[derive(Debug, Module)]
pub struct Critic<B: Backend> {
    actor_output_seq: Sequential<B>,
    gat: Gat<B>,
    combining_sequential: Sequential<B>,
}

// impl<B: Backend> Critic<B> {
//     fn forward(
//         &self,
//         input_edges: Tensor<B, 2>,
//         input_features: Tensor<B, 2>,
//         actor_output: Tensor<B, 2>,
//     ) -> Tensor<B, 1> {
//         // let actor_output = self.actor_output_seq.forward(actor_output);
//         // let gat_out = self.gat.forward(input_edges, input_features);
//         // self.combining_sequential
//         //     .forward(Tensor::cat(vec![actor_output, gat_out], 0))
//         todo!()
//     }
// }
