//! Changing and persisting the active language.

mod language_command;

pub use language_command::SetLanguageCommand;

#[cfg(test)]
#[path = "set_language_tests.rs"]
mod set_language_tests;
