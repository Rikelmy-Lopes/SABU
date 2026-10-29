//! Command that loads a CSV file into the application state.

use std::path::PathBuf;

use sabu_core::CoreError;
use sabu_core::dataset::Dataset;
use sabu_core::state::AppState;

use crate::commands::Command;

/// Loads a dataset from a CSV file.
pub struct LoadDatasetCommand {
    path: PathBuf,
    previous: Option<Dataset>,
}

impl LoadDatasetCommand {
    /// Creates a command for the given path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            previous: None,
        }
    }
}

impl Command for LoadDatasetCommand {
    fn label_key(&self) -> &'static str {
        "command.load_dataset"
    }

    fn execute(&mut self, state: &mut AppState) -> Result<(), CoreError> {
        let dataset = Dataset::load(&self.path)?;
        self.previous = state.dataset.take();
        state.set_dataset(Some(dataset));
        Ok(())
    }

    fn undo(&mut self, state: &mut AppState) -> Result<(), CoreError> {
        state.set_dataset(self.previous.take());
        Ok(())
    }
}
