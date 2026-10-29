//! Application configuration.

mod language;
mod theme;
mod user_config;

pub use language::Language;
pub use theme::Theme;
pub use user_config::Config;

#[cfg(test)]
#[path = "config_tests.rs"]
mod config_tests;
