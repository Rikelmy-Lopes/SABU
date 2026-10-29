//! Command pattern implementation.
//!
//! The UI never mutates core state directly. It dispatches [`Command`] values
//! to a worker thread, which owns a [`CommandHistory`].

mod clear_dataset;
mod command;
mod dispatch;
mod history;
mod load_dataset;
mod set_language;
mod set_theme;

pub use clear_dataset::ClearDatasetCommand;
pub use command::Command;
pub use dispatch::{CommandDispatcher, CommandEvent, DispatcherError};
pub use history::CommandHistory;
pub use load_dataset::LoadDatasetCommand;
pub use set_language::SetLanguageCommand;
pub use set_theme::SetThemeCommand;

#[cfg(test)]
pub(crate) mod test_support;
