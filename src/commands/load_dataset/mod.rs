//! Loading a dataset from a CSV file.

mod load_command;

pub use load_command::LoadDatasetCommand;

#[cfg(test)]
#[path = "load_dataset_tests.rs"]
mod load_dataset_tests;
