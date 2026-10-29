//! Tests for the load dataset command.

use crate::commands::test_support::{sample_path, state_in};
use crate::commands::{Command, LoadDatasetCommand};

#[test]
fn loads_a_dataset() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let mut state = state_in(&directory);
    let mut command = LoadDatasetCommand::new(sample_path());

    assert!(command.execute(&mut state).is_ok());
    assert!(state.dataset.is_some());
    assert_eq!(command.label_key(), "command.load_dataset");
}

#[test]
fn a_failed_load_keeps_the_previous_dataset() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let mut state = state_in(&directory);
    let mut load = LoadDatasetCommand::new(sample_path());
    assert!(load.execute(&mut state).is_ok());

    let mut missing = LoadDatasetCommand::new(directory.path().join("missing.csv"));
    assert!(missing.execute(&mut state).is_err());
    assert!(state.dataset.is_some());
}

#[test]
fn undo_restores_the_previous_dataset() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let mut state = state_in(&directory);
    let mut command = LoadDatasetCommand::new(sample_path());
    assert!(command.execute(&mut state).is_ok());

    assert!(command.undo(&mut state).is_ok());
    assert!(state.dataset.is_none());
}
