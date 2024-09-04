use burn::{module::Module, prelude::Backend, train::RegressionOutput};

use crate::node_process::NodeProcessor;

#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    node_processor: NodeProcessor<B>,
}

impl<B: Backend> Model<B> {
    pub fn forward_regression_objectness(&self) -> RegressionOutput<B> {
        todo!()
    }
}
