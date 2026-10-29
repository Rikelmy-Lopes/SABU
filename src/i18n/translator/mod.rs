//! Translation lookup with a Portuguese fallback.

mod text_resolver;

pub use text_resolver::Translator;

#[cfg(test)]
#[path = "translator_tests.rs"]
mod translator_tests;
