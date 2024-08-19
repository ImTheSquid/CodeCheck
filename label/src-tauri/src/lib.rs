#![feature(iterator_try_collect)]

use ast::{guess_language_from_path, Language, TreeParseError};
use std::fs::{self, read_to_string};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use tauri::Manager;
use util::{Dataset, Mark, Pair};
use walkdir::WalkDir;

const DATASET_FILE_NAME: &'static str = "dataset.json";

macro_rules! str_error {
    ($t: ty) => {
        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                serializer.serialize_str(self.to_string().as_ref())
            }
        }
    };
}

#[derive(Debug)]
struct CurrentDataset {
    path: PathBuf,
    data: Dataset,
    items: Vec<PathBuf>,
}

#[derive(Debug, Default)]
struct AppState {
    current_dataset: RwLock<Option<CurrentDataset>>,
}

#[derive(Debug, thiserror::Error)]
enum DirectoryValidationError {
    #[error("Given path is not a directory")]
    InvalidPath,
    #[error(transparent)]
    AstError(#[from] TreeParseError),
    #[error("Insufficient data points")]
    InsufficientData,
    #[error("Multiple languages found")]
    MultipleLanguages,
    #[error("Dataset loading error: {0}")]
    DatasetLoadingError(#[from] std::io::Error),
    #[error("Bad dataset format: {0}")]
    BadDatasetFormat(#[from] serde_json::Error),
}

str_error!(DirectoryValidationError);

/// Ensures that all files in the directory are of the same (supported) file type and ignores everything else.
/// If multiple supported file types are detected, this is an error
#[tauri::command]
fn validate_directory(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), DirectoryValidationError> {
    // Make sure this is actually a directory
    let path = Path::new(&path);
    if !path.is_dir() {
        return Err(DirectoryValidationError::InvalidPath);
    }

    // Get a list of all files recursively
    // Are all of the source code files of the same type?
    let mut file_paths = WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter_map(|e| match guess_language_from_path(e.path()) {
            Err(_) => None,
            Ok(l) => Some((
                e.path()
                    .strip_prefix(&path)
                    .expect("path to be child")
                    .to_owned(),
                l,
            )),
        })
        .collect::<Vec<_>>();
    file_paths.sort_unstable_by_key(|(a, b)| a.clone());
    let (file_paths, detected_languages): (Vec<_>, Vec<_>) = file_paths.into_iter().unzip();

    // Check that there is at least two files
    if detected_languages.len() <= 1 {
        return Err(DirectoryValidationError::InsufficientData);
    }

    if !detected_languages
        .iter()
        .all(|l| *l == detected_languages[0])
    {
        return Err(DirectoryValidationError::MultipleLanguages);
    }

    // Set this directory as the current state directory, creating a data file for it if it doesn't exist
    let load = if path.join("dataset.json").exists() {
        let data = fs::read_to_string(path.join(DATASET_FILE_NAME))?;
        let data = serde_json::from_str(&data)?;
        data
    } else {
        Dataset {
            pairs: Default::default(),
        }
    };

    let _ = state
        .current_dataset
        .write()
        .unwrap()
        .insert(CurrentDataset {
            data: load,
            path: path.to_path_buf(),
            items: file_paths,
        });

    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct Item {
    path: String,
    contents: String,
}

#[derive(Debug, serde::Serialize)]
struct PairData {
    a: Item,
    b: Item,
    marks: Vec<Mark>,
    lang: String,
}

#[derive(Debug, thiserror::Error)]
enum DatasetError {
    #[error("No dataset loaded")]
    NoDataset,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Tree(#[from] TreeParseError),
}

str_error!(DatasetError);

#[tauri::command]
fn get_overview(state: tauri::State<'_, AppState>) -> Result<Vec<Option<usize>>, DatasetError> {
    let current = state.current_dataset.read().unwrap();

    let current = current.as_ref().ok_or(DatasetError::NoDataset)?;
    let mut status = vec![None; current.items.len() * (current.items.len() - 1) / 2];

    for (&k, v) in &current.data.pairs {
        status[k] = Some(v.marks.len());
    }

    Ok(status)
}

struct PairedIndices {
    i: usize,
    j: usize,
}

fn find_paired_indices_from_pair_index(mut k: usize, n: usize) -> PairedIndices {
    let mut remaining = n - 1;
    let mut i = 0;
    while k >= remaining {
        i += 1;
        k -= remaining;
        remaining -= 1;
    }

    PairedIndices { i, j: i + k + 1 }
}
// Oops this doesn't work, it overflows the stack
// TCO results in an infinite hang using `tailcall`, and `become` isn't in Rust yet
// // YOU CAN DO BETTER! THANKS JOHN FOR O(log(n))
// // Special binary search implementation that tries to guess values of k at certain i values,
// // homing in on correct one
// fn binary_search_index(k: usize, n: usize, l: usize, r: usize) -> PairedIndices {
//     let x = (r - l) / 2 + l;
//     let val = n * (n - 1) / 2 - x * (x - 1) / 2;
//     if val <= k {
//         if k - val < x - 1 {
//             return PairedIndices { i: n - x, j: val };
//         }
//         return binary_search_index(k, n, l, x + 1);
//     }
//     binary_search_index(k, n, x + 1, r)
// }

// fn find_paired_indices_from_pair_index(k: usize, n: usize) -> PairedIndices {
//     let PairedIndices { i, j } = binary_search_index(k, n, 0, n);
//     PairedIndices {
//         i,
//         j: k - j + i + 1,
//     }
// }

#[tauri::command]
fn load_pair(
    pair_index: usize,
    state: tauri::State<'_, AppState>,
) -> Result<PairData, DatasetError> {
    let (path, is_cached) = {
        let dataset = state.current_dataset.read().unwrap();
        let dataset = dataset.as_ref().ok_or(DatasetError::NoDataset)?;
        (
            dataset.path.clone(),
            dataset.data.pairs.get(&pair_index).is_some(),
        )
    };
    let (marks, a, b) = if is_cached {
        let dataset = state.current_dataset.read().unwrap();
        let dataset = dataset.as_ref().ok_or(DatasetError::NoDataset)?;
        let d = dataset.data.pairs.get(&pair_index).unwrap();
        (d.marks.clone(), PathBuf::from(&d.a), PathBuf::from(&d.b))
    } else {
        let mut dataset = state.current_dataset.write().unwrap();
        let dataset = dataset.as_mut().ok_or(DatasetError::NoDataset)?;
        let PairedIndices { i, j } =
            find_paired_indices_from_pair_index(pair_index, dataset.items.len());
        dataset.data.pairs.insert(
            pair_index,
            Pair {
                a: dataset.items[i].to_string_lossy().into_owned(),
                b: dataset.items[j].to_string_lossy().into_owned(),
                marks: vec![],
            },
        );

        (vec![], dataset.items[i].clone(), dataset.items[j].clone())
    };

    let a_full = path.join(&a);
    let b_full = path.join(&b);

    let lang = match guess_language_from_path(&a_full)? {
        Language::C => "c",
        Language::Cpp => "cpp",
        Language::Java => "java",
        Language::Python => "python",
    }
    .to_string();

    let a_contents = read_to_string(&a_full)?;
    let b_contents = read_to_string(&b_full)?;

    Ok(PairData {
        a: Item {
            path: a.to_string_lossy().into_owned(),
            contents: a_contents,
        },
        b: Item {
            path: b.to_string_lossy().into_owned(),
            contents: b_contents,
        },
        marks,
        lang,
    })
}

#[tauri::command]
fn set_spans(
    pair_index: usize,
    marks: Vec<Mark>,
    state: tauri::State<'_, AppState>,
) -> Result<(), DatasetError> {
    let mut state = state.current_dataset.write().unwrap();
    let pairs = &mut state.as_mut().ok_or(DatasetError::NoDataset)?.data.pairs;
    let item = pairs.get_mut(&pair_index).expect("some value to be in map");
    item.marks = marks;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            validate_directory,
            load_pair,
            set_spans,
            get_overview
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| match event {
            tauri::RunEvent::Exit => {
                let state = app.state::<AppState>();
                let state = state.current_dataset.read().unwrap();
                let Some(dataset) = state.as_ref() else {
                    return;
                };

                let path = dataset.path.join(DATASET_FILE_NAME);
                let mut f = fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(path)
                    .unwrap();
                f.write_all(serde_json::to_string(&dataset.data).unwrap().as_bytes())
                    .unwrap();
            }
            _ => {}
        });
}
