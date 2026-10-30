use std::path::PathBuf;

use crate::dataset::ballot_dataset::BallotDataset;

fn example_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../example/data.csv")
}

#[test]
fn loads_example_dataset_and_computes_metadata() {
    let dataset = BallotDataset::load(example_path()).unwrap();

    assert!(!dataset.records().is_empty());
    assert_eq!(dataset.summary().record_count, dataset.records().len());

    let votes: u64 = dataset
        .records()
        .iter()
        .map(|record| record.votes.unwrap_or(0).max(0) as u64)
        .sum();

    assert_eq!(dataset.summary().total_votes, votes);
    assert!(!dataset.analytics().by_office.is_empty());
    assert!(!dataset.analytics().by_party.is_empty());
    assert!(!dataset.analytics().by_votable.is_empty());
    assert_eq!(dataset.path(), example_path());
}
