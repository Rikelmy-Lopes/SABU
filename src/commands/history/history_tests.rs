//! Tests for the undo/redo history.

use sabu_core::config::Language;

use crate::commands::test_support::{sample_path, state_in};
use crate::commands::{
    ClearDatasetCommand, CommandHistory, LoadDatasetCommand, SetLanguageCommand,
};

#[test]
fn loads_a_dataset_and_supports_undo_and_redo() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let mut state = state_in(&directory);
    let mut history = CommandHistory::new();

    let command = Box::new(LoadDatasetCommand::new(sample_path()));
    assert!(history.execute(command, &mut state).is_ok());
    assert!(state.dataset.is_some());
    let revision_after_load = state.dataset_revision();

    let undone = history.undo(&mut state);
    assert_eq!(undone.ok().flatten(), Some("command.load_dataset"));
    assert!(state.dataset.is_none());
    assert!(state.dataset_revision() > revision_after_load);

    let redone = history.redo(&mut state);
    assert_eq!(redone.ok().flatten(), Some("command.load_dataset"));
    assert!(state.dataset.is_some());
}

#[test]
fn failed_commands_are_not_recorded() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let mut state = state_in(&directory);
    let mut history = CommandHistory::new();

    let command = Box::new(LoadDatasetCommand::new(
        directory.path().join("missing.csv"),
    ));
    assert!(history.execute(command, &mut state).is_err());
    assert!(state.dataset.is_none());
    assert_eq!(history.undo(&mut state).ok().flatten(), None);
}

#[test]
fn clearing_data_can_be_undone() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let mut state = state_in(&directory);
    let mut history = CommandHistory::new();

    assert!(
        history
            .execute(Box::new(LoadDatasetCommand::new(sample_path())), &mut state)
            .is_ok()
    );
    assert!(
        history
            .execute(Box::new(ClearDatasetCommand::new()), &mut state)
            .is_ok()
    );
    assert!(state.dataset.is_none());

    assert!(history.undo(&mut state).is_ok());
    assert!(state.dataset.is_some());
}

#[test]
fn the_redo_stack_is_cleared_by_a_new_command() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let mut state = state_in(&directory);
    let mut history = CommandHistory::new();

    assert!(
        history
            .execute(Box::new(SetLanguageCommand::new(Language::En)), &mut state)
            .is_ok()
    );
    assert!(history.undo(&mut state).is_ok());
    assert!(
        history
            .execute(Box::new(SetLanguageCommand::new(Language::En)), &mut state)
            .is_ok()
    );
    assert_eq!(history.redo(&mut state).ok().flatten(), None);
}
