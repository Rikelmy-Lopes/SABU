//! Tests for the clear dataset command.

use crate::commands::test_support::{sample_path, state_in};
use crate::commands::{ClearDatasetCommand, Command, LoadDatasetCommand};

#[test]
fn clears_and_restores_the_dataset() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let mut state = state_in(&directory);
    let mut load = LoadDatasetCommand::new(sample_path());
    assert!(load.execute(&mut state).is_ok());

    let mut clear = ClearDatasetCommand::new();
    assert!(clear.execute(&mut state).is_ok());
    assert!(state.dataset.is_none());

    assert!(clear.undo(&mut state).is_ok());
    assert!(state.dataset.is_some());
    assert_eq!(clear.label_key(), "command.clear_dataset");
}
