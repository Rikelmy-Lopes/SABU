use std::collections::HashSet;

use crate::model::bulletin_record::BulletinRecord;

/// High level counters shown on the summary panel.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DatasetSummary {
    /// Number of CSV rows.
    pub record_count: usize,
    /// Sum of all valid `QT_VOTOS` values.
    pub total_votes: u64,
    /// Number of distinct ballot boxes.
    pub ballot_box_count: usize,
    /// Number of distinct municipalities.
    pub municipality_count: usize,
}

impl DatasetSummary {
    /// Builds the summary from parsed records.
    pub fn from_records(records: &[BulletinRecord]) -> Self {
        let mut ballot_boxes = HashSet::new();
        let mut municipalities = HashSet::new();
        let mut total_votes = 0_u64;

        for record in records {
            total_votes += record.votes.unwrap_or(0).max(0) as u64;
            ballot_boxes.insert((record.zone, record.section, record.ballot_box_number));
            municipalities.insert(record.municipality_code);
        }

        Self {
            record_count: records.len(),
            total_votes,
            ballot_box_count: ballot_boxes.len(),
            municipality_count: municipalities.len(),
        }
    }
}
