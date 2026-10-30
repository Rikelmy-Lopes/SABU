use thiserror::Error;

/// Failures raised while starting the application.
#[derive(Debug, Error)]
pub enum StartupError {
    /// The core layer failed.
    #[error(transparent)]
    Core(#[from] sabu_core::CoreError),

    /// Translations could not be loaded.
    #[error(transparent)]
    I18n(#[from] crate::i18n::error::I18nError),

    /// The command worker could not start.
    #[error(transparent)]
    Dispatcher(#[from] crate::commands::dispatch::DispatcherError),

    /// Logging could not be initialized.
    #[error("failed to initialize logging: {0}")]
    Logging(String),

    /// The graphical interface failed.
    #[error("failed to start the graphical interface: {0}")]
    Eframe(#[from] eframe::Error),
}
