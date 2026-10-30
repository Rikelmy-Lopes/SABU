use std::collections::HashMap;

use rayon::prelude::*;

use crate::analytics::vote_group::VoteGroup;
use crate::model::bulletin_record::BulletinRecord;

/// Vote totals grouped by the dimensions shown in the interface.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VoteAnalytics {
    /// Votes grouped by office (cargo).
    pub by_office: Vec<VoteGroup>,
    /// Votes grouped by party.
    pub by_party: Vec<VoteGroup>,
    /// Votes grouped by votable (candidate, blank vote, null vote, and so on).
    pub by_votable: Vec<VoteGroup>,
}

impl VoteAnalytics {
    /// Aggregates records in parallel.
    pub fn from_records(records: &[BulletinRecord]) -> Self {
        Self {
            by_office: aggregate(records, office_key),
            by_party: aggregate(records, party_key),
            by_votable: aggregate(records, votable_key),
        }
    }
}

fn aggregate(
    records: &[BulletinRecord],
    key_of: fn(&BulletinRecord) -> Option<&str>,
) -> Vec<VoteGroup> {
    let totals = records
        .par_iter()
        .fold(HashMap::<String, u64>::new, |mut totals, record| {
            if let Some(key) = key_of(record)
                && let Some(votes) = record.votes
            {
                *totals.entry(key.to_owned()).or_insert(0) += votes.max(0) as u64;
            }

            totals
        })
        .reduce(HashMap::<String, u64>::new, |mut left, right| {
            for (key, votes) in right {
                *left.entry(key).or_insert(0) += votes;
            }

            left
        });

    let mut groups: Vec<VoteGroup> = totals
        .into_iter()
        .map(|(key, votes)| VoteGroup { key, votes })
        .collect();

    groups.sort_by(|left, right| {
        right
            .votes
            .cmp(&left.votes)
            .then_with(|| left.key.cmp(&right.key))
    });

    groups
}

fn office_key(record: &BulletinRecord) -> Option<&str> {
    non_empty(&record.office_name)
}

fn party_key(record: &BulletinRecord) -> Option<&str> {
    non_empty(&record.party_acronym)
}

fn votable_key(record: &BulletinRecord) -> Option<&str> {
    non_empty(&record.votable_name)
}

fn non_empty(value: &str) -> Option<&str> {
    if value.is_empty() { None } else { Some(value) }
}
