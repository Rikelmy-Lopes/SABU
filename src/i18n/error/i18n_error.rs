use thiserror::Error;

/// Failures raised by the internationalization layer.
#[derive(Debug, Error)]
pub enum I18nError {
    /// A language has no bundled JSON catalog.
    #[error("no bundled translations found for '{0}'")]
    MissingTranslations(&'static str),

    /// A bundled catalog is not valid JSON.
    #[error("failed to parse translations for '{language}': {source}")]
    InvalidTranslations {
        /// Language code of the broken catalog.
        language: &'static str,
        /// Underlying JSON error.
        #[source]
        source: serde_json::Error,
    },
}
