use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Duration,
};

use ast::{Language, r#gen::cpp14parser::Default};
use clap::Parser;
use data::{ContentLength, PlagiarismEvent, generate_code};
use eyre::{Result, bail};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use ollama_rs::Ollama;
use rand::distr::{Alphanumeric, SampleString};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    time::Instant,
};
use util::{Dataset, Mark, MarkSpan, Pair};

#[derive(Debug, clap::Parser)]
struct Args {
    model_name: String,
    /// The directory in which to place the dataset (created if doesn't exist)
    dataset_dir: PathBuf,
    /// How big of a problem the model should try to generate
    #[arg(short = 'l', default_value = "medium")]
    content_length: ContentLength,
    /// Number of examples to generate
    #[arg(short = 'n', default_value = "1")]
    num_iters: u64,
    /// How many previous tasks the model can remember
    #[arg(short = 'm', default_value = "10")]
    memory: usize,
    /// Ignore errors from generation
    #[arg(long = "ignore-errors", default_value = "false")]
    ignore_errors: bool,
    /// Overwrites output directory if it exists
    #[arg(long = "overwrite", default_value = "false")]
    overwrite: bool,
}

async fn write_files_and_update_manifest(
    dataset: &mut Dataset,
    files: &[(String, Language)],
    pairs: &HashMap<String, Vec<PlagiarismEvent>>,
    basedir: &Path,
    base_index: usize,
) -> Result<()> {
    let files = files
        .iter()
        .map(|(c, l)| {
            (
                c,
                format!(
                    "{}.{}",
                    Alphanumeric.sample_string(&mut rand::rng(), 10),
                    match l {
                        Language::C => "c",
                        Language::Cpp => "cpp",
                        Language::Java => "java",
                        Language::Python => "py",
                    }
                ),
            )
        })
        .collect::<Vec<_>>();

    // Write files to disk
    for (code, path) in &files {
        let mut f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(basedir.join(path))
            .await?;

        f.write_all(code.as_bytes()).await?;
    }

    // Add to the dataset
    // Need to collate
    // Known: pairs.values() vectors are sorted
    let mut data: HashMap<(usize, usize), Vec<Mark>> = Default::default();
    for plagiarising_set in pairs.values() {
        // Each of these combinations plagiarizes off eachother
        for (a, b) in plagiarising_set.iter().tuple_combinations() {
            assert!(
                a.file < b.file,
                "a's file index should always be less than b's, but got a={}, b={}",
                a.file,
                b.file
            );

            let mark = Mark {
                a: MarkSpan {
                    start: a.start,
                    end: a.end,
                },
                b: MarkSpan {
                    start: b.start,
                    end: b.end,
                },
            };
            match data.get_mut(&(a.file, b.file)) {
                Some(spans) => {
                    spans.push(mark);
                }
                None => {
                    data.insert((a.file, b.file), vec![mark]);
                }
            }
        }
    }

    for (i, ((a, b), marks)) in data.into_iter().enumerate() {
        dataset.pairs.insert(
            i + base_index,
            Pair {
                a: files[a].1.clone(),
                b: files[b].1.clone(),
                marks,
            },
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if fs::try_exists(&args.dataset_dir).await?
        && fs::read_dir(&args.dataset_dir)
            .await?
            .next_entry()
            .await?
            .is_some()
    {
        if args.overwrite {
            fs::remove_dir_all(&args.dataset_dir).await?;
        } else {
            bail!("Dataset directory not empty!");
        }
    }

    fs::create_dir_all(&args.dataset_dir).await?;

    let p = ProgressBar::new(args.num_iters);
    p.enable_steady_tick(Duration::from_millis(100));
    p.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({msg})")
            .unwrap()
            .progress_chars("#>-"));

    p.set_message("avg ?s");

    let ollama = Ollama::default();
    let mut topics = Vec::with_capacity(args.memory);
    let mut durations = Vec::with_capacity(args.num_iters as usize);
    let mut dataset = Dataset::default();

    for _ in 0..args.num_iters {
        let start = Instant::now();
        let res = generate_code(
            &ollama,
            args.model_name.clone(),
            &topics,
            args.content_length,
        )
        .await;
        p.inc(1);
        let res = match res {
            Ok(res) => res,
            Err(e) => {
                if args.ignore_errors {
                    eprintln!("Error encountered: {e}");

                    continue;
                } else {
                    bail!(e);
                }
            }
        };

        let base_index = dataset.pairs.keys().max().cloned().unwrap_or_default();
        write_files_and_update_manifest(
            &mut dataset,
            &res.codes,
            &res.pairs,
            &args.dataset_dir,
            if base_index == 0 { 0 } else { base_index + 1 },
        )
        .await?;

        if topics.len() == args.memory {
            topics.clear();
        }

        topics.push(res.topic);
        let end = Instant::now();
        durations.push(end - start);
        let s = durations
            .iter()
            .cloned()
            .reduce(|p, n| p.saturating_add(n))
            .unwrap_or_default();
        p.set_message(format!(
            "avg {}s",
            s.checked_div(durations.len() as u32).unwrap().as_secs()
        ));
    }

    p.finish();

    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(args.dataset_dir.join("dataset.json"))
        .await?;

    f.write_all(serde_json::to_string(&dataset)?.as_bytes())
        .await?;

    Ok(())
}
