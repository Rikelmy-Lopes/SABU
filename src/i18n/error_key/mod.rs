//! Mapping core errors to translation keys.

mod error_translation;

pub use error_translation::error_key;

#[cfg(test)]
#[path = "error_key_tests.rs"]
mod error_key_tests;
