use std::{
    fs::{create_dir_all, remove_dir_all},
    path::{Path, PathBuf},
};

use burn::{
    backend::{wgpu::WgpuDevice, Autodiff, Wgpu},
    config::Config,
    data::dataloader::DataLoaderBuilder,
    module::Module,
    optim::AdamConfig,
    record::CompactRecorder,
    tensor::backend::AutodiffBackend,
    train::{
        metric::{CpuMemory, CpuTemperature, CpuUse, LossMetric},
        LearnerBuilder,
    },
};
use clap::Parser;
use neural::{
    data::{AnyDataset, AstBatcher, CollatedAstDataset, RawAstDataset},
    gat::GatConfig,
    model::ModelConfig,
};

use burn::{
    nn::{
        conv::{Conv2d, Conv2dConfig},
        pool::{AdaptiveAvgPool2d, AdaptiveAvgPool2dConfig},
        Dropout, DropoutConfig, Linear, LinearConfig, Relu,
    },
    prelude::*,
};

type Backend = Wgpu;

#[derive(Config)]
pub struct TrainingConfig {
    pub model: ModelConfig,
    pub optimizer: AdamConfig,
    #[config(default = 10)]
    pub num_epochs: usize,
    #[config(default = 64)]
    pub batch_size: usize,
    #[config(default = 4)]
    pub num_workers: usize,
    #[config(default = 42)]
    pub seed: u64,
    #[config(default = 1.0e-4)]
    pub learning_rate: f64,
}

/// Ensures the given path exists as a directory
fn create_artifact_directory(path: &Path) {
    remove_dir_all(path).ok();
    create_dir_all(path).expect("directory creation to succeed");
}

fn train<B: AutodiffBackend>(
    artifact_dir: &Path,
    config_dir: &Path,
    raw_datasets: Vec<RawAstDataset>,
    config: TrainingConfig,
    device: B::Device,
) {
    create_artifact_directory(artifact_dir);

    config
        .save(config_dir.join("config.json"))
        .expect("config to save successfully");

    B::seed(config.seed);

    let mut dataset = CollatedAstDataset::default();

    for raw in raw_datasets {
        dataset.include(raw);
    }

    let dataset = dataset.to_arc();

    let batcher_train = AstBatcher::<B>::default();
    let batcher_valid = AstBatcher::<B::InnerBackend>::default();

    let dataloader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(dataset.train());

    let dataloader_valid = DataLoaderBuilder::new(batcher_valid)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(dataset.test());

    let learner = LearnerBuilder::new(artifact_dir)
        .metric_train_numeric(LossMetric::new())
        // .metric_valid_numeric(LossMetric::new())
        .metric_train_numeric(CpuUse::new())
        .metric_valid_numeric(CpuUse::new())
        .metric_train_numeric(CpuMemory::new())
        .metric_valid_numeric(CpuMemory::new())
        .metric_train_numeric(CpuTemperature::new())
        .metric_valid_numeric(CpuTemperature::new())
        .with_file_checkpointer(CompactRecorder::new())
        .devices(vec![device.clone()])
        .num_epochs(config.num_epochs)
        .summary()
        .build(
            config.model.init::<B>(&device),
            config.optimizer.init(),
            config.learning_rate,
        );

    let trained_model = learner.fit(dataloader_train, dataloader_valid);

    trained_model
        .save_file(artifact_dir.join("model"), &CompactRecorder::new())
        .expect("Trained model should be saved successfully");
}

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

    let gat_config = GatConfig::new(vec![100, 90, 80, 70], vec![8, 8, 1]);

    let device = WgpuDevice::default();

    let config = ModelConfig::new(gat_config);
    let config = TrainingConfig::new(config, AdamConfig::new());

    train::<Autodiff<Backend>>(
        &args.artifact_dir,
        &args.config_dir,
        datasets,
        config,
        device,
    );
}
