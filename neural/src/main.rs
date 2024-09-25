use std::{
    fs::{create_dir_all, remove_dir_all},
    path::Path,
};

use burn::{
    backend::Wgpu,
    config::Config,
    data::dataloader::DataLoaderBuilder,
    module::Module,
    optim::AdamConfig,
    record::CompactRecorder,
    tensor::backend::AutodiffBackend,
    train::{metric::LossMetric, LearnerBuilder},
};
use neural::{
    data::{AnyDataset, AstBatcher, CollatedAstDataset},
    model::ModelConfig,
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

fn train<B: AutodiffBackend>(artifact_dir: &Path, config: TrainingConfig, device: B::Device)
where
    <B as AutodiffBackend>::InnerBackend: AutodiffBackend,
{
    create_artifact_directory(artifact_dir);

    config
        .save(artifact_dir.join("config.json"))
        .expect("config to save successfully");

    B::seed(config.seed);

    let mut dataset = CollatedAstDataset::default();

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
        .metric_valid_numeric(LossMetric::new())
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

fn main() {}
