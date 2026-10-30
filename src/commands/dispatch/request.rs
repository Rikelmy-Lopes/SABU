use crate::commands::command::Command;

/// Message sent from the GUI to the worker.
pub(crate) struct CommandRequest {
    pub command: Box<dyn Command>,
}
