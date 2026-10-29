//! Votes grouped by office or question.

use crate::dataset::Dataset;

use super::VoteGroup;

/// Total votes per office/question, highest first.
pub fn votes_by_cargo(dataset: &Dataset) -> Vec<VoteGroup> {
    dataset.analytics().by_cargo.clone()
}
