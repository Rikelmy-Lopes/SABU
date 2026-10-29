//! Parsed ballot box bulletins.

mod ballot_dataset;
mod summary;

pub use ballot_dataset::Dataset;
pub use summary::DatasetSummary;

#[cfg(test)]
#[path = "dataset_tests.rs"]
mod dataset_tests;
