//! Votes grouped by votable.

use crate::dataset::Dataset;

use super::VoteGroup;

/// Total votes per votable, highest first.
pub fn votes_by_votavel(dataset: &Dataset) -> Vec<VoteGroup> {
    dataset.analytics().by_votavel.clone()
}
