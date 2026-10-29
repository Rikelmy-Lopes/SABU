//! Command that clears the loaded dataset.

use sabu_core::CoreError;
use sabu_core::dataset::Dataset;
use sabu_core::state::AppState;

use crate::commands::Command;

/// Removes the current dataset, keeping it for undo.
#[derive(Default)]
pub struct ClearDatasetCommand {
    previous: Option<Dataset>,
}

impl ClearDatasetCommand {
    /// Creates the command.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Command for ClearDatasetCommand {
    fn label_key(&self) -> &'static str {
        "command.clear_dataset"
    }

    fn execute(&mut self, state: &mut AppState) -> Result<(), CoreError> {
        self.previous = state.dataset.take();
        state.set_dataset(None);
        Ok(())
    }

    fn undo(&mut self, state: &mut AppState) -> Result<(), CoreError> {
        if let Some(previous) = self.previous.take() {
            state.set_dataset(Some(previous));
        }
        Ok(())
    }
}
