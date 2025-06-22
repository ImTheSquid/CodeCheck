use ast::{guess_language_from_path, prune_tree, Language, SyntaxTree};
use burn::{
    prelude::Backend,
    tensor::{Int, Tensor},
};
use core::range::Range;
use ndarray::{array, Array1, Array2, Axis};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Weak},
};
use util::{
    arr_vec_to_view, find_paired_indices_from_pair_index, Dataset as MarkDataset, DatasetError,
    Pair,
};
use walkdir::WalkDir;

// const TRAIN_SPLIT: f32 = 0.8;

/// AST dataset as it exists on the filesystem
pub struct RawAstDataset {
    language: Language,
    files: Vec<PathBuf>,
    dataset: MarkDataset,
    // self_ref: Weak<Self>,
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
            // self_ref: Default::default(),
        })
    }
}

/// A path with a specific language bound to it for parsing
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

// pub trait AnyDataset: Send + Sync {
//     fn num_comps(&self) -> usize;

//     fn train(&self) -> AstDataset<Self>;

//     fn test(&self) -> AstDataset<Self>;

//     fn file(&self, index: usize) -> LanguageBoundPath;

//     fn pair(&self, index: usize) -> Option<&Pair>;

//     fn num_files(&self) -> usize;
// }

#[derive(Debug, Default, Clone)]
pub struct CollatedAstDataset {
    files: Vec<LanguageBoundPath>,
    dataset: HashMap<(usize, usize), Pair>,
    self_ref: Weak<Self>,
}

pub struct CompilationOutput {
    pub features: Vec<Array2<f64>>,
    pub edges: Vec<Array2<usize>>,
    pub feature_spans: Vec<Array2<usize>>,
    pub dataset: HashMap<(usize, usize), Pair>,
}

impl CollatedAstDataset {
    pub fn to_arc(mut self) -> Arc<Self> {
        Arc::new_cyclic(|d| {
            self.self_ref = d.clone();
            self
        })
    }

    pub fn include(&mut self, dataset: RawAstDataset) {
        // Adding new datasets to the existing set means all their pair indices need to be recalculated, from scratch
        // Instead, I will reverse-lookup the indices
        // Very inefficient but only used during training
        let n = dataset.files.len();
        self.dataset.extend(
            dataset
                .dataset
                .pairs
                .into_iter()
                .map(|(k, v)| (find_paired_indices_from_pair_index(k, n), v)),
        );

        self.files
            .extend(dataset.files.into_iter().map(|p| LanguageBoundPath {
                language: dataset.language,
                path: p,
            }));
    }

    pub fn compile(self) -> Result<CompilationOutput, DataError> {
        #[allow(clippy::type_complexity)]
        let r: Result<Vec<(Array2<f64>, (Array2<usize>, Array2<usize>))>, DataError> = self
            .files
            .into_par_iter()
            .map(|f| {
                let bt = build_edges_and_features(&f.path, f.language)?;
                Ok((bt.features, (bt.edges, bt.feature_spans)))
            })
            .collect();
        let r = r?;
        let (features, edges_and_feature_spans): (_, Vec<_>) = r.into_iter().unzip();
        let (edges, feature_spans): (Vec<Array2<usize>>, Vec<Array2<usize>>) =
            edges_and_feature_spans.into_iter().unzip();
        Ok(CompilationOutput {
            features,
            edges,
            feature_spans,
            dataset: self.dataset,
        })
    }
}

fn build_edges_and_features(path: &Path, language: Language) -> Result<BatchedTensors, DataError> {
    let file_data = fs::read_to_string(path)?;
    // Add an extra newline to prevent parsing errors if it's Python
    let file_data = if matches!(language, Language::Python) {
        format!("{file_data}\n")
    } else {
        file_data
    };
    let char_map = resolve_line_numbers_from_character_positions(&file_data);
    // let num_lines = file_data.lines().count();

    let TensorBuildData {
        edges,
        features,
        feature_spans,
    } = match language {
        Language::C => {
            let mut tree = ast::c::CTree::try_from(file_data)?.symbol_tree()?;
            prune_tree(&mut tree);
            convert_tree_to_tensor(tree, language, char_map)?
        }
        Language::Cpp => {
            let mut tree = ast::cpp::CppTree::try_from(file_data)?.symbol_tree()?;
            prune_tree(&mut tree);
            convert_tree_to_tensor(tree, language, char_map)?
        }
        Language::Java => {
            let mut tree = ast::java::JavaTree::try_from(file_data)?.symbol_tree()?;
            prune_tree(&mut tree);
            convert_tree_to_tensor(tree, language, char_map)?
        }
        Language::Python => {
            let mut tree = ast::python::PythonTree::try_from(file_data)?.symbol_tree()?;
            prune_tree(&mut tree);
            convert_tree_to_tensor(tree, language, char_map)?
        }
    };

    Ok(BatchedTensors {
        edges,
        features,
        feature_spans,
        // num_lines,
    })
}

/// Converts a character index to its line number
fn resolve_line_numbers_from_character_positions(file_data: &str) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    let mut current_line = 0_usize;
    for (i, c) in file_data.char_indices() {
        map.insert(i, current_line);
        if c == '\n' {
            current_line += 1;
        }
    }
    // The final character is a bit tricky and can sometimes exist, add just in case
    map.insert(file_data.len(), current_line);
    map
}

fn convert_tree_to_tensor<T>(
    tree: syntree::Tree<T, usize, usize>,
    language: Language,
    character_map: HashMap<usize, usize>,
) -> Result<TensorBuildData, DataError>
where
    T: Copy + Into<Array1<f64>>,
{
    let mut edge_indices = Range::from(0..tree.len())
        .into_iter()
        .map(|_| vec![])
        .collect::<Vec<_>>();
    let mut features = Vec::with_capacity(tree.len());
    let mut spans = Vec::with_capacity(tree.len());
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
            let node: Array1<f64> = node.value().into();
            let (leading, trailing) = language.padding();
            let leading_padding = Array1::from_shape_simple_fn([leading], || 0.0);
            let trailing_padding = Array1::from_shape_simple_fn([trailing], || 0.0);
            let padding = Array1::from_shape_simple_fn([MAX_FEATURES - node.dim() - 1], || 0.0);
            ndarray::concatenate![Axis(0), leading_padding, node, padding, trailing_padding]
        };
        features.push(node_feature);
        let span = node.span();
        spans.push(array![
            *character_map
                .get(&span.start)
                .ok_or(DataError::InvalidCharacterMapConstruction {
                    character_index: span.start,
                    designator: InvalidCharacterMapSpanPartDesignator::Start
                })?,
            *character_map
                .get(&span.end)
                .ok_or(DataError::InvalidCharacterMapConstruction {
                    character_index: span.end,
                    designator: InvalidCharacterMapSpanPartDesignator::End
                })?
        ]);

        last_index = i;
        i += 1;
    }

    let mut paired_indices: Vec<Array1<usize>> = vec![];
    // let mut hash_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for (from, list) in edge_indices.iter().enumerate() {
        for &to in list {
            paired_indices.push(array![from, to]);
            // if let Some(list) = hash_map.get_mut(&from) {
            //     list.push(to);
            // } else {
            //     hash_map.insert(from, vec![to]);
            // }
        }
    }

    // // Self-attention
    // for i in Range::from(0..tree.len()).into_iter() {
    //     paired_indices.push(Tensor::from_ints([i, i], &self.device));
    // }

    Ok(TensorBuildData {
        edges: ndarray::stack(Axis(0), arr_vec_to_view!(paired_indices))
            .expect("valid stack")
            .t()
            .to_owned(),
        // edges_hash: hash_map,
        features: ndarray::stack(Axis(0), arr_vec_to_view!(features)).expect("valid stack"),
        feature_spans: ndarray::stack(Axis(0), arr_vec_to_view!(spans)).expect("valid stack"),
    })
}

// impl AnyDataset for CollatedAstDataset {
//     fn num_comps(&self) -> usize {
//         self.files.len()
//     }

//     fn train(&self) -> AstDataset<Self> {
//         AstDataset {
//             base: self.self_ref.upgrade().expect("upgrade to allocted"),
//             range: Range::from(0..(self.num_comps() as f32 * TRAIN_SPLIT) as usize),
//         }
//     }

//     fn test(&self) -> AstDataset<Self> {
//         AstDataset {
//             base: self.self_ref.upgrade().expect("upgrade to allocted"),
//             range: Range::from((self.num_comps() as f32 * TRAIN_SPLIT) as usize..self.num_comps()),
//         }
//     }

//     fn file(&self, index: usize) -> LanguageBoundPath {
//         self.files[index].clone()
//     }

//     fn pair(&self, index: usize) -> Option<&Pair> {
//         // self.dataset.get(&index)
//         unimplemented!()
//     }

//     fn num_files(&self) -> usize {
//         self.files.len()
//     }
// }

// impl AnyDataset for RawAstDataset {
//     fn num_comps(&self) -> usize {
//         self.files.len()
//     }

//     fn train(&self) -> AstDataset<Self> {
//         AstDataset {
//             base: self.self_ref.upgrade().expect("upgrade to allocated"),
//             range: Range::from(0..(self.num_comps() as f32 * TRAIN_SPLIT) as usize),
//         }
//     }

//     fn test(&self) -> AstDataset<Self> {
//         AstDataset {
//             base: self.self_ref.upgrade().expect("upgrade to allocated"),
//             range: Range::from((self.num_comps() as f32 * TRAIN_SPLIT) as usize..self.num_comps()),
//         }
//     }

//     fn file(&self, index: usize) -> LanguageBoundPath {
//         LanguageBoundPath {
//             path: self.files[index].clone(),
//             language: self.language,
//         }
//     }

//     fn pair(&self, index: usize) -> Option<&Pair> {
//         self.dataset.pairs.get(&index)
//     }

//     fn num_files(&self) -> usize {
//         self.files.len()
//     }
// }

// #[derive(Debug, Clone)]
// pub struct AstDataset<Base: AnyDataset + ?Sized> {
//     base: Arc<Base>,
//     range: Range<usize>,
// }

// impl<Base: AnyDataset + ?Sized> Dataset<AstDatasetSingle> for AstDataset<Base> {
//     fn len(&self) -> usize {
//         self.range.end - self.range.start
//     }

//     fn get(&self, index: usize) -> Option<AstDatasetSingle> {
//         if !self.range.contains(&index) {
//             return None;
//         }

//         let PairedIndices { i, j } =
//             find_paired_indices_from_pair_index(index, self.base.num_files());

//         Some(AstDatasetSingle {
//             a: self.base.file(i),
//             b: self.base.file(j),
//             marks: self
//                 .base
//                 .pair(index)
//                 .cloned()
//                 .map(|p| p.marks)
//                 .unwrap_or_default(),
//         })
//     }
// }

// #[derive(Debug, Clone)]
// pub struct AstDatasetSingle {
//     a: LanguageBoundPath,
//     b: LanguageBoundPath,
//     marks: Vec<Mark>,
// }

// #[derive(Debug, Clone, Copy)]
// pub struct AstBuilder<B: Backend> {
//     device: B::Device,
// }

struct BatchedTensors {
    edges: Array2<usize>,
    // edges_hash: HashMap<usize, Vec<usize>>,
    features: Array2<f64>,
    feature_spans: Array2<usize>,
    // num_lines: usize,
}

struct TensorBuildData {
    edges: Array2<usize>,
    // edges_hash: HashMap<usize, Vec<usize>>,
    features: Array2<f64>,
    feature_spans: Array2<usize>,
}

// impl<B: Backend> AstBuilder<B> {
//     pub fn new(device: B::Device) -> Self {
//         AstBuilder { device }
//     }
// }

pub const MAX_SPANS: usize = 20;
pub const MAX_NODES: usize = 1_000;
pub const MAX_FEATURES: usize = 200;
pub const MAX_EDGES: usize = MAX_NODES - 1;

// impl<B: Backend> Batcher<B, AstDatasetSingle, AstBatch<B>> for AstBuilder<B> {
//     fn batch(&self, items: Vec<AstDatasetSingle>) -> AstBatch<B> {
//         // Read each item in the dataset, loading in all of the files in each batch
//         // This is gonna take a ton of memory but oh well
//         struct FeaturePair<B: Backend> {
//             a: Tensor<B, 2>,
//             b: Tensor<B, 2>,
//         }
//         let num_items = items.len();
//         let (edges, features, spans): (Vec<_>, Vec<_>, Vec<_>) = itertools::multiunzip(
//             items
//                 .into_iter()
//                 .map(|AstDatasetSingle { a, b, marks }| {
//                     // Traverse the tree in the default order that syntree does, converting nodes to features
//                     let BatchedTensors {
//                         edges: a_edge,
//                         edges_hash: a_edges_hash,
//                         features: a_feature,
//                         num_lines: a_lines,
//                     } = self
//                         .build_edges_and_features(a.path.as_path(), a.language)
//                         .expect("Valid tree build A");

//                     let BatchedTensors {
//                         edges: b_edge,
//                         edges_hash: b_edges_hash,
//                         features: b_feature,
//                         num_lines: b_lines,
//                     } = self
//                         .build_edges_and_features(b.path.as_path(), b.language)
//                         .expect("Valid tree build B");

//                     let a_lines = a_lines as f32;
//                     let b_lines = b_lines as f32;

//                     // let edge = Tensor::cat(vec![a_edge, b_edge], 0);
//                     // let features = Tensor::cat(vec![a_feature, b_feature], 0);
//                     let features = FeaturePair {
//                         a: a_feature,
//                         b: b_feature,
//                     };

//                     // Load spans
//                     assert!(
//                         marks.len() <= MAX_SPANS,
//                         "Too many marks for files {a:?} {b:?}"
//                     );

//                     let num_marks = marks.len();

//                     let spans = if num_marks > 0 {
//                         let marks = marks.iter().map(|m| {
//                             Tensor::<B, 1>::from_floats(
//                                 // s_1 s_2 e_1 e_2
//                                 [
//                                     m.a.start as f32 / a_lines,
//                                     m.b.start as f32 / b_lines,
//                                     (m.a.end as f32 + if m.a.start == m.a.end { 1.0 } else { 0.0 })
//                                         / a_lines,
//                                     (m.b.end as f32 + if m.b.start == m.b.end { 1.0 } else { 0.0 })
//                                         / b_lines,
//                                 ],
//                                 &self.device,
//                             )
//                         });

//                         let marks = Tensor::stack(marks.collect(), 0);

//                         let padding =
//                             Tensor::<B, 2>::full([MAX_SPANS - num_marks, 4], -1.0, &self.device);

//                         Tensor::cat(vec![marks, padding], 0)
//                     } else {
//                         Tensor::<B, 2>::full([MAX_SPANS, 4], -1.0, &self.device)
//                     };

//                     ([a_edge, b_edge], features, spans)
//                 })
//                 .collect::<Vec<(_, _, _)>>(),
//         );

//         // Find the maximum values for features and edges, padding each tensor to the correct size
//         // let max_nodes = features
//         //     .iter()
//         //     .map(|t| t.a.dims()[0].max(t.b.dims()[0]))
//         //     .max()
//         //     .expect("some max feature value");

//         // let max_edges = edges
//         //     .iter()
//         //     .map(|t| t.dims()[0])
//         //     .max()
//         //     .expect("some max edges value");

//         // let edges = edges
//         //     .into_iter()
//         //     .map(|edge| {
//         //         if edge.dims()[0] < max_edges {
//         //             let difference = max_edges - edge.dims()[0];
//         //             let padding = Tensor::<B, 2, Int>::full([difference, 2], -1, &self.device);

//         //             Tensor::cat(vec![edge, padding], 0)
//         //         } else {
//         //             edge
//         //         }
//         //         .transpose()
//         //     })
//         //     .collect();

//         // fn normalize_to_max_nodes_if_needed<B: Backend>(
//         //     feature: Tensor<B, 2>,
//         //     max: usize,
//         //     device: &B::Device,
//         // ) -> Tensor<B, 2> {
//         //     if feature.dims()[0] < max {
//         //         let difference = max - feature.dims()[0];
//         //         let padding = Tensor::<B, 2>::full([difference, MAX_FEATURES], 0.0, device);

//         //         Tensor::cat(vec![feature, padding], 0)
//         //     } else {
//         //         feature
//         //     }
//         // }

//         let mut offset = 0_i64;
//         let mut output = Vec::with_capacity(edges.len());
//         for edge_tensor in edges.into_iter().flatten() {
//             let next_offset = edge_tensor.dims()[1] as i64;
//             output.push(edge_tensor.add_scalar(offset));
//             offset += next_offset;
//         }

//         let edges = Tensor::cat(output, 1);

//         let features: Vec<_> = features
//             .into_iter()
//             .flat_map(|feature| [feature.a, feature.b])
//             .collect();

//         assert_eq!(features.len(), num_items * 2, "Feature data lost!");

//         // Create the graph index array
//         // Each graph node will have an associated item in this tensor such that for some node N_i,
//         // graph[N_i] = graph index it came from
//         // To get the pair index, do N_i // 2
//         let graph_feature_indices = features
//             .iter()
//             .enumerate()
//             .map(|(i, feature)| {
//                 Tensor::<B, 1, Int>::full([feature.dims()[0]], i as u64, &self.device)
//             })
//             .collect::<Vec<_>>();

//         let graph_feature_indices = Tensor::cat(graph_feature_indices, 0);

//         assert!(
//             graph_feature_indices
//                 .clone()
//                 .greater_elem(0)
//                 .any()
//                 .into_scalar(),
//             "Graph feature indices all zero!"
//         );

//         AstBatch {
//             edges,
//             features: Tensor::cat(features, 0),
//             spans: Tensor::stack(spans, 0),
//             graph_feature_indices,
//         }
//     }
// }

#[derive(Debug)]
pub enum InvalidCharacterMapSpanPartDesignator {
    Start,
    End,
}

impl Display for InvalidCharacterMapSpanPartDesignator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Start => "START",
            Self::End => "END",
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Tree(#[from] ast::TreeParseError),
    /// The character map doesn't match up with the tree span
    #[error(
        "The character map doesn't match up with the tree span! {designator} {character_index}"
    )]
    InvalidCharacterMapConstruction {
        character_index: usize,
        designator: InvalidCharacterMapSpanPartDesignator,
    },
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
