use std::sync::Arc;

use sabu_core::state::AppState;

/// Notifications published by the worker thread.
#[derive(Debug)]
pub enum CommandEvent {
    /// A command started running.
    Started,
    /// A command finished successfully.
    Succeeded {
        /// Command id.
        id: &'static str,
        /// State snapshot after the command.
        state: Arc<AppState>,
    },
    /// A command failed.
    Failed {
        /// Command id.
        id: &'static str,
        /// Technical error message.
        message: String,
        /// State snapshot after the failure.
        state: Arc<AppState>,
    },
}
