use burn::{nn, tensor::backend::Backend};

pub struct Gat<B: Backend> {
    attention: nn::Linear<B>,
}
