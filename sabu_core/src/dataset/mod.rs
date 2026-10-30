//! Datasets loaded by the application.

pub mod ballot_dataset;
pub mod summary;

pub use ballot_dataset::BallotDataset;
pub use summary::DatasetSummary;

#[cfg(test)]
mod dataset_tests;
