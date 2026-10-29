//! Tests for the set language command.

use sabu_core::config::{Config, Language};

use crate::commands::test_support::state_in;
use crate::commands::{Command, SetLanguageCommand};

#[test]
fn changes_and_persists_the_language() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let mut state = state_in(&directory);
    let config_path = state.config_path().to_path_buf();
    let mut command = SetLanguageCommand::new(Language::En);

    assert!(command.execute(&mut state).is_ok());
    assert_eq!(state.config.language, Language::En);
    assert_eq!(
        Config::load(&config_path)
            .ok()
            .map(|config| config.language),
        Some(Language::En)
    );
    assert_eq!(command.label_key(), "command.set_language");
}

#[test]
fn undo_restores_the_previous_language() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let mut state = state_in(&directory);
    let config_path = state.config_path().to_path_buf();
    let mut command = SetLanguageCommand::new(Language::En);

    assert!(command.execute(&mut state).is_ok());
    assert!(command.undo(&mut state).is_ok());
    assert_eq!(state.config.language, Language::PtBr);
    assert_eq!(
        Config::load(&config_path)
            .ok()
            .map(|config| config.language),
        Some(Language::PtBr)
    );
}
