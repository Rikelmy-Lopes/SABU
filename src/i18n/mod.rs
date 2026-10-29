//! Internationalization support.

mod dictionary;
mod error;
mod error_key;
mod language_catalog;
mod number_formatter;
mod translator;

pub use error::I18nError;
pub use error_key::error_key;
pub use translator::Translator;
