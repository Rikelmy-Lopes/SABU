use crate::commands::command::{Command, CommandContext, CommandError};

/// Stable id of the command that unloads the active dataset.
pub const CLEAR_DATASET: &str = "dataset.clear";

/// Removes the active dataset from the application state.
#[derive(Debug, Default)]
pub struct ClearDatasetCommand;

impl ClearDatasetCommand {
    /// Creates the command.
    pub fn new() -> Self {
        Self
    }
}

impl Command for ClearDatasetCommand {
    fn id(&self) -> &'static str {
        CLEAR_DATASET
    }

    fn execute(self: Box<Self>, context: &mut CommandContext<'_>) -> Result<(), CommandError> {
        context.state.clear_dataset();

        Ok(())
    }
}
