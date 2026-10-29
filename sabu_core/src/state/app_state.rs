//! State shared between the UI thread and the command worker.

use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::dataset::Dataset;
use crate::error::CoreError;

/// State shared between the UI thread and the command worker.
#[derive(Debug)]
pub struct AppState {
    /// Current user configuration.
    pub config: Config,
    /// Loaded dataset, if any.
    pub dataset: Option<Dataset>,
    /// Incremented whenever the dataset changes, so the UI can cache analytics.
    dataset_revision: u64,
    config_path: PathBuf,
}

impl AppState {
    /// Creates a new state.
    pub fn new(config: Config, config_path: PathBuf) -> Self {
        Self {
            config,
            dataset: None,
            dataset_revision: 0,
            config_path,
        }
    }

    /// Path where the configuration is saved.
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }

    /// Monotonic revision of the dataset.
    pub fn dataset_revision(&self) -> u64 {
        self.dataset_revision
    }

    /// Replaces the dataset and bumps its revision.
    pub fn set_dataset(&mut self, dataset: Option<Dataset>) {
        self.dataset = dataset;
        self.dataset_revision = self.dataset_revision.saturating_add(1);
    }

    /// Saves the current configuration.
    pub fn save_config(&self) -> Result<(), CoreError> {
        self.config.save(&self.config_path)
    }
}
