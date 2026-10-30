//! User configuration and its persistence.

pub mod config_store;
pub mod language;
pub mod theme;
pub mod user_config;

pub use config_store::ConfigStore;
pub use language::Language;
pub use theme::Theme;
pub use user_config::UserConfig;

#[cfg(test)]
mod config_tests;
