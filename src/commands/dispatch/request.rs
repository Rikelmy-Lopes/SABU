//! Requests sent from the UI to the command worker.

use crate::commands::Command;

/// A message sent to the worker thread.
pub(super) enum Request {
    Execute(Box<dyn Command>),
    Undo,
    Redo,
    Shutdown,
}
