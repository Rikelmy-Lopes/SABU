//! Core business logic for SABU.
//!
//! This crate is independent from any user interface. It owns CSV parsing,
//! configuration, the in-memory dataset, and analytics.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub mod analytics;
pub mod config;
pub mod dataset;
pub mod error;
pub mod model;
pub mod parser;
pub mod paths;
pub mod state;

pub use error::CoreError;
