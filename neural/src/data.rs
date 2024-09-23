use ast::{guess_language_from_path, Language, SyntaxTree};
use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    prelude::Backend,
    tensor::Tensor,
};
use core::range::Range;
use std::{
    collections::HashMap,
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};
use util::{
    find_paired_indices_from_pair_index, Dataset as MarkDataset, DatasetError, Mark, Pair,
    PairedIndices,
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

#[derive(Debug)]
struct LanguageBoundPath {
    language: Language,
    path: PathBuf,
}

impl LanguageBoundPath {
    fn as_ref(&self) -> LanguageBoundPathRef<'_> {
        LanguageBoundPathRef {
            language: self.language,
            path: self.path.as_path(),
        }
    }
}

#[derive(Debug)]
pub struct LanguageBoundPathRef<'a> {
    language: Language,
    path: &'a Path,
}

pub trait AnyDataset: Send + Sync {
    fn num_comps(&self) -> usize;

    fn train(&self) -> AstDataset<'_, Self>;

    fn test(&self) -> AstDataset<'_, Self>;

    fn file(&self, index: usize) -> LanguageBoundPathRef<'_>;

    fn pair(&self, index: usize) -> Option<&Pair>;
}

#[derive(Debug, Default)]
pub struct CollatedAstDataset {
    files: Vec<LanguageBoundPath>,
    dataset: HashMap<usize, Pair>,
}

impl CollatedAstDataset {
    pub fn include(&mut self, dataset: RawAstDataset) {
        self.dataset.extend(
            dataset
                .dataset
                .pairs
                .into_iter()
                .map(|(k, v)| (k + self.files.len(), v)),
        );

        self.files
            .extend(dataset.files.into_iter().map(|p| LanguageBoundPath {
                language: dataset.language,
                path: p,
            }));
    }
}

impl AnyDataset for CollatedAstDataset {
    fn num_comps(&self) -> usize {
        self.files.len() * (self.files.len() - 1) / 2
    }

    fn train(&self) -> AstDataset<'_, Self> {
        AstDataset {
            base: self,
            range: Range::from(0..(self.num_comps() as f32 * TRAIN_SPLIT) as usize),
        }
    }

    fn test(&self) -> AstDataset<'_, Self> {
        AstDataset {
            base: self,
            range: Range::from((self.num_comps() as f32 * TRAIN_SPLIT) as usize..self.num_comps()),
        }
    }

    fn file(&self, index: usize) -> LanguageBoundPathRef<'_> {
        self.files[index].as_ref()
    }

    fn pair(&self, index: usize) -> Option<&Pair> {
        self.dataset.get(&index)
    }
}

impl AnyDataset for RawAstDataset {
    fn num_comps(&self) -> usize {
        self.files.len() * (self.files.len() - 1) / 2
    }

    fn train(&self) -> AstDataset<'_, Self> {
        AstDataset {
            base: self,
            range: Range::from(0..(self.num_comps() as f32 * TRAIN_SPLIT) as usize),
        }
    }

    fn test(&self) -> AstDataset<'_, Self> {
        AstDataset {
            base: self,
            range: Range::from((self.num_comps() as f32 * TRAIN_SPLIT) as usize..self.num_comps()),
        }
    }

    fn file(&self, index: usize) -> LanguageBoundPathRef<'_> {
        LanguageBoundPathRef {
            path: self.files[index].as_path(),
            language: self.language,
        }
    }

    fn pair(&self, index: usize) -> Option<&Pair> {
        self.dataset.pairs.get(&index)
    }
}

pub struct AstDataset<'a, Base: AnyDataset + ?Sized> {
    base: &'a Base,
    range: Range<usize>,
}

impl<'a, Base: AnyDataset + ?Sized> Dataset<AstDatasetSingle<'a>> for AstDataset<'a, Base> {
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
            a: self.base.file(i),
            b: self.base.file(j),
            marks: self
                .base
                .pair(index)
                .map(|p| p.marks.as_slice())
                .unwrap_or_default(),
        })
    }
}

pub struct AstDatasetSingle<'a> {
    a: LanguageBoundPathRef<'a>,
    b: LanguageBoundPathRef<'a>,
    marks: &'a [Mark],
}

pub struct AstBatcher<B: Backend> {
    _device: PhantomData<B>,
}

pub const MAX_SPANS: usize = 50;
pub const MAX_NODES: usize = 100_000;
pub const MAX_EDGES: usize = MAX_NODES - 1;

impl<B: Backend> Batcher<AstDatasetSingle<'_>, AstBatch<B>> for AstBatcher<B> {
    fn batch(&self, items: Vec<AstDatasetSingle>) -> AstBatch<B> {
        // Read each item in the dataset, loading in all of the files in each batch
        // This is gonna take a ton of memory but oh well
        let (edges, features, spans): (Vec<_>, Vec<_>, Vec<_>) = itertools::multiunzip(
            items
                .into_iter()
                .map(|AstDatasetSingle { a, b, marks }| {
                    // Traverse the tree in the default order that syntree does, converting nodes to features
                    let (a_edge, a_feature) = build_edges_and_features::<B>(a.path, a.language, 0)
                        .expect("Valid tree build A");
                    let (b_edge, b_feature) =
                        build_edges_and_features::<B>(b.path, b.language, a_edge.shape().dims[0])
                            .expect("Valid tree build B");

                    let edge = Tensor::cat(vec![a_edge, b_edge], 0);
                    let features = Tensor::cat(vec![a_feature, b_feature], 0);

                    // Load spans
                    assert!(
                        marks.len() <= MAX_SPANS,
                        "Too many marks for files {a:?} {b:?}"
                    );

                    let num_marks = marks.len();

                    let marks = marks.iter().map(|m| {
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

                    (edge, features, spans)
                })
                .collect::<Vec<(_, _, _)>>(),
        );

        AstBatch {
            edges: Tensor::stack(edges, 0),
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

fn build_edges_and_features<B: Backend>(
    path: &Path,
    language: Language,
    index_offset: usize,
) -> Result<(Tensor<B, 2>, Tensor<B, 2>), DataError> {
    let file_data = fs::read_to_string(path)?;

    Ok(match language {
        Language::C => {
            let tree = ast::c::CTree::try_from(file_data)?.symbol_tree()?;
            convert_tree_to_tensor(tree, index_offset)
        }
        Language::Cpp => {
            let tree = ast::cpp::CppTree::try_from(file_data)?.symbol_tree()?;
            convert_tree_to_tensor(tree, index_offset)
        }
        Language::Java => {
            let tree = ast::java::JavaTree::try_from(file_data)?.symbol_tree()?;
            convert_tree_to_tensor(tree, index_offset)
        }
        Language::Python => {
            todo!()
        }
    })
}

fn convert_tree_to_tensor<B: Backend, T>(
    tree: syntree::Tree<T, usize, usize>,
    index_offset: usize,
) -> (Tensor<B, 2>, Tensor<B, 2>)
where
    T: Copy + Into<Tensor<B, 1>>,
{
    let mut edge_indices = Range::from(0..tree.len())
        .into_iter()
        .map(|_| vec![])
        .collect::<Vec<_>>();
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
            edge_indices[*parent_idx].push(i);
        }

        features.push(node.value().into());

        last_index = i;
    }

    let mut paired_indices: Vec<Tensor<B, 1>> = vec![];

    for (from, list) in edge_indices.iter().enumerate() {
        for to in list {
            paired_indices.push(Tensor::from_floats(
                [(from + index_offset) as f32, (*to + index_offset) as f32],
                &B::Device::default(),
            ))
        }
    }

    // Self-attention
    for i in Range::from(0..tree.len())
        .into_iter()
        .map(|i| (i + index_offset) as f32)
    {
        paired_indices.push(Tensor::from_floats([i, i], &B::Device::default()));
    }

    (Tensor::stack(paired_indices, 0), Tensor::stack(features, 0))
}

/// Represents a batch of ASTs for training
pub struct AstBatch<B: Backend> {
    /// [batch_size, E * 2, 2]
    edges: Tensor<B, 3>,
    /// [batch_size, N * 2, F]
    features: Tensor<B, 3>,
    /// [batch_size, MAX_SPANS, 4]
    spans: Tensor<B, 3>,
}
