//! Application state mutated only through commands.

mod app_state;

pub use app_state::AppState;

#[cfg(test)]
#[path = "state_tests.rs"]
mod state_tests;
