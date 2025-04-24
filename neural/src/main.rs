use std::path::PathBuf;

// use burn::{
//     backend::{wgpu::WgpuDevice, Autodiff},
//     config::Config,
//     data::dataloader::DataLoaderBuilder,
//     grad_clipping::GradientClippingConfig,
//     module::Module,
//     nn::LstmConfig,
//     optim::{decay::WeightDecayConfig, AdamConfig},
//     record::CompactRecorder,
//     tensor::backend::AutodiffBackend,
//     train::{
//         metric::{CpuMemory, CpuTemperature, CpuUse, LossMetric},
//         LearnerBuilder,
//     },
// };
use clap::Parser;
use neural::{
    data::{CollatedAstDataset, RawAstDataset},
    KeyData,
};
use pyo3::Python;

// type Backend = ::burn::backend::Wgpu;

// #[derive(Config)]
// pub struct TrainingConfig {
//     pub model: ModelConfig,
//     pub optimizer: AdamConfig,
//     #[config(default = 10)]
//     pub num_epochs: usize,
//     #[config(default = 64)]
//     pub batch_size: usize,
//     #[config(default = 4)]
//     pub num_workers: usize,
//     #[config(default = 42)]
//     pub seed: u64,
//     #[config(default = 1.0e-4)]
//     pub learning_rate: f64,
// }

// fn train<B: AutodiffBackend>(
//     artifact_dir: &Path,
//     config_dir: &Path,
//     raw_datasets: Vec<RawAstDataset>,
//     config: TrainingConfig,
//     device: B::Device,
// ) {
//     config
//         .save(config_dir.join("config.json"))
//         .expect("config to save successfully");

//     B::seed(config.seed);

//     let mut dataset = CollatedAstDataset::default();

//     for raw in raw_datasets {
//         dataset.include(raw);
//     }

//     let dataset = dataset.to_arc();

//     let batcher_train = AstBuilder::<B>::new(device.clone());
//     let batcher_valid = AstBuilder::<B::InnerBackend>::new(device.clone());

//     let dataloader_train = DataLoaderBuilder::new(batcher_train)
//         .batch_size(config.batch_size)
//         .shuffle(config.seed)
//         .num_workers(config.num_workers)
//         .build(dataset.train());

//     let dataloader_valid = DataLoaderBuilder::new(batcher_valid)
//         .batch_size(config.batch_size)
//         .shuffle(config.seed)
//         .num_workers(config.num_workers)
//         .build(dataset.test());

//     // let s = config.model.init::<B>(&device).to_string();
//     // println!("{s}");

//     let learner = LearnerBuilder::new(artifact_dir)
//         .metric_train_numeric(LossMetric::new())
//         // .metric_valid_numeric(LossMetric::new())
//         .metric_train_numeric(CpuUse::new())
//         .metric_valid_numeric(CpuUse::new())
//         .metric_train_numeric(CpuMemory::new())
//         .metric_valid_numeric(CpuMemory::new())
//         .metric_train_numeric(CpuTemperature::new())
//         .metric_valid_numeric(CpuTemperature::new())
//         .with_file_checkpointer(CompactRecorder::new())
//         .devices(vec![device.clone()])
//         .num_epochs(config.num_epochs)
//         .summary()
//         .build(
//             config.model.init::<B>(&device),
//             config.optimizer.init(),
//             config.learning_rate,
//         );

//     let trained_model = learner.fit(dataloader_train, dataloader_valid);

//     trained_model
//         .save_file(artifact_dir.join("model"), &CompactRecorder::new())
//         .expect("Trained model should be saved successfully");
// }

#[derive(Debug, clap::Parser)]
struct Args {
    /// Where to store training artifacts.
    /// Will be deleted and recreated if already exists!
    artifact_dir: PathBuf,
    /// Where to store the training configuration.
    /// config_dir/config.json will be overwritten!
    config_dir: PathBuf,
    /// The datasets to include in training.
    /// Must all have a `dataset.json` file in the root!
    datasets: Vec<PathBuf>,
    /// A Python venv to use
    #[arg(short)]
    venv: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

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

    let (features, edges, feature_spans, keys) = collated.compile().expect("valid load");

    let keys = keys
        .into_iter()
        .map(|(k, v)| KeyData {
            a: k.0,
            b: k.1,
            marks: v.marks,
        })
        .collect::<Vec<_>>();

    // Initialize Python manually to allow for venv loading
    // SAFETY: All calls here are checked
    // unsafe {
    //     use pyo3::ffi::*;

    //     unsafe fn bail_on_bad_status(status: PyStatus) {
    //         if PyStatus_Exception(status) != 0 {
    //             Py_ExitStatusException(status);
    //         }
    //     }

    //     fn to_wide_null(s: &str) -> Vec<i32> {
    //         s.encode_utf16().map(|c| c as i32).chain(Some(0)).collect()
    //     }

    //     let mut cfg: PyConfig = std::mem::zeroed();
    //     PyConfig_InitPythonConfig(&mut cfg);

    //     bail_on_bad_status(PyConfig_SetString(
    //         &mut cfg,
    //         &mut cfg.home,
    //         to_wide_null("/Users/jackhogan/Code/Rust/CodeCheck/neural/py/.venv").as_ptr(),
    //     ));

    //     cfg.use_environment = 1;

    //     bail_on_bad_status(Py_InitializeFromConfig(&cfg));

    //     PyConfig_Clear(&mut cfg);
    // }

    Python::with_gil(|py| {
        neural::initialize_python(py, args.venv);

        if let Err(e) = neural::train(py, &features, &edges, &feature_spans, &keys) {
            e.print(py);
            std::process::exit(1);
        }
    });

    // let gat_config = GatConfig::new(vec![15, 15, 15, 8], vec![8, 8, 3]);
    // let lstm_config = LstmConfig::new(8, 16, true);

    // let device = WgpuDevice::default();

    // let config = ModelConfig::new(gat_config, lstm_config);
    // let config = TrainingConfig::new(
    //     config,
    //     AdamConfig::new()
    //         .with_weight_decay(Some(WeightDecayConfig::new(1e-5)))
    //         .with_grad_clipping(Some(GradientClippingConfig::Value(1.0))),
    // )
    // .with_num_workers(num_cpus::get());

    // train::<Autodiff<Backend>>(
    //     &args.artifact_dir,
    //     &args.config_dir,
    //     datasets,
    //     config,
    //     device,
    // );
}
