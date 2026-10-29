//! Tests for dataset loading and summary computation.

use std::path::Path;

use crate::model::BulletinRecord;

use super::Dataset;

fn sample_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("example")
        .join("data.csv")
}

fn record(cargo: &str, municipio: &str, votavel: &str, votes: u64) -> BulletinRecord {
    BulletinRecord {
        cargo: cargo.to_owned(),
        municipio: municipio.to_owned(),
        nm_votavel: votavel.to_owned(),
        qt_votos: Some(votes),
        ..BulletinRecord::default()
    }
}

#[test]
fn loads_the_sample_file() {
    let dataset = Dataset::load(&sample_path());
    assert!(dataset.is_ok());
    let Ok(dataset) = dataset else {
        return;
    };

    assert_eq!(dataset.records().len(), 49);
    assert_eq!(dataset.summary().total_records, 49);
    assert!(dataset.summary().total_votes > 0);
    assert_eq!(dataset.summary().municipalities, 1);
    assert_eq!(dataset.summary().cargos, 1);
}

#[test]
fn missing_file_is_an_io_error() {
    let dataset = Dataset::load(&sample_path().with_file_name("missing.csv"));
    assert!(dataset.is_err());
}

#[test]
fn summary_counts_distinct_values_and_votes() {
    let dataset = Dataset::new(vec![
        record("Presidente", "São Paulo", "Maria", 10),
        record("Presidente", "Campinas", "Maria", 5),
        record("Governador", "Campinas", "João", 7),
        BulletinRecord {
            cargo: "Governador".to_owned(),
            municipio: "Campinas".to_owned(),
            nm_votavel: "João".to_owned(),
            qt_votos: None,
            ..BulletinRecord::default()
        },
    ]);

    let summary = dataset.summary();
    assert_eq!(summary.total_records, 4);
    assert_eq!(summary.total_votes, 22);
    assert_eq!(summary.municipalities, 2);
    assert_eq!(summary.cargos, 2);
    assert_eq!(summary.votaveis, 2);
}

#[test]
fn computes_analytics_when_the_dataset_is_built() {
    let dataset = Dataset::new(vec![
        record("Presidente", "São Paulo", "Maria", 10),
        record("Presidente", "Campinas", "Maria", 5),
        record("Governador", "Campinas", "João", 7),
    ]);

    let analytics = dataset.analytics();
    assert_eq!(analytics.by_cargo.len(), 2);
    assert_eq!(analytics.by_cargo[0].name, "Presidente");
    assert_eq!(analytics.by_cargo[0].votes, 15);
    assert_eq!(analytics.by_votavel.len(), 2);
}

#[test]
fn empty_dataset_has_zeroed_summary() {
    let dataset = Dataset::new(Vec::new());
    let summary = dataset.summary();
    assert_eq!(summary, super::DatasetSummary::default());
}
