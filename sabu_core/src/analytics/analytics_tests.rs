//! Tests for vote aggregations.

use crate::dataset::Dataset;
use crate::model::BulletinRecord;

use super::{VoteGroup, votes_by_cargo, votes_by_partido, votes_by_votavel};

fn record(cargo: &str, partido: &str, votavel: &str, votes: Option<u64>) -> BulletinRecord {
    BulletinRecord {
        cargo: cargo.to_owned(),
        partido: partido.to_owned(),
        nm_votavel: votavel.to_owned(),
        qt_votos: votes,
        ..BulletinRecord::default()
    }
}

fn dataset() -> Dataset {
    Dataset::new(vec![
        record("Presidente", "PT", "Maria", Some(10)),
        record("Presidente", "PSDB", "João", Some(30)),
        record("Governador", "PT", "Ana", Some(20)),
        record("Governador", "PT", "", Some(100)),
    ])
}

#[test]
fn aggregates_votes_by_cargo_in_descending_order() {
    let groups = votes_by_cargo(&dataset());
    assert_eq!(
        groups,
        vec![
            VoteGroup {
                name: "Governador".to_owned(),
                votes: 120,
            },
            VoteGroup {
                name: "Presidente".to_owned(),
                votes: 40,
            },
        ]
    );
}

#[test]
fn aggregates_votes_by_votable_and_ignores_empty_names() {
    let groups = votes_by_votavel(&dataset());
    assert_eq!(
        groups,
        vec![
            VoteGroup {
                name: "João".to_owned(),
                votes: 30,
            },
            VoteGroup {
                name: "Ana".to_owned(),
                votes: 20,
            },
            VoteGroup {
                name: "Maria".to_owned(),
                votes: 10,
            },
        ]
    );
}

#[test]
fn aggregates_votes_by_party() {
    let groups = votes_by_partido(&dataset());
    assert_eq!(
        groups,
        vec![
            VoteGroup {
                name: "PT".to_owned(),
                votes: 130,
            },
            VoteGroup {
                name: "PSDB".to_owned(),
                votes: 30,
            },
        ]
    );
}

#[test]
fn missing_votes_are_counted_as_zero() {
    let dataset = Dataset::new(vec![record("Presidente", "PT", "Maria", None)]);
    let groups = votes_by_cargo(&dataset);
    assert_eq!(groups[0].votes, 0);
}

#[test]
fn empty_dataset_produces_no_groups() {
    let dataset = Dataset::new(Vec::new());
    assert!(votes_by_cargo(&dataset).is_empty());
    assert!(votes_by_votavel(&dataset).is_empty());
    assert!(votes_by_partido(&dataset).is_empty());
}

#[test]
fn ties_are_sorted_by_name() {
    let dataset = Dataset::new(vec![
        record("Presidente", "PT", "Zeca", Some(10)),
        record("Presidente", "PT", "Ana", Some(10)),
    ]);
    let groups = votes_by_votavel(&dataset);
    assert_eq!(groups[0].name, "Ana");
    assert_eq!(groups[1].name, "Zeca");
}
