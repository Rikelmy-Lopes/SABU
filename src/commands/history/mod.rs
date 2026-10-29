//! Undo/redo history for executed commands.

mod command_history;

pub use command_history::CommandHistory;

#[cfg(test)]
#[path = "history_tests.rs"]
mod history_tests;
