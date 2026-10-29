//! Errors raised when dispatching commands.

use thiserror::Error;

/// Errors raised when dispatching commands.
#[derive(Debug, Error)]
pub enum DispatcherError {
    /// The worker thread is not running.
    #[error("the command worker is not running")]
    WorkerStopped,
    /// The worker thread could not be created.
    #[error("could not start the command worker: {0}")]
    Spawn(#[from] std::io::Error),
}
