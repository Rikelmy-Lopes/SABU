//! Events reported by the command worker.

use sabu_core::CoreError;

/// Result of a command dispatched to the worker.
#[derive(Debug)]
pub enum CommandEvent {
    /// The command finished successfully.
    Completed {
        /// Translation key of the command.
        label_key: &'static str,
    },
    /// The command failed.
    Failed {
        /// Translation key of the command.
        label_key: &'static str,
        /// Error returned by the command.
        error: CoreError,
    },
    /// A command was undone.
    Undone {
        /// Translation key of the undone command.
        label_key: &'static str,
    },
    /// A command was redone.
    Redone {
        /// Translation key of the redone command.
        label_key: &'static str,
    },
    /// There was nothing to undo.
    NothingToUndo,
    /// There was nothing to redo.
    NothingToRedo,
}
