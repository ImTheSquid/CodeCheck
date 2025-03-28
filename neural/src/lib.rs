#![feature(new_range_api)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod contrastive;
pub mod critic;
pub mod data;
pub mod elu;
pub mod gat;
pub mod loss;
pub mod model;
pub mod node_process;
pub mod sequential;

fn leaky_gain(slope: f64) -> f64 {
    (2.0 / (1.0 + slope.powi(2))).sqrt()
}
