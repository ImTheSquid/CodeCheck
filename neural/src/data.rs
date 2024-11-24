use ast::{guess_language_from_path, Language, SyntaxTree};
use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    prelude::Backend,
    tensor::{Int, Tensor},
};
use core::range::Range;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Weak},
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
    self_ref: Weak<Self>,
}

// impl RawAstDataset {
//     fn to_arc(mut self) -> Arc<Self> {
//         Arc::new_cyclic(|d| {
//             self.self_ref = d.clone();
//             self
//         })
//     }
// }

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
            self_ref: Default::default(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LanguageBoundPath {
    language: Language,
    path: PathBuf,
}

// impl LanguageBoundPath {
//     fn as_ref(&self) -> LanguageBoundPathRef<'_> {
//         LanguageBoundPathRef {
//             language: self.language,
//             path: self.path.as_path(),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct LanguageBoundPathRef<'a> {
//     language: Language,
//     path: &'a Path,
// }

pub trait AnyDataset: Send + Sync {
    fn num_comps(&self) -> usize;

    fn train(&self) -> AstDataset<Self>;

    fn test(&self) -> AstDataset<Self>;

    fn file(&self, index: usize) -> LanguageBoundPath;

    fn pair(&self, index: usize) -> Option<&Pair>;

    fn num_files(&self) -> usize;
}

#[derive(Debug, Default, Clone)]
pub struct CollatedAstDataset {
    files: Vec<LanguageBoundPath>,
    dataset: HashMap<usize, Pair>,
    self_ref: Weak<Self>,
}

impl CollatedAstDataset {
    pub fn to_arc(mut self) -> Arc<Self> {
        Arc::new_cyclic(|d| {
            self.self_ref = d.clone();
            self
        })
    }

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

    fn train(&self) -> AstDataset<Self> {
        AstDataset {
            base: self.self_ref.upgrade().expect("upgrade to allocted"),
            range: Range::from(0..(self.num_comps() as f32 * TRAIN_SPLIT) as usize),
        }
    }

    fn test(&self) -> AstDataset<Self> {
        AstDataset {
            base: self.self_ref.upgrade().expect("upgrade to allocted"),
            range: Range::from((self.num_comps() as f32 * TRAIN_SPLIT) as usize..self.num_comps()),
        }
    }

    fn file(&self, index: usize) -> LanguageBoundPath {
        self.files[index].clone()
    }

    fn pair(&self, index: usize) -> Option<&Pair> {
        self.dataset.get(&index)
    }

    fn num_files(&self) -> usize {
        self.files.len()
    }
}

impl AnyDataset for RawAstDataset {
    fn num_comps(&self) -> usize {
        self.files.len() * (self.files.len() - 1) / 2
    }

    fn train(&self) -> AstDataset<Self> {
        AstDataset {
            base: self.self_ref.upgrade().expect("upgrade to allocated"),
            range: Range::from(0..(self.num_comps() as f32 * TRAIN_SPLIT) as usize),
        }
    }

    fn test(&self) -> AstDataset<Self> {
        AstDataset {
            base: self.self_ref.upgrade().expect("upgrade to allocated"),
            range: Range::from((self.num_comps() as f32 * TRAIN_SPLIT) as usize..self.num_comps()),
        }
    }

    fn file(&self, index: usize) -> LanguageBoundPath {
        LanguageBoundPath {
            path: self.files[index].clone(),
            language: self.language,
        }
    }

    fn pair(&self, index: usize) -> Option<&Pair> {
        self.dataset.pairs.get(&index)
    }

    fn num_files(&self) -> usize {
        self.files.len()
    }
}

#[derive(Debug, Clone)]
pub struct AstDataset<Base: AnyDataset + ?Sized> {
    base: Arc<Base>,
    range: Range<usize>,
}

impl<Base: AnyDataset + ?Sized> Dataset<AstDatasetSingle> for AstDataset<Base> {
    fn len(&self) -> usize {
        self.range.end - self.range.start
    }

    fn get(&self, index: usize) -> Option<AstDatasetSingle> {
        if !self.range.contains(&index) {
            return None;
        }

        let PairedIndices { i, j } =
            find_paired_indices_from_pair_index(index, self.base.num_files());

        Some(AstDatasetSingle {
            a: self.base.file(i),
            b: self.base.file(j),
            marks: self
                .base
                .pair(index)
                .cloned()
                .map(|p| p.marks)
                .unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct AstDatasetSingle {
    a: LanguageBoundPath,
    b: LanguageBoundPath,
    marks: Vec<Mark>,
}

#[derive(Debug, Clone, Copy)]
pub struct AstBatcher<B: Backend> {
    device: B::Device,
}

impl<B: Backend> AstBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        AstBatcher { device }
    }

    fn build_edges_and_features(
        &self,
        path: &Path,
        language: Language,
        index_offset: usize,
    ) -> Result<(Tensor<B, 2, Int>, Tensor<B, 2>), DataError> {
        let file_data = fs::read_to_string(path)?;

        Ok(match language {
            Language::C => {
                let tree = ast::c::CTree::try_from(file_data)?.symbol_tree()?;
                self.convert_tree_to_tensor(tree, index_offset, 0.0)
            }
            Language::Cpp => {
                let tree = ast::cpp::CppTree::try_from(file_data)?.symbol_tree()?;
                self.convert_tree_to_tensor(tree, index_offset, 1.0)
            }
            Language::Java => {
                let tree = ast::java::JavaTree::try_from(file_data)?.symbol_tree()?;
                self.convert_tree_to_tensor(tree, index_offset, 2.0)
            }
            Language::Python => {
                todo!()
            }
        })
    }

    fn convert_tree_to_tensor<T>(
        &self,
        tree: syntree::Tree<T, usize, usize>,
        index_offset: usize,
        language_index: f64,
    ) -> (Tensor<B, 2, burn::tensor::Int>, Tensor<B, 2>)
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
        let mut i = 0;

        for (event, node) in tree.walk_events() {
            match event {
                syntree::node::Event::Up => {
                    last_index = parent_index_stack
                        .pop()
                        .expect("pop always comes after push");
                    // An Up event will revisit a previously visited node! Skip the rest of this iteration
                    continue;
                }
                syntree::node::Event::Down => {
                    parent_index_stack.push(last_index);
                }
                syntree::node::Event::Next => {}
            }

            if let Some(parent_idx) = parent_index_stack.last() {
                edge_indices[*parent_idx].push(i);
            }

            let node_feature = {
                let node = node.value().into().to_device(&self.device);
                let language_identifier =
                    Tensor::<B, 1>::from_floats([language_index], &self.device);
                let padding =
                    Tensor::<B, 1>::full([MAX_FEATURES - node.dims()[0] - 1], 0.0, &self.device);
                Tensor::cat(vec![language_identifier, node, padding], 0)
            };
            features.push(node_feature);

            last_index = i;
            i += 1;
        }

        let mut paired_indices: Vec<Tensor<B, 1, burn::tensor::Int>> = vec![];

        for (from, list) in edge_indices.iter().enumerate() {
            for to in list {
                paired_indices.push(Tensor::from_ints([from, *to], &self.device))
            }
        }

        // Self-attention
        for i in Range::from(0..tree.len()).into_iter() {
            paired_indices.push(Tensor::from_ints([i, i], &self.device));
        }

        (
            Tensor::stack(paired_indices, 0)
                .add_scalar(index_offset as i64)
                .transpose(),
            Tensor::stack(features, 0),
        )
    }
}

pub const MAX_SPANS: usize = 20;
pub const MAX_NODES: usize = 1_000;
pub const MAX_FEATURES: usize = 200;
pub const MAX_EDGES: usize = MAX_NODES - 1;

impl<B: Backend> Batcher<AstDatasetSingle, AstBatch<B>> for AstBatcher<B> {
    fn batch(&self, items: Vec<AstDatasetSingle>) -> AstBatch<B> {
        // Read each item in the dataset, loading in all of the files in each batch
        // This is gonna take a ton of memory but oh well
        struct FeaturePair<B: Backend> {
            a: Tensor<B, 2>,
            b: Tensor<B, 2>,
        }
        let num_items = items.len();
        let (edges, features, spans): (Vec<_>, Vec<_>, Vec<_>) = itertools::multiunzip(
            items
                .into_iter()
                .map(|AstDatasetSingle { a, b, marks }| {
                    // Traverse the tree in the default order that syntree does, converting nodes to features
                    let (a_edge, a_feature) = self
                        .build_edges_and_features(a.path.as_path(), a.language, 0)
                        .expect("Valid tree build A");
                    let (b_edge, b_feature) = self
                        .build_edges_and_features(b.path.as_path(), b.language, 0)
                        .expect("Valid tree build B");

                    // let edge = Tensor::cat(vec![a_edge, b_edge], 0);
                    // let features = Tensor::cat(vec![a_feature, b_feature], 0);
                    let features = FeaturePair {
                        a: a_feature,
                        b: b_feature,
                    };

                    // Load spans
                    assert!(
                        marks.len() <= MAX_SPANS,
                        "Too many marks for files {a:?} {b:?}"
                    );

                    let num_marks = marks.len();

                    let spans = if num_marks > 0 {
                        let marks = marks.iter().map(|m| {
                            Tensor::<B, 1>::from_floats(
                                // s_1 s_2 e_1 e_2
                                [
                                    m.a.start as f32,
                                    m.b.start as f32,
                                    m.a.end as f32 + if m.a.start == m.a.end { 1.0 } else { 0.0 },
                                    m.b.end as f32 + if m.b.start == m.b.end { 1.0 } else { 0.0 },
                                ],
                                &self.device,
                            )
                        });

                        let marks = Tensor::stack(marks.collect(), 0);

                        let padding =
                            Tensor::<B, 2>::full([MAX_SPANS - num_marks, 4], -1.0, &self.device);

                        Tensor::cat(vec![marks, padding], 0)
                    } else {
                        Tensor::<B, 2>::full([MAX_SPANS, 4], -1.0, &self.device)
                    };

                    ([a_edge, b_edge], features, spans)
                })
                .collect::<Vec<(_, _, _)>>(),
        );

        // Find the maximum values for features and edges, padding each tensor to the correct size
        // let max_nodes = features
        //     .iter()
        //     .map(|t| t.a.dims()[0].max(t.b.dims()[0]))
        //     .max()
        //     .expect("some max feature value");

        // let max_edges = edges
        //     .iter()
        //     .map(|t| t.dims()[0])
        //     .max()
        //     .expect("some max edges value");

        // let edges = edges
        //     .into_iter()
        //     .map(|edge| {
        //         if edge.dims()[0] < max_edges {
        //             let difference = max_edges - edge.dims()[0];
        //             let padding = Tensor::<B, 2, Int>::full([difference, 2], -1, &self.device);

        //             Tensor::cat(vec![edge, padding], 0)
        //         } else {
        //             edge
        //         }
        //         .transpose()
        //     })
        //     .collect();

        // fn normalize_to_max_nodes_if_needed<B: Backend>(
        //     feature: Tensor<B, 2>,
        //     max: usize,
        //     device: &B::Device,
        // ) -> Tensor<B, 2> {
        //     if feature.dims()[0] < max {
        //         let difference = max - feature.dims()[0];
        //         let padding = Tensor::<B, 2>::full([difference, MAX_FEATURES], 0.0, device);

        //         Tensor::cat(vec![feature, padding], 0)
        //     } else {
        //         feature
        //     }
        // }

        let mut offset = 0_i64;
        let mut output = Vec::with_capacity(edges.len());
        for edge_tensor in edges.into_iter().flatten() {
            let next_offset = edge_tensor.dims()[1] as i64;
            output.push(edge_tensor.add_scalar(offset));
            offset += next_offset;
        }

        let edges = Tensor::cat(output, 1);

        let features: Vec<_> = features
            .into_iter()
            .flat_map(|feature| [feature.a, feature.b])
            .collect();

        assert_eq!(features.len(), num_items * 2, "Feature data lost!");

        // Create the graph index array
        // Each graph node will have an associated item in this tensor such that for some node N_i,
        // graph[N_i] = graph index it came from
        // To get the pair index, do N_i // 2
        let graph_feature_indices = features
            .iter()
            .enumerate()
            .map(|(i, feature)| {
                Tensor::<B, 1, Int>::full([feature.dims()[0]], i as u64, &self.device)
            })
            .collect::<Vec<_>>();

        let graph_feature_indices = Tensor::cat(graph_feature_indices, 0);

        assert!(
            graph_feature_indices
                .clone()
                .greater_elem(0)
                .any()
                .into_scalar(),
            "Graph feature indices all zero!"
        );

        AstBatch {
            edges,
            features: Tensor::cat(features, 0),
            spans: Tensor::stack(spans, 0),
            graph_feature_indices,
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

/// Represents a batch of ASTs for training
#[derive(Debug, Clone)]
pub struct AstBatch<B: Backend> {
    /// [E, 2]
    pub edges: Tensor<B, 2, burn::tensor::Int>,
    /// [N, F]
    pub features: Tensor<B, 2>,
    /// [batch_size, MAX_SPANS, 4]
    pub spans: Tensor<B, 3>,
    pub graph_feature_indices: Tensor<B, 1, Int>,
}
