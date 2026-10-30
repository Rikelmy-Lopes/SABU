use std::path::PathBuf;

use sabu_core::dataset::ballot_dataset::BallotDataset;

use crate::commands::command::{Command, CommandContext, CommandError};

/// Stable id of the command that loads a CSV file.
pub const LOAD_DATASET: &str = "dataset.load";

/// Parses a CSV file in the background and replaces the active dataset.
pub struct LoadDatasetCommand {
    path: PathBuf,
}

impl LoadDatasetCommand {
    /// Creates the command for `path`.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl Command for LoadDatasetCommand {
    fn id(&self) -> &'static str {
        LOAD_DATASET
    }

    fn execute(self: Box<Self>, context: &mut CommandContext<'_>) -> Result<(), CommandError> {
        let dataset = BallotDataset::load(&self.path)?;

        context.state.replace_dataset(dataset);

        Ok(())
    }
}
