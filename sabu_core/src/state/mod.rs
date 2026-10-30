//! Application state owned by the core layer.

pub mod app_state;

pub use app_state::AppState;

#[cfg(test)]
mod state_tests;
