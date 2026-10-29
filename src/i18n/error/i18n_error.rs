//! Errors raised while loading translation files.

use thiserror::Error;

/// Errors raised while loading translation files.
#[derive(Debug, Error)]
pub enum I18nError {
    /// A translation file is not valid JSON or not a string map.
    #[error("invalid translation file: {0}")]
    Json(#[from] serde_json::Error),
}
