//! Clearing the loaded dataset.

mod clear_command;

pub use clear_command::ClearDatasetCommand;

#[cfg(test)]
#[path = "clear_dataset_tests.rs"]
mod clear_dataset_tests;
