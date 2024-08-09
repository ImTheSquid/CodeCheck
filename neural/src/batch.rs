use std::marker::PhantomData;

use burn::{
    data::dataloader::batcher::Batcher,
    tensor::{backend::Backend, Int, Tensor},
};
use syntree::{index::Index, pointer::Width, Node, Tree};

pub struct GatBatch<B: Backend> {
    /// The vector indexed 0 is for batching
    /// The vector indexed 1 is each tree
    /// The vector indexed 2 is the target of the attention heads
    /// All subsequent rows contain neighboring vectors (should include itself)
    pub family: Tensor<B, 4, Int>,
}

#[derive(Debug, Default, Clone)]
pub struct GatBatcher<B: Backend> {
    _backend: PhantomData<B>,
}

impl<B: Backend, T: Into<Tensor<B, 1, Int>> + Copy, I: Index, W: Width>
    Batcher<Tree<T, I, W>, GatBatch<B>> for GatBatcher<B>
{
    // TODO: Turn this into an AssociatedStruct<Tree> as input, since that is the actual data here
    fn batch(&self, items: Vec<Tree<T, I, W>>) -> GatBatch<B> {
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
