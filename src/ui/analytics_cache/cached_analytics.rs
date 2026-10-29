//! Cached summary and analytics for one dataset revision.

use sabu_core::dataset::DatasetSummary;

use super::TableCache;

/// UI copy of the dataset analytics, refreshed only when the revision changes.
pub(crate) struct AnalyticsCache {
    pub(crate) revision: u64,
    pub(crate) summary: DatasetSummary,
    pub(crate) by_cargo: TableCache,
    pub(crate) by_votavel: TableCache,
    pub(crate) by_partido: TableCache,
}
