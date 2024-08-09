#![feature(iterator_try_collect)]

use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use ast::{guess_language_from_path, Language, TreeParseError};
use util::{Dataset, Pair};
use std::sync::RwLock;
use std::fs;

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
    }
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
fn validate_directory(path: String, state: tauri::State<'_, AppState>) -> Result<(), DirectoryValidationError> {
    // Make sure this is actually a directory
    let path = Path::new(&path);
    if !path.is_dir() {
        return Err(DirectoryValidationError::InvalidPath);
    }

    // Get a list of all files recursively
    let mut file_paths = WalkDir::new(&path).into_iter().filter_map(|e| e.ok()).map(|e| e.path().strip_prefix(&path).expect("path to be child").to_owned()).collect::<Vec<_>>();
    file_paths.sort();

    // Are all of the source code files of the same type?
    let detected_languages = file_paths.iter().filter_map(|p| guess_language_from_path(p).ok()).collect::<Vec<_>>();

    // Check that there is at least two files
    if detected_languages.len() <= 1 {
        return Err(DirectoryValidationError::InsufficientData);
    }

    if !detected_languages.iter().all(|l| *l == detected_languages[0]) {
        return Err(DirectoryValidationError::MultipleLanguages);
    }

    // Set this directory as the current state directory, creating a data file for it if it doesn't exist
    let load = if path.join("dataset.json").exists() {
        let data = fs::read_to_string(path.join("dataset.json"))?;
        let data = serde_json::from_str(&data)?;
        data
    } else {
        Dataset {
            pairs: Default::default(),
        }
    };

    let _ = state.current_dataset.write().unwrap().insert(CurrentDataset {
        data: load,
        path: path.to_path_buf(),
        items: file_paths,
    });

    Ok(())
}

struct Item {
    path: String,
    contents: String,
}

struct PairData {
    a: Item,
    b: Item,
    pairs: Vec<Pair>,
}

#[derive(Debug, thiserror::Error)]
enum DatasetError {
    #[error("No dataset loaded")]
    NoDataset,
}

str_error!(DatasetError);

#[tauri::command]
fn get_overview(state: tauri::State<'_, AppState>) -> Result<Vec<Option<usize>>, DatasetError> {
    let current = state.current_dataset.read().unwrap();
    match current.as_ref() {
        None => Err(DatasetError::NoDataset),
        Some(current) => {
            let mut status = vec![None; current.items.len()];

            for (&k, v) in &current.data.pairs {
                status[k] = Some(v.marks.len());
            }

            Ok(status)
        }
    }
}

#[tauri::command]
fn load_pair(pair_index: usize) {

}

#[tauri::command]
fn set_spans(pair_index: usize) {

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![validate_directory, load_pair, set_spans, get_overview])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
