#![feature(async_closure)]

use std::{env::args, fs, path::{Path, PathBuf}, io::Write};

use analysis::{AssociatedFileProvider, AssociatedStruct, detect_plagiarism_in_sources, Language};
use async_trait::async_trait;
use futures::{stream::StreamExt, future::join_all};

extern crate analysis;

struct FileProvider {
    original: PathBuf,
    plagiarized: Vec<PathBuf>,
    non_plagiarized: Vec<PathBuf>,
}

#[async_trait]
impl AssociatedFileProvider for FileProvider {
    type Ident = usize;

    type S = String;

    async fn read_files(&self) -> anyhow::Result<Vec<AssociatedStruct<Self::Ident, Self::S>>> {
        let iter = futures::stream::iter([&self.original]).chain(futures::stream::iter(self.plagiarized.iter())).chain(futures::stream::iter(self.non_plagiarized.iter()));

        let res = iter.map(async move |p| AssociatedStruct { owner: 1234, source: p.to_str().unwrap().to_owned(), inner: tokio::fs::read_to_string(p).await.unwrap() }).collect::<Vec<_>>().await;
        let res = join_all(res).await;
        Ok(res)
    }
}

// Examples from https://infedu.vu.lt/journal/INFEDU/article/16/info (https://github.com/oscarkarnalim/sourcecodeplagiarismdataset)
#[tokio::main]
async fn main() {
    let mut args = args().skip(1);
    if args.len() != 2 {
        panic!("Inavlid usage! Usage: <exec> <dataset root path> <output parent path>");
    }
    let dataset = PathBuf::from(args.next().unwrap()).canonicalize().unwrap();
    let output = PathBuf::from(args.next().unwrap());
    fs::create_dir_all(&output).unwrap();
    
    for case in fs::read_dir(dataset).unwrap() {
        let case = case.unwrap();

        let plagiarized_dir = case.path().join("plagiarized");
        let plagiarized_files = fs::read_dir(plagiarized_dir).unwrap()
            .filter(|rd| rd.as_ref().unwrap().file_type().unwrap().is_dir())
            .flat_map(|rd| fs::read_dir(rd.unwrap().path()).unwrap())
            .filter(|rd| rd.as_ref().unwrap().file_type().unwrap().is_dir())
            .map(|rd| get_single_file_from_dir(rd.unwrap().path()))
            .collect::<Vec<_>>();

        let non_plagiarized_dir = case.path().join("non-plagiarized");
        let non_plagiarized_files = fs::read_dir(non_plagiarized_dir).unwrap().filter(|rd| rd.as_ref().unwrap().file_type().unwrap().is_dir()).map(|rd| get_single_file_from_dir(rd.unwrap().path())).collect::<Vec<_>>();

        let original_file = get_single_file_from_dir(case.path().join("original"));

        let provider = FileProvider {
            original: original_file.clone(),
            plagiarized: plagiarized_files.clone(),
            non_plagiarized: non_plagiarized_files.clone(),
        };

        let mut f = fs::File::create(output.join(format!("{}.csv", case.file_name().to_string_lossy()))).unwrap();

        f.write_all(b",").unwrap();

        let orig = vec![original_file];
        let it = orig.iter().chain(plagiarized_files.iter()).chain(non_plagiarized_files.iter());
        
        for path in it.clone() {
            f.write_all(format!("{}, ", path.to_string_lossy()).as_bytes()).unwrap();
        }

        f.write_all(b"\n").unwrap();

        let mat = detect_plagiarism_in_sources(&provider, Some(Language::Java)).await.unwrap();
        for (row, file) in mat.row_iter().zip(it) {
            f.write_all(format!("{}, ", file.to_string_lossy()).as_bytes()).unwrap();

            for item in &row {
                f.write_all(format!("{}, ", item).as_bytes()).unwrap();
            }

            f.write_all(b"\n").unwrap();
        }
    }
}

#[inline]
fn get_single_file_from_dir<P: AsRef<Path>>(path: P) -> PathBuf {
    fs::read_dir(path).unwrap().find(|rd| rd.as_ref().unwrap().file_name().to_str().unwrap().ends_with(".java")).unwrap().unwrap().path()
}