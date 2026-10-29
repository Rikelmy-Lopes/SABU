//! Aggregated counters for a dataset.

use std::collections::HashSet;

use crate::model::BulletinRecord;

/// Aggregated counters for a dataset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DatasetSummary {
    /// Number of parsed rows.
    pub total_records: usize,
    /// Sum of `QT_VOTOS` across all rows.
    pub total_votes: u64,
    /// Number of distinct municipalities.
    pub municipalities: usize,
    /// Number of distinct offices or questions.
    pub cargos: usize,
    /// Number of distinct votables.
    pub votaveis: usize,
}

impl DatasetSummary {
    /// Computes the summary for a list of records.
    pub(crate) fn from_records(records: &[BulletinRecord]) -> Self {
        let municipalities = records
            .iter()
            .map(|record| record.municipio.as_str())
            .collect::<HashSet<_>>()
            .len();
        let cargos = records
            .iter()
            .map(|record| record.cargo.as_str())
            .collect::<HashSet<_>>()
            .len();
        let votaveis = records
            .iter()
            .map(|record| record.nm_votavel.as_str())
            .collect::<HashSet<_>>()
            .len();

        Self {
            total_records: records.len(),
            total_votes: records.iter().filter_map(|record| record.qt_votos).sum(),
            municipalities,
            cargos,
            votaveis,
        }
    }
}
