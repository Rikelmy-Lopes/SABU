//! Changing and persisting the visual theme.

mod theme_command;

pub use theme_command::SetThemeCommand;

#[cfg(test)]
#[path = "set_theme_tests.rs"]
mod set_theme_tests;
