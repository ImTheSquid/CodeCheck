#![feature(async_closure)]

use std::{env::args, fs, path::{Path, PathBuf}, io::Write, time::Duration, thread::spawn, borrow::Cow};

use analysis::{AssociatedStruct, detect_plagiarism_in_sources, Language};
use std::sync::mpsc;
use indicatif::{ProgressBar, ProgressStyle, HumanDuration};

extern crate analysis;

// Examples from https://infedu.vu.lt/journal/INFEDU/article/16/info (https://github.com/oscarkarnalim/sourcecodeplagiarismdataset)
fn main() {
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

        let mut f = fs::File::create(output.join(format!("{}.csv", case.file_name().to_string_lossy()))).unwrap();

        f.write_all(b",").unwrap();

        let orig = vec![original_file];
        let it = orig.iter().chain(plagiarized_files.iter()).chain(non_plagiarized_files.iter()).collect::<Vec<_>>();

        let num_comps = {
            let num_trees = it.len();
            num_trees * (num_trees - 1) / 2
        };
        
        for path in &it {
            f.write_all(format!("{}, ", path.to_string_lossy()).as_bytes()).unwrap();
        }

        let sources = it.iter().map(|p| AssociatedStruct {
            owner: &1234,
            source: Cow::Owned(p.as_os_str().to_string_lossy().as_ref().to_owned()),
            inner: fs::read_to_string(p).unwrap(),
        }).collect::<Vec<_>>();

        f.write_all(b"\n").unwrap();

        println!("Starting dataset {}", case.file_name().to_string_lossy());
        let (tx, rx) = mpsc::channel();
        let jh = spawn(move || {
            let pb = ProgressBar::new(num_comps as u64).with_style(ProgressStyle::with_template("[{elapsed_precise}] {spinner} Executing comparisons... {wide_bar} {pos:>7}/{len:7} ({percent}%)").unwrap());
            pb.enable_steady_tick(Duration::from_millis(100));
            while let Ok(()) = rx.recv() {
                pb.inc(1);
            }
            pb.finish();
            println!("✅ Comparisons finished in {}", HumanDuration(pb.elapsed()));
        });

        let mat = detect_plagiarism_in_sources(&sources, Some(Language::Java), Some(tx)).unwrap();
        for (row, file) in mat.row_iter().zip(&it) {
            f.write_all(format!("{}, ", file.to_string_lossy()).as_bytes()).unwrap();

            for item in &row {
                f.write_all(format!("{}, ", item).as_bytes()).unwrap();
            }

            f.write_all(b"\n").unwrap();
        }

        jh.join().unwrap();
    }
}

#[inline]
fn get_single_file_from_dir<P: AsRef<Path>>(path: P) -> PathBuf {
    fs::read_dir(path).unwrap().find(|rd| rd.as_ref().unwrap().file_name().to_str().unwrap().ends_with(".java")).unwrap().unwrap().path()
}