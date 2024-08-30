use ast::{guess_language_from_path, Language, SyntaxTree};
use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    prelude::Backend,
    tensor::Tensor,
};
use core::range::Range;
use std::{
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};
use util::{
    find_paired_indices_from_pair_index, Dataset as MarkDataset, DatasetError, Mark, PairedIndices,
};
use walkdir::WalkDir;

const TRAIN_SPLIT: f32 = 0.8;

/// AST dataset as it exists on the filesystem
pub struct RawAstDataset {
    language: Language,
    files: Vec<PathBuf>,
    dataset: MarkDataset,
}

impl TryFrom<&Path> for RawAstDataset {
    type Error = DatasetError;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let (entries, langs): (Vec<_>, Vec<_>) = WalkDir::new(value)
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.ok()?;

                let lang = guess_language_from_path(entry.path()).ok()?;

                Some((entry.path().to_path_buf(), lang))
            })
            .unzip();

        if langs.is_empty() || langs.iter().any(|l| *l != langs[0]) {
            return Err(DatasetError::InvalidComposition);
        }

        let dataset: MarkDataset =
            serde_json::from_str(&fs::read_to_string(value.join("dataset.json"))?)?;

        Ok(Self {
            language: langs[0],
            files: entries,
            dataset,
        })
    }
}

impl RawAstDataset {
    fn num_comps(&self) -> usize {
        self.files.len() * (self.files.len() - 1) / 2
    }

    pub fn train(&self) -> AstDataset<'_> {
        AstDataset {
            base: self,
            range: Range::from(0..(self.num_comps() as f32 * TRAIN_SPLIT) as usize),
        }
    }

    pub fn test(&self) -> AstDataset<'_> {
        AstDataset {
            base: self,
            range: Range::from((self.num_comps() as f32 * TRAIN_SPLIT) as usize..self.num_comps()),
        }
    }
}

pub struct AstDataset<'a> {
    base: &'a RawAstDataset,
    range: Range<usize>,
}

impl<'a> Dataset<AstDatasetSingle<'a>> for AstDataset<'a> {
    fn len(&self) -> usize {
        self.range.end - self.range.start
    }

    fn get(&self, index: usize) -> Option<AstDatasetSingle<'a>> {
        if !self.range.contains(&index) {
            return None;
        }

        let PairedIndices { i, j } =
            find_paired_indices_from_pair_index(index, self.base.num_comps());

        Some(AstDatasetSingle {
            a: self.base.files[i].as_path(),
            b: self.base.files[j].as_path(),
            language: self.base.language,
            marks: self
                .base
                .dataset
                .pairs
                .get(&index)
                .expect("dataset to exist")
                .marks
                .as_slice(),
        })
    }
}

pub struct AstDatasetSingle<'a> {
    a: &'a Path,
    b: &'a Path,
    language: Language,
    marks: &'a [Mark],
}

pub struct AstBatcher<B: Backend> {
    _device: PhantomData<B>,
}

const MAX_SPANS: usize = 50;

impl<B: Backend> Batcher<AstDatasetSingle<'_>, AstBatch<B>> for AstBatcher<B> {
    fn batch(&self, items: Vec<AstDatasetSingle>) -> AstBatch<B> {
        // Read each item in the dataset, loading in all of the files in each batch
        // This is gonna take a ton of memory but oh well
        let (edge_indices, features, spans): (Vec<_>, Vec<_>, Vec<_>) = itertools::multiunzip(
            items
                .into_iter()
                .map(
                    |AstDatasetSingle {
                         a,
                         b,
                         language,
                         marks,
                     }| {
                        // Traverse the tree in the default order that syntree does, converting nodes to features
                        let (a_edge, a_feature) = build_edge_indices_and_features::<B>(a, language)
                            .expect("Valid tree build A");
                        let (b_edge, b_feature) = build_edge_indices_and_features::<B>(b, language)
                            .expect("Valid tree build B");

                        let edges: Tensor<B, 2> = Tensor::stack(vec![a_edge, b_edge], 0);
                        let features: Tensor<B, 2> = Tensor::stack(vec![a_feature, b_feature], 0);

                        // Load spans
                        assert!(
                            marks.len() <= MAX_SPANS,
                            "Too many marks for files {a:?} {b:?}"
                        );

                        let num_marks = marks.len();

                        let marks = marks.into_iter().map(|m| {
                            Tensor::<B, 1>::from_floats(
                                [
                                    m.a.start as f32,
                                    m.a.end as f32,
                                    m.b.start as f32,
                                    m.b.end as f32,
                                ],
                                &B::Device::default(),
                            )
                        });

                        let marks = Tensor::stack(marks.collect(), 0);

                        let padding = Tensor::<B, 2>::full(
                            [MAX_SPANS - num_marks, 4],
                            -1.0,
                            &B::Device::default(),
                        );

                        let spans = Tensor::cat(vec![marks, padding], 1);

                        (edges, features, spans)
                    },
                )
                .collect::<Vec<(_, _, _)>>(),
        );

        AstBatch {
            edge_indices: Tensor::stack(edge_indices, 0),
            features: Tensor::stack(features, 0),
            spans: Tensor::stack(spans, 0),
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum DataError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Tree(#[from] ast::TreeParseError),
}

fn build_edge_indices_and_features<B: Backend>(
    path: &Path,
    language: Language,
) -> Result<(Tensor<B, 2>, Tensor<B, 2>), DataError> {
    let file_data = fs::read_to_string(path)?;

    Ok(match language {
        Language::C => {
            let tree = ast::c::CTree::try_from(file_data)?.symbol_tree()?;
            convert_tree_to_tensor(tree)
        }
        Language::Cpp => {
            let tree = ast::cpp::CppTree::try_from(file_data)?.symbol_tree()?;
            convert_tree_to_tensor(tree)
        }
        Language::Java => {
            let tree = ast::java::JavaTree::try_from(file_data)?.symbol_tree()?;
            convert_tree_to_tensor(tree)
        }
        Language::Python => {
            todo!()
        }
    })
}

fn convert_tree_to_tensor<B: Backend, T>(
    tree: syntree::Tree<T, usize, usize>,
) -> (Tensor<B, 2>, Tensor<B, 2>)
where
    T: Copy + Into<Tensor<B, 1>>,
{
    let mut edge_indices = Vec::with_capacity(tree.len());
    let mut features = Vec::with_capacity(tree.len());
    let mut last_index = 0;
    let mut parent_index_stack = vec![];

    for (i, (event, node)) in tree.walk_events().enumerate() {
        match event {
            syntree::node::Event::Up => {
                parent_index_stack.pop();
            }
            syntree::node::Event::Down => {
                parent_index_stack.push(last_index);
            }
            syntree::node::Event::Next => {}
        }

        if let Some(parent_idx) = parent_index_stack.last() {
            edge_indices.push(Tensor::from_floats(
                [*parent_idx as f32, i as f32],
                &B::Device::default(),
            ));
        }

        features.push(node.value().into());

        last_index = i;
    }

    (Tensor::stack(edge_indices, 0), Tensor::stack(features, 0))
}

/// Represents a batch of ASTs for training
/// Fastest dimensions are the actual data
/// Second slowest dimension is each pair (doesn't exist for spans)
/// Slowest dimension is a vector of pairs
pub struct AstBatch<B: Backend> {
    edge_indices: Tensor<B, 4>,
    features: Tensor<B, 4>,
    spans: Tensor<B, 3>,
}
