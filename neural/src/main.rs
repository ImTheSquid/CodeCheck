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
    // /// Where to store the training configuration.
    // /// config_dir/config.json will be overwritten!
    // config_dir: Option<PathBuf>,
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
    #[arg(long = "cpu", default_value = "false")]
    force_cpu: bool,
    #[arg(long = "multiprocessing-fork", default_value = "false")]
    _mpf: bool,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Mode {
    Train,
    Embed,
    EmbedTest,
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
struct LaunchType {
    /// Where to store training artifacts.
    /// Will be deleted and recreated if already exists!
    #[arg(short = 'a')]
    artifact_dir: Option<PathBuf>,
    #[arg(short = 'c')]
    py: Option<String>,
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

        Python::with_gil(|py| {
            neural::initialize_python(py, args.venv);
            neural::add_rust_data(py).expect("add rd");

            if let Err(e) = neural::train(
                py,
                dataset,
                match args.mode {
                    Mode::Embed => "embed",
                    Mode::EmbedTest => "embed-test",
                    Mode::Train => "train",
                },
                &args
                    .lt
                    .artifact_dir
                    .expect("artifact dir")
                    .to_string_lossy(),
                args.top_k,
                args.force_cpu,
            ) {
                e.print(py);
                std::process::exit(1);
            }
        });
    }
}
