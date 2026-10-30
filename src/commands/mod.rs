//! Command pattern used by the GUI to mutate state.

pub mod clear_dataset;
pub mod command;
pub mod dispatch;
pub mod load_dataset;
pub mod set_language;
pub mod set_theme;

pub use clear_dataset::ClearDatasetCommand;
pub use load_dataset::LoadDatasetCommand;
pub use set_language::SetLanguageCommand;
pub use set_theme::SetThemeCommand;

#[cfg(test)]
mod commands_tests;
