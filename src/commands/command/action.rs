//! A user action that can be executed and undone.

use sabu_core::CoreError;
use sabu_core::state::AppState;

/// A user action that can be executed and undone.
pub trait Command: Send {
    /// Translation key describing the action.
    fn label_key(&self) -> &'static str;

    /// Applies the action to the application state.
    fn execute(&mut self, state: &mut AppState) -> Result<(), CoreError>;

    /// Reverts the last execution of this command.
    fn undo(&mut self, state: &mut AppState) -> Result<(), CoreError>;
}
