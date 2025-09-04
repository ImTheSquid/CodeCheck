#![feature(new_range_api)]

use std::{
    env::{self, args_os},
    ffi::CString,
    path::PathBuf,
    str::FromStr,
};

use clap::{Args, Parser};
use neural::data::{CollatedAstDataset, RawAstDataset};
use pyo3::Python;

#[derive(Debug, clap::Parser)]
struct Arguments {
    #[command(flatten)]
    lt: LaunchType,
    /// The datasets to include in training.
    /// Must all have a `dataset.json` file in the root!
    datasets: Vec<PathBuf>,
    /// A Python venv to use
    #[arg(short)]
    venv: Option<PathBuf>,
    /// Mode
    #[arg(short = 'm', value_enum, default_value = "train")]
    mode: Mode,
    /// Embeddings only: Top-k for triplet mining
    #[arg(short = 'k', default_value = "3")]
    top_k: usize,
    #[arg(short = 'd')]
    device: Option<String>,
    #[arg(long = "multiprocessing-fork", default_value = "false")]
    _mpf: bool,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Mode {
    Train,
    Embed,
    EmbedTest,
    Eval,
}

impl Mode {
    fn py_id(&self) -> &str {
        match self {
            Mode::Train => "train",
            Mode::Embed => "embed",
            Mode::EmbedTest => "embed-test",
            Mode::Eval => "eval",
        }
    }
}

#[derive(Debug, Args)]
#[group(required = true, args = ["artifact_dir", "py", "config_dir"])]
struct LaunchType {
    #[command(flatten)]
    dirs: Directories,

    #[arg(short = 'c')]
    py: Option<String>,
}

#[derive(Debug, Args)]
struct Directories {
    /// Where to store the training configuration.
    #[arg(short = 'o', requires = "artifact_dir")]
    config_dir: Option<PathBuf>,
    /// Where to store training artifacts.
    /// Will be deleted and recreated if already exists!
    #[arg(short = 'a', requires = "config_dir")]
    artifact_dir: Option<PathBuf>,
}

fn main() {
    let a: Vec<_> = args_os().collect();

    let args = Arguments::parse_from(a.iter());

    env::set_var("PYTHONMALLOC", "mimalloc");
    if let Some(cmd) = args.lt.py {
        Python::with_gil(|py| {
            neural::initialize_python(
                py,
                env::var("CODECHECK_VENV")
                    .ok()
                    .map(|ccv| PathBuf::from_str(&ccv).expect("valid path")),
            );
            neural::add_rust_data(py).expect("add rd");
            neural::mp_mode(py).expect("add mp");

            let _ = py.run(CString::new(cmd).expect("cstr").as_c_str(), None, None);
        });
    } else {
        let datasets = args
            .datasets
            .into_iter()
            .map(|p| {
                RawAstDataset::try_from(p.as_path())
                    .unwrap_or_else(|e| panic!("{e:?}: Valid dataset on path {p:?}"))
            })
            .collect::<Vec<_>>();

        let mut collated = CollatedAstDataset::default();

        for dataset in datasets {
            collated.include(dataset);
        }

        let dataset = collated.compile_to_tmpdir().expect("valid load for path");

        if let Some(venv) = &args.venv {
            println!("🔄 Loading venv from {venv:?}");
            env::set_var("CODECHECK_VENV", venv);
        }

        let dirs = args.lt.dirs;

        Python::with_gil(|py| {
            neural::initialize_python(py, args.venv);
            neural::add_rust_data(py).expect("add rd");

            match args.mode {
                Mode::Embed | Mode::EmbedTest | Mode::Train => {
                    if let Err(e) = neural::train(
                        py,
                        dataset,
                        args.mode.py_id(),
                        &dirs.artifact_dir.unwrap().to_string_lossy(),
                        &dirs.config_dir.unwrap().to_string_lossy(),
                        args.top_k,
                        args.device,
                    ) {
                        e.print(py);
                        std::process::exit(1);
                    }
                }
                Mode::Eval => {
                    match neural::eval(
                        py,
                        dataset,
                        &dirs.artifact_dir.unwrap().to_string_lossy(),
                        &dirs.config_dir.unwrap().to_string_lossy(),
                        args.device,
                    ) {
                        Ok(res) => {
                            for (k, v) in res {
                                if k == -1 {
                                    println!("The following items have no cluster:");
                                } else {
                                    println!("The following items are grouped as PLAGIARIZED:");
                                }

                                for er in v {
                                    println!("{}: {}..{}", er.id, er.range.start, er.range.end);
                                }
                            }
                        }
                        Err(e) => {
                            e.print(py);
                            std::process::exit(1);
                        }
                    }
                }
            }
        });
    }
}
