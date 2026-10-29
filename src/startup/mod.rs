//! Application startup orchestration.

mod application;
mod configuration;
mod logging;
mod startup_error;
mod window;

pub(crate) use application::run;
pub(crate) use startup_error::StartupError;
