//! GUI independent business logic for SABU.
//!
//! This crate owns CSV parsing, domain models, analytics, state, configuration,
//! and filesystem locations. It must never depend on `egui` or `eframe`.

pub mod analytics;
pub mod config;
pub mod dataset;
pub mod error;
pub mod model;
pub mod parser;
pub mod paths;
pub mod state;

pub use dataset::ballot_dataset::BallotDataset;
pub use error::core_error::{CoreError, Result};
pub use model::bulletin_record::BulletinRecord;
pub use parser::csv_parser::CsvParser;
pub use state::app_state::AppState;
