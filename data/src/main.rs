use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
    time::Duration,
};

use ast::Language;
use clap::Parser;
use data::{PlagiarismEvent, ProblemComplexity, generate_code};
use eyre::{Result, bail};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use ndarray::Axis;
use ollama_rs::{
    Ollama,
    generation::embeddings::{
        GenerateEmbeddingsResponse,
        request::{EmbeddingsInput, GenerateEmbeddingsRequest},
    },
};
use rand::distr::{Alphanumeric, SampleString};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    time::Instant,
};
use util::{Dataset, Mark, MarkSpan, Pair, arr_vec_to_view};

#[derive(Debug, clap::Parser)]
struct Args {
    /// The name of the model to use
    model_name: String,
    /// The host at which Ollama is served
    #[arg(short = 'o', default_value = "http://127.0.0.1")]
    ollama_host: String,
    /// The port at which Ollama is served
    #[arg(short = 'p', default_value = "11434")]
    ollama_port: u16,
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Data {
        /// The directory in which to place the dataset (created if doesn't exist)
        dataset_dir: PathBuf,
        /// How big of a problem the model should try to generate
        #[arg(short = 'c', default_value = "average")]
        complexity: ProblemComplexity,
        /// Number of examples to generate
        #[arg(short = 'n', default_value = "1")]
        num_iters: u64,
        /// How many previous tasks the model can remember
        #[arg(short = 'm', default_value = "10")]
        memory: usize,
        /// Ignore generation failures
        #[arg(long = "ignore-failures", default_value = "false")]
        ignore_failures: bool,
        /// Overwrites output directory if it exists
        #[arg(long = "overwrite", default_value = "false")]
        overwrite: bool,
        /// Forbids non-plagiarized entries in the dataset
        #[arg(long = "forbid-np", default_value = "false")]
        disallow_non_plagiarized_code: bool,
    },
    Embeddings {
        /// The path to output the numpy file
        output_path: PathBuf,
    },
}

async fn write_files_and_update_manifest(
    dataset: &mut Dataset,
    files: &[(String, Language)],
    pairs: &HashMap<String, Vec<PlagiarismEvent>>,
    basedir: &Path,
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
            // Make sure files always stay ordered
            let (a, b) = if a.file < b.file { (a, b) } else { (b, a) };

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

    for ((a, b), marks) in data {
        dataset.pairs.push(Pair {
            a: files[a].1.clone(),
            b: files[b].1.clone(),
            marks,
        });
    }

    Ok(())
}

async fn create_embeddings(
    ollama: &Ollama,
    model_name: String,
    output_path: PathBuf,
) -> Result<()> {
    use ast::{c::CTreeItem, cpp::CppTreeItem, java::JavaTreeItem, python::PythonTreeItem};
    use strum::VariantNames;
    let all_items: Vec<_> = [
        CTreeItem::VARIANTS,
        CppTreeItem::VARIANTS,
        JavaTreeItem::VARIANTS,
        PythonTreeItem::VARIANTS,
    ]
    .into_iter()
    .flatten()
    .map(|s| s.to_string())
    .collect();

    let GenerateEmbeddingsResponse { embeddings } = ollama
        .generate_embeddings(GenerateEmbeddingsRequest::new(
            model_name,
            EmbeddingsInput::Multiple(all_items.clone()),
        ))
        .await?;

    let embeddings = embeddings
        .into_iter()
        .map(ndarray::Array1::from_vec)
        .collect::<Vec<_>>();
    let embeddings = ndarray::stack(Axis(0), arr_vec_to_view!(embeddings))?;
    let f = File::create(output_path)?;
    let mut w = ndarray_npy::NpzWriter::new(f);
    w.add_array("embeddings", &embeddings)?;
    w.finish()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        model_name,
        ollama_host,
        ollama_port,
        ..
    } = args;

    let ollama = Ollama::new(ollama_host, ollama_port);
    if !ollama
        .list_local_models()
        .await?
        .iter()
        .any(|m| m.name == model_name.as_str())
    {
        println!("Model \"{model_name}\" not found locally, attempting to pull.",);
        ollama.pull_model(model_name.clone(), false).await?;
    }

    match args.command {
        Command::Embeddings { output_path } => {
            create_embeddings(&ollama, model_name, output_path).await?;
        }
        Command::Data {
            dataset_dir,
            complexity,
            num_iters,
            memory,
            ignore_failures,
            overwrite,
            disallow_non_plagiarized_code,
        } => {
            if fs::try_exists(&dataset_dir).await?
                && fs::read_dir(&dataset_dir)
                    .await?
                    .next_entry()
                    .await?
                    .is_some()
            {
                if overwrite {
                    fs::remove_dir_all(&dataset_dir).await?;
                } else {
                    bail!("Dataset directory not empty!");
                }
            }

            fs::create_dir_all(&dataset_dir).await?;

            let mut topics = Vec::with_capacity(memory);
            let mut durations = Vec::with_capacity(num_iters as usize);
            let mut dataset = Dataset::default();
            let mut failures = 0usize;

            let p = ProgressBar::new(num_iters);
            p.enable_steady_tick(Duration::from_millis(100));
            p.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({msg})")
                            .unwrap()
                            .progress_chars("#>-"));

            p.set_message("avg ?s, 0 failures");

            while !p.is_finished() {
                let start = Instant::now();

                let res = generate_code(
                    &ollama,
                    model_name.clone(),
                    &topics,
                    complexity,
                    disallow_non_plagiarized_code,
                )
                .await;

                let end = Instant::now();
                durations.push(end - start);
                let s = durations
                    .iter()
                    .cloned()
                    .reduce(|p, n| p.saturating_add(n))
                    .unwrap_or_default();

                match res {
                    Ok(res) => {
                        p.inc(1);
                        if p.position() == p.length().expect("length") {
                            p.finish();
                        }

                        write_files_and_update_manifest(
                            &mut dataset,
                            &res.codes,
                            &res.pairs,
                            &dataset_dir,
                        )
                        .await?;

                        if topics.len() == memory {
                            topics.clear();
                        }

                        if let Some(topic) = res.topic {
                            topics.push(topic);
                        }
                    }
                    Err(e) => {
                        if ignore_failures {
                            eprintln!("Error encountered: {e}");
                            failures += 1;
                        } else {
                            bail!(e);
                        }
                    }
                }

                p.set_message(format!(
                    "avg {}, {} failure{}",
                    indicatif::HumanDuration(s.checked_div(durations.len() as u32).unwrap()),
                    failures,
                    if failures != 1 { "s" } else { "" }
                ));
            }

            let mut f = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(dataset_dir.join("dataset.json"))
                .await?;

            f.write_all(serde_json::to_string(&dataset)?.as_bytes())
                .await?;
        }
    }

    Ok(())
}
