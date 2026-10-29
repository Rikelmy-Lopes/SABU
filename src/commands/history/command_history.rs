//! Undo/redo stacks for executed commands.

use sabu_core::CoreError;
use sabu_core::state::AppState;

use crate::commands::Command;

/// Keeps executed commands so they can be undone and redone.
#[derive(Default)]
pub struct CommandHistory {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}

impl CommandHistory {
    /// Creates an empty history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Executes a command and records it.
    pub fn execute(
        &mut self,
        mut command: Box<dyn Command>,
        state: &mut AppState,
    ) -> Result<(), CoreError> {
        command.execute(state)?;
        self.undo_stack.push(command);
        self.redo_stack.clear();
        Ok(())
    }

    /// Undoes the last command and returns its label key.
    pub fn undo(&mut self, state: &mut AppState) -> Result<Option<&'static str>, CoreError> {
        let Some(mut command) = self.undo_stack.pop() else {
            return Ok(None);
        };

        let label_key = command.label_key();
        match command.undo(state) {
            Ok(()) => {
                self.redo_stack.push(command);
                Ok(Some(label_key))
            }
            Err(error) => {
                self.undo_stack.push(command);
                Err(error)
            }
        }
    }

    /// Re-executes the last undone command and returns its label key.
    pub fn redo(&mut self, state: &mut AppState) -> Result<Option<&'static str>, CoreError> {
        let Some(mut command) = self.redo_stack.pop() else {
            return Ok(None);
        };

        let label_key = command.label_key();
        match command.execute(state) {
            Ok(()) => {
                self.undo_stack.push(command);
                Ok(Some(label_key))
            }
            Err(error) => {
                self.redo_stack.push(command);
                Err(error)
            }
        }
    }
}
