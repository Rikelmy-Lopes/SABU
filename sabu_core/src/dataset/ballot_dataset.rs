//! The in-memory dataset.

use std::path::Path;

use crate::analytics::DatasetAnalytics;
use crate::error::CoreError;
use crate::model::BulletinRecord;
use crate::parser;

use super::DatasetSummary;

/// Parsed ballot box bulletins plus their summary and analytics.
#[derive(Debug, Clone, Default)]
pub struct Dataset {
    records: Vec<BulletinRecord>,
    summary: DatasetSummary,
    analytics: DatasetAnalytics,
}

impl Dataset {
    /// Builds a dataset and computes its summary and analytics.
    pub fn new(records: Vec<BulletinRecord>) -> Self {
        let summary = DatasetSummary::from_records(&records);
        let analytics = DatasetAnalytics::compute(&records);
        Self {
            records,
            summary,
            analytics,
        }
    }

    /// Parsed records.
    pub fn records(&self) -> &[BulletinRecord] {
        &self.records
    }

    /// Precomputed summary.
    pub fn summary(&self) -> DatasetSummary {
        self.summary
    }

    /// Precomputed vote aggregations.
    pub fn analytics(&self) -> &DatasetAnalytics {
        &self.analytics
    }

    /// Loads and parses a CSV file from disk.
    pub fn load(path: &Path) -> Result<Self, CoreError> {
        let file = std::fs::File::open(path)?;
        let records = parser::parse_reader(file)?;
        Ok(Self::new(records))
    }
}
