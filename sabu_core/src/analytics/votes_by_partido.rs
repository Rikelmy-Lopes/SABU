//! Votes grouped by party.

use crate::dataset::Dataset;

use super::VoteGroup;

/// Total votes per party acronym, highest first.
pub fn votes_by_partido(dataset: &Dataset) -> Vec<VoteGroup> {
    dataset.analytics().by_partido.clone()
}
