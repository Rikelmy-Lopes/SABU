//! Tests for application state.

use std::path::PathBuf;

use crate::config::{Config, Language};
use crate::dataset::Dataset;

use super::AppState;

#[test]
fn starts_without_a_dataset_at_revision_zero() {
    let state = AppState::new(Config::default(), PathBuf::from("config.json"));
    assert!(state.dataset.is_none());
    assert_eq!(state.dataset_revision(), 0);
}

#[test]
fn setting_a_dataset_bumps_the_revision() {
    let mut state = AppState::new(Config::default(), PathBuf::from("config.json"));
    state.set_dataset(Some(Dataset::default()));
    assert_eq!(state.dataset_revision(), 1);
    state.set_dataset(None);
    assert_eq!(state.dataset_revision(), 2);
    assert!(state.dataset.is_none());
}

#[test]
fn saves_the_configuration_to_its_path() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let path = directory.path().join("config.json");
    let mut state = AppState::new(Config::default(), path.clone());
    state.config.language = Language::En;

    assert!(state.save_config().is_ok());
    assert_eq!(Config::load(&path).ok(), Some(state.config));
}
