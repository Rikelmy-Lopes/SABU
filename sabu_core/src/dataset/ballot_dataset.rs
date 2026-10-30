use std::path::{Path, PathBuf};

use crate::analytics::dataset_analytics::VoteAnalytics;
use crate::dataset::summary::DatasetSummary;
use crate::error::core_error::Result;
use crate::model::bulletin_record::BulletinRecord;
use crate::parser::csv_parser::CsvParser;

/// A parsed bulletin file together with its derived analytics.
#[derive(Debug, Clone)]
pub struct BallotDataset {
    path: PathBuf,
    records: Vec<BulletinRecord>,
    summary: DatasetSummary,
    analytics: VoteAnalytics,
}

impl BallotDataset {
    /// Parses `path` and computes its summary and analytics.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let records = CsvParser::parse_file(&path)?;
        let summary = DatasetSummary::from_records(&records);
        let analytics = VoteAnalytics::from_records(&records);

        log::info!(
            "dataset loaded from {}: {} records",
            path.display(),
            records.len()
        );

        Ok(Self {
            path,
            records,
            summary,
            analytics,
        })
    }

    /// Source file of the dataset.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Parsed rows.
    pub fn records(&self) -> &[BulletinRecord] {
        &self.records
    }

    /// Summary counters.
    pub fn summary(&self) -> &DatasetSummary {
        &self.summary
    }

    /// Aggregated votes.
    pub fn analytics(&self) -> &VoteAnalytics {
        &self.analytics
    }
}
