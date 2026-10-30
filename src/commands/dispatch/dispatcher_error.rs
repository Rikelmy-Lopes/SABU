use thiserror::Error;

/// Failures raised by the command dispatcher.
#[derive(Debug, Error)]
pub enum DispatcherError {
    /// The worker thread could not be created.
    #[error("failed to spawn the command worker: {0}")]
    Spawn(String),

    /// The worker is no longer accepting commands.
    #[error("the command worker is not available")]
    Unavailable,
}
