//! Errors that prevent the application from starting.

use thiserror::Error;

use crate::commands::DispatcherError;
use crate::i18n::I18nError;

/// Errors that prevent the application from starting.
#[derive(Debug, Error)]
pub(crate) enum StartupError {
    #[error(transparent)]
    Core(#[from] sabu_core::CoreError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("could not initialize logging: {0}")]
    Logging(#[from] log::SetLoggerError),

    #[error(transparent)]
    I18n(#[from] I18nError),

    #[error(transparent)]
    Dispatcher(#[from] DispatcherError),

    #[error(transparent)]
    Ui(#[from] eframe::Error),
}
