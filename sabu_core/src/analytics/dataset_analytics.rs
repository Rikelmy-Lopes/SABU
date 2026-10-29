//! All vote aggregations computed for a dataset.

use crate::model::BulletinRecord;

use super::VoteGroup;
use super::aggregate::aggregate;

/// Vote totals for the dimensions currently shown by the interface.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DatasetAnalytics {
    /// Total votes per office/question, highest first.
    pub by_cargo: Vec<VoteGroup>,
    /// Total votes per votable, highest first.
    pub by_votavel: Vec<VoteGroup>,
    /// Total votes per party acronym, highest first.
    pub by_partido: Vec<VoteGroup>,
}

impl DatasetAnalytics {
    /// Computes every aggregation for the given records.
    pub fn compute(records: &[BulletinRecord]) -> Self {
        Self {
            by_cargo: aggregate(records, |record| &record.cargo),
            by_votavel: aggregate(records, |record| &record.nm_votavel),
            by_partido: aggregate(records, |record| &record.partido),
        }
    }
}
