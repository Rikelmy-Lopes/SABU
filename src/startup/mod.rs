//! Application bootstrap: logging, configuration, and the GUI event loop.

mod application;
mod logging;
mod startup_error;

pub use application::run;
