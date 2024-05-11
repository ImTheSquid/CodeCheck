use std::marker::PhantomData;

use burn::{
    data::dataloader::batcher::Batcher,
    tensor::{backend::Backend, Int, Tensor},
};
use syntree::{pointer::Width, Node};

pub struct GatBatch<B: Backend> {
    /// The vector indexed 0 is for batching
    /// The vector indexed 1 is the target of the attention heads
    /// All subsequent rows contain neighboring vectors (should include itself)
    pub family: Tensor<B, 3, Int>,
}

#[derive(Debug, Default, Clone)]
pub struct GatBatcher<B: Backend> {
    _backend: PhantomData<B>,
}

impl<'a, B: Backend, T: Into<Tensor<B, 1, Int>> + Copy, I, W: Width>
    Batcher<Node<'a, T, I, W>, GatBatch<B>> for GatBatcher<B>
{
    fn batch(&self, items: Vec<Node<'a, T, I, W>>) -> GatBatch<B> {
        let tensors = items
            .into_iter()
            .map(|n| {
                let tensors = n
                    .children()
                    .map(|c| Some(c.into()))
                    .chain(n.parent().map(Into::into))
                    .filter_map(|maybe_tensor| maybe_tensor)
                    .map(|tensor| (*tensor.value()).into())
                    .collect::<Vec<Tensor<B, 1, _>>>();

                let tensors = Tensor::stack::<2>(tensors, 0);

                tensors
            })
            .collect();

        let tensors = Tensor::stack::<3>(tensors, 0);

        GatBatch { family: tensors }
    }
}
