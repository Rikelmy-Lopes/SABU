//! Tests for the set theme command.

use sabu_core::config::{Config, Theme};

use crate::commands::test_support::state_in;
use crate::commands::{Command, SetThemeCommand};

#[test]
fn changes_and_persists_the_theme() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let mut state = state_in(&directory);
    let config_path = state.config_path().to_path_buf();
    let mut command = SetThemeCommand::new(Theme::Light);

    assert!(command.execute(&mut state).is_ok());
    assert_eq!(state.config.theme, Theme::Light);
    assert_eq!(
        Config::load(&config_path).ok().map(|config| config.theme),
        Some(Theme::Light)
    );
    assert_eq!(command.label_key(), "command.set_theme");
}

#[test]
fn undo_restores_the_previous_theme() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let mut state = state_in(&directory);
    let mut command = SetThemeCommand::new(Theme::Light);

    assert!(command.execute(&mut state).is_ok());
    assert!(command.undo(&mut state).is_ok());
    assert_eq!(state.config.theme, Theme::Dark);
}
