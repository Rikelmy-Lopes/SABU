use thiserror::Error;

/// Failures raised while a command runs.
#[derive(Debug, Error)]
pub enum CommandError {
    /// The core layer rejected the operation.
    #[error(transparent)]
    Core(#[from] sabu_core::CoreError),
}
