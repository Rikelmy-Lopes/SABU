//! The command trait and its execution context.

mod action;
mod context;
mod error;

pub use action::Command;
pub use context::CommandContext;
pub use error::CommandError;
