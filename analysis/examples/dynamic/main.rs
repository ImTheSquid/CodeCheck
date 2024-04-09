#![feature(iterator_try_collect)]

use std::{
    borrow::Cow, env::args, fs, io::Write, path::PathBuf, sync::Arc, thread::spawn, time::Duration,
};

use analysis::{detect_plagiarism_in_sources, AssociatedStruct};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::sync::mpsc;

extern crate analysis;

fn main() {
    // Open each file in pointed-to directory
    // Find all relevant files in directories
    let mut args = args();
    let exec = args.next().expect("executable");
    if args.len() < 3 {
        panic!("Inavlid usage! Usage: {exec} <dataset root path> <output parent path> <expected file extensions..., eg java cpp>");
    }
    let mut dataset = PathBuf::from(args.next().unwrap()).canonicalize().unwrap();
    let output = PathBuf::from(args.next().unwrap());
    let extensions = args.collect::<Vec<_>>();
    fs::create_dir_all(&output).unwrap();

    dataset.push(PathBuf::from("**/*.*"));

    let files = glob::glob(dataset.to_str().expect("Path string"))
        .expect("good glob")
        .filter(|f| {
            f.as_ref().is_ok_and(|o| {
                extensions.contains(
                    &o.extension()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                )
            })
        })
        .try_collect::<Vec<_>>()
        .expect("good files");

    let num_comps = {
        let num_trees = files.len();
        num_trees * (num_trees - 1) / 2
    };

    let mut f = fs::File::create(output.join("output.csv")).unwrap();

    for path in &files {
        f.write_all(format!("{}, ", path.to_string_lossy()).as_bytes())
            .unwrap();
    }

    let sources = files
        .iter()
        .map(|p| AssociatedStruct {
            owner: Arc::new(
                p.components()
                    .nth(dataset.components().collect::<Vec<_>>().len() - 2)
                    .expect("submission")
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_owned(),
            ),
            source: Cow::Owned(p.as_os_str().to_string_lossy().as_ref().to_owned()),
            inner: fs::read_to_string(p).unwrap(),
        })
        .collect::<Vec<_>>();

    f.write_all(b"\n").unwrap();

    println!("Starting dataset");
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

    let mat = detect_plagiarism_in_sources(sources, None, Some(tx)).unwrap();
    for (row, file) in mat.row_iter().zip(&files) {
        f.write_all(format!("{}, ", file.to_string_lossy()).as_bytes())
            .unwrap();

        for item in &row {
            f.write_all(format!("{}, ", item).as_bytes()).unwrap();
        }

        f.write_all(b"\n").unwrap();
    }

    jh.join().unwrap();
}
