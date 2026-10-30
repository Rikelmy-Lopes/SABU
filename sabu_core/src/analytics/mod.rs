//! Vote aggregation independent of the user interface.

pub mod dataset_analytics;
pub mod vote_group;

pub use dataset_analytics::VoteAnalytics;
pub use vote_group::VoteGroup;

#[cfg(test)]
mod analytics_tests;
