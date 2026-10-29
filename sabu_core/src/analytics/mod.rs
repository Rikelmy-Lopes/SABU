//! Aggregations over a dataset.

mod aggregate;
mod dataset_analytics;
mod vote_group;
mod votes_by_cargo;
mod votes_by_partido;
mod votes_by_votavel;

pub use dataset_analytics::DatasetAnalytics;
pub use vote_group::VoteGroup;
pub use votes_by_cargo::votes_by_cargo;
pub use votes_by_partido::votes_by_partido;
pub use votes_by_votavel::votes_by_votavel;

#[cfg(test)]
#[path = "analytics_tests.rs"]
mod analytics_tests;
