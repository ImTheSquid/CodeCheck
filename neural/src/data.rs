use ast::{guess_language_from_path, prune_tree, Language, SyntaxTree};
use pythonize::{depythonize, pythonize};

use core::range::Range;
use ndarray::{arr2, array, Array1, Array2, Axis};
use ndarray_npy::NpzReader;
use pyo3::{
    exceptions::{PyFileNotFoundError, PyRuntimeError},
    pyclass, pymethods, pymodule,
    types::{PyAnyMethods, PyDict, PyDictMethods, PyList, PyTuple},
    Bound, IntoPyObject, PyAny, PyRef, PyResult, Python,
};
use rand::distr::{Alphanumeric, SampleString};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    env::temp_dir,
    fmt::Display,
    fs::{self, create_dir_all, remove_dir_all, File},
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, Weak},
};
use util::{arr_vec_to_view, Dataset as MarkDataset, DatasetError};
use walkdir::WalkDir;

use crate::KeyData;

// const TRAIN_SPLIT: f32 = 0.8;

/// AST dataset as it exists on the filesystem
pub struct RawAstDataset {
    files: Vec<LanguageBoundPath>,
    dataset: MarkDataset,
    base: PathBuf,
}

impl TryFrom<&Path> for RawAstDataset {
    type Error = DatasetError;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let entries: Vec<_> = WalkDir::new(value)
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.ok()?;

                if entry.path().extension()?.eq_ignore_ascii_case("json") {
                    return None;
                }

                let lang = guess_language_from_path(entry.path()).ok()?;

                Some(LanguageBoundPath {
                    path: entry.path().to_path_buf(),
                    language: lang,
                })
            })
            .collect();

        if entries.is_empty() {
            return Err(DatasetError::NoFiles);
        }

        let dataset: MarkDataset =
            serde_json::from_str(&fs::read_to_string(value.join("dataset.json"))?)?;

        Ok(Self {
            files: entries,
            dataset,
            base: value.to_path_buf(),
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

#[derive(Debug, Default, Clone)]
pub struct CollatedAstDataset {
    files: Vec<LanguageBoundPath>,
    dataset: Vec<KeyData>,
    self_ref: Weak<Self>,
}

pub struct CompilationOutput {
    pub features: Vec<Array2<f64>>,
    pub edges: Vec<Array2<usize>>,
    pub feature_spans: Vec<Array2<usize>>,
    pub dataset: Vec<KeyData>,
    pub languages: Vec<usize>,
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
        let file_lookup = dataset
            .files
            .iter()
            .enumerate()
            .map(|(i, f)| {
                (
                    f.path
                        .strip_prefix(&dataset.base)
                        .expect("base on dataset")
                        .to_path_buf(),
                    i,
                )
            })
            .collect::<HashMap<_, _>>();

        let offset = self.dataset.len();

        self.dataset
            .extend(dataset.dataset.pairs.into_iter().map(|p| KeyData {
                a: file_lookup[&PathBuf::from_str(&p.a).expect("a path")] + offset,
                b: file_lookup[&PathBuf::from_str(&p.b).expect("a path")] + offset,
                marks: p.marks,
            }));

        self.files.extend(dataset.files);
    }

    pub fn compile_to_tmpdir(self) -> Result<TmpDirDataset, DataError> {
        let tmpdir = temp_dir().join(format!(
            "dataset-{}",
            Alphanumeric.sample_string(&mut rand::rng(), 10)
        ));
        create_dir_all(&tmpdir)?;
        let r: Result<Vec<usize>, DataError> = self
            .files
            .into_par_iter()
            .enumerate()
            .map(|(index, f)| {
                let bt = build_edges_and_features(&f.path, f.language)?;

                TmpDirDataset::write(&tmpdir, index, &bt.features, &bt.edges, &bt.feature_spans);

                Ok(f.language.id())
            })
            .collect();

        let languages = r?;

        Ok(TmpDirDataset {
            keys: self.dataset,
            languages: ndarray::Array1::from_vec(languages),
            dir: tmpdir,
        })
    }

    pub fn compile(self) -> Result<CompilationOutput, DataError> {
        use itertools::MultiUnzip;
        #[allow(clippy::type_complexity)]
        let r: Result<Vec<(Array2<f64>, Array2<usize>, Array2<usize>, usize)>, DataError> = self
            .files
            .into_par_iter()
            .map(|f| {
                let bt = build_edges_and_features(&f.path, f.language)?;
                Ok((bt.features, bt.edges, bt.feature_spans, f.language.id()))
            })
            .collect();
        let r = r?;
        let (features, edges, feature_spans, languages) = r.into_iter().multiunzip();

        Ok(CompilationOutput {
            features,
            edges,
            feature_spans,
            dataset: self.dataset,
            languages,
        })
    }
}

#[pyclass(frozen)]
pub struct TmpDirDataset {
    keys: Vec<KeyData>,
    languages: ndarray::Array1<usize>,
    dir: PathBuf,
}

#[pymodule]
pub mod rust_data {
    #[pymodule_export]
    use super::{TmpDirDataset, TransferrableLoader};
}

impl TmpDirDataset {
    fn write(
        dir: &Path,
        index: usize,
        features: &ndarray::Array2<f64>,
        edges: &ndarray::Array2<usize>,
        feature_spans: &ndarray::Array2<usize>,
    ) {
        let f = dir.join(format!("{index}.npz"));
        let f = File::create(f).expect("file creation");
        let mut w = ndarray_npy::NpzWriter::new(f);
        w.add_array("features", &features.mapv(|v| v as f32))
            .expect("add features");
        w.add_array("edges", &edges.mapv(|e| e as u64))
            .expect("add edges");
        w.add_array("feature_spans", &feature_spans.mapv(|fs| fs as u64))
            .expect("add feature spans");
        w.finish().expect("finish writing");
    }

    pub fn new(compiled: CompilationOutput, dir: PathBuf) -> Self {
        for i in 0..compiled.features.len() {
            Self::write(
                &dir,
                i,
                &compiled.features[i],
                &compiled.edges[i],
                &compiled.feature_spans[i],
            );
        }
        Self {
            keys: compiled.dataset,
            languages: ndarray::Array1::from_vec(compiled.languages),
            dir,
        }
    }
}

impl Drop for TmpDirDataset {
    fn drop(&mut self) {
        let _ = remove_dir_all(&self.dir);
    }
}

#[pymethods]
impl TmpDirDataset {
    #[getter]
    pub fn keys<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let keys = PyList::new(
            py,
            self.keys.iter().map(|k| {
                let marks = k
                    .marks
                    .iter()
                    .map(|m| {
                        [
                            m.a.start as f64,
                            m.b.start as f64,
                            m.a.end as f64,
                            m.b.end as f64,
                        ]
                    })
                    .collect::<Vec<_>>();

                let marks = arr2(&marks);

                let marks = numpy::PyArray2::from_array(py, &marks);

                ((k.a, k.b), marks)
            }),
        )?;

        PyDict::from_sequence(&keys)
    }

    #[getter]
    pub fn languages<'a>(&self, py: Python<'a>) -> Bound<'a, numpy::PyArray1<usize>> {
        numpy::PyArray1::from_array(py, &self.languages)
    }

    pub fn __len__(&self) -> usize {
        self.languages.len()
    }

    pub fn get<'a>(&self, py: Python<'a>, i: usize) -> PyResult<Bound<'a, PyDict>> {
        self.loader().get(py, i)
    }

    #[getter]
    pub fn loader(&self) -> TransferrableLoader {
        TransferrableLoader {
            dir: self.dir.clone(),
            len: self.__len__(),
        }
    }
}

#[pyclass(frozen, module = "rust_data")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TransferrableLoader {
    dir: PathBuf,
    len: usize,
}

#[pymethods]
impl TransferrableLoader {
    pub fn __len__(&self) -> usize {
        self.len
    }

    pub fn get<'a>(&self, py: Python<'a>, i: usize) -> PyResult<Bound<'a, PyDict>> {
        let path = self.dir.to_path_buf().join(format!("{i}.npz"));
        if !path.exists() {
            return Err(PyFileNotFoundError::new_err(format!(
                "File not found: {}",
                path.display()
            )));
        }

        let f = File::open(path)?;
        let mut r = NpzReader::new(f).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let f: Array2<f32> = r
            .by_name("features")
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let f = numpy::PyArray2::from_array(py, &f);
        let e: Array2<u64> = r
            .by_name("edges")
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let e = numpy::PyArray2::from_array(py, &e);
        let fs: Array2<u64> = r
            .by_name("feature_spans")
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let fs = numpy::PyArray2::from_array(py, &fs);

        let d = PyDict::new(py);
        d.set_item("features", f)?;
        d.set_item("edge_index", e)?;
        d.set_item("feature_spans", fs)?;

        Ok(d)
    }

    #[staticmethod]
    fn from_encoded(serialized: &Bound<'_, PyAny>) -> PyResult<Self> {
        depythonize(serialized).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    fn __reduce__<'a>(slf: PyRef<'a, Self>, py: Python<'a>) -> PyResult<Bound<'a, PyTuple>> {
        let serialized =
            pythonize(py, &*slf).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

        (
            slf.into_pyobject(py)?.getattr("from_encoded")?,
            PyTuple::new(py, [serialized])?,
        )
            .into_pyobject(py)
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
            ndarray::concatenate![Axis(0), leading_padding, node, trailing_padding]
        };
        features.push(node_feature);
        let span = node.span();
        // Spans are not inclusive
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

type BatchedTensors = TensorBuildData;

struct TensorBuildData {
    edges: Array2<usize>,
    // edges_hash: HashMap<usize, Vec<usize>>,
    features: Array2<f64>,
    feature_spans: Array2<usize>,
}

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
