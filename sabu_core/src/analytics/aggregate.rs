//! Parallel vote aggregation by a selected record field.

use std::collections::HashMap;

use rayon::prelude::*;

use crate::model::BulletinRecord;

use super::VoteGroup;

/// Aggregates votes by the name returned by `selector`.
pub(super) fn aggregate(
    records: &[BulletinRecord],
    selector: fn(&BulletinRecord) -> &str,
) -> Vec<VoteGroup> {
    let totals = records
        .par_iter()
        .fold(HashMap::<String, u64>::new, |mut totals, record| {
            let name = selector(record);
            if !name.is_empty() {
                *totals.entry(name.to_owned()).or_insert(0) += record.qt_votos.unwrap_or(0);
            }
            totals
        })
        .reduce(HashMap::<String, u64>::new, |mut left, right| {
            for (name, votes) in right {
                *left.entry(name).or_insert(0) += votes;
            }
            left
        });

    let mut groups = totals
        .into_iter()
        .map(|(name, votes)| VoteGroup { name, votes })
        .collect::<Vec<_>>();
    groups.sort_by(|left, right| {
        right
            .votes
            .cmp(&left.votes)
            .then_with(|| left.name.cmp(&right.name))
    });
    groups
}
