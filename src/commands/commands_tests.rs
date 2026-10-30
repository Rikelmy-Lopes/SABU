use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use sabu_core::config::config_store::ConfigStore;
use sabu_core::config::theme::Theme;
use sabu_core::state::app_state::AppState;
use tempfile::TempDir;

use crate::commands::command::Command;
use crate::commands::dispatch::{CommandDispatcher, CommandEvent};
use crate::commands::{
    ClearDatasetCommand, LoadDatasetCommand, SetLanguageCommand, SetThemeCommand,
};

use crate::commands::clear_dataset::clear_command::CLEAR_DATASET;
use crate::commands::load_dataset::load_command::LOAD_DATASET;
use crate::commands::set_language::language_command::SET_LANGUAGE;
use crate::commands::set_theme::theme_command::SET_THEME;

fn start_dispatcher() -> (CommandDispatcher, ConfigStore, TempDir) {
    let dir = tempfile::tempdir().expect("temporary directory");
    let store = ConfigStore::new(dir.path().join("Sabu/config.json"));
    let dispatcher =
        CommandDispatcher::spawn(AppState::default(), store.clone()).expect("dispatcher");

    (dispatcher, store, dir)
}

fn wait_for_completion(dispatcher: &CommandDispatcher) -> CommandEvent {
    let deadline = Instant::now() + Duration::from_secs(5);

    loop {
        while let Some(event) = dispatcher.try_recv() {
            if matches!(
                event,
                CommandEvent::Succeeded { .. } | CommandEvent::Failed { .. }
            ) {
                return event;
            }
        }

        assert!(Instant::now() < deadline, "command timed out");

        thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn command_ids_are_stable() {
    assert_eq!(LOAD_DATASET, "dataset.load");
    assert_eq!(CLEAR_DATASET, "dataset.clear");
    assert_eq!(SET_THEME, "settings.set_theme");
    assert_eq!(SET_LANGUAGE, "settings.set_language");

    assert_eq!(LoadDatasetCommand::new("data.csv").id(), LOAD_DATASET);
    assert_eq!(ClearDatasetCommand::new().id(), CLEAR_DATASET);
    assert_eq!(SetThemeCommand::new(Theme::Light).id(), SET_THEME);
    assert_eq!(
        SetLanguageCommand::new(sabu_core::config::Language::PtBr).id(),
        SET_LANGUAGE
    );
}

#[test]
fn theme_command_updates_state_and_persists_configuration() {
    let (dispatcher, store, _dir) = start_dispatcher();

    dispatcher
        .dispatch(Box::new(SetThemeCommand::new(Theme::Light)))
        .expect("dispatch");

    let event = wait_for_completion(&dispatcher);

    match event {
        CommandEvent::Succeeded { id, state } => {
            assert_eq!(id, SET_THEME);
            assert_eq!(state.config().theme, Theme::Light);
        }
        other => panic!("unexpected event: {other:?}"),
    }

    assert_eq!(store.load().expect("config").theme, Theme::Light);
}

#[test]
fn loads_and_clears_the_example_dataset() {
    let (dispatcher, _store, _dir) = start_dispatcher();
    let example = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("example/data.csv");

    dispatcher
        .dispatch(Box::new(LoadDatasetCommand::new(example)))
        .expect("dispatch load");

    match wait_for_completion(&dispatcher) {
        CommandEvent::Succeeded { id, state } => {
            assert_eq!(id, LOAD_DATASET);
            assert!(state.has_dataset());
            assert!(state.dataset().expect("dataset").summary().record_count > 0);
        }
        other => panic!("unexpected event: {other:?}"),
    }

    dispatcher
        .dispatch(Box::new(ClearDatasetCommand::new()))
        .expect("dispatch clear");

    match wait_for_completion(&dispatcher) {
        CommandEvent::Succeeded { id, state } => {
            assert_eq!(id, CLEAR_DATASET);
            assert!(!state.has_dataset());
        }
        other => panic!("unexpected event: {other:?}"),
    }
}

#[test]
fn failed_command_publishes_the_error() {
    let (dispatcher, _store, dir) = start_dispatcher();
    let missing = dir.path().join("missing.csv");

    dispatcher
        .dispatch(Box::new(LoadDatasetCommand::new(missing)))
        .expect("dispatch");

    match wait_for_completion(&dispatcher) {
        CommandEvent::Failed { id, message, state } => {
            assert_eq!(id, LOAD_DATASET);
            assert!(!message.is_empty());
            assert!(!state.has_dataset());
        }
        other => panic!("unexpected event: {other:?}"),
    }
}
