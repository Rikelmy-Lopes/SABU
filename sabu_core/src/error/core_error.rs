//! Error type shared by the core modules.

use thiserror::Error;

/// Errors that can happen inside `sabu_core`.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Reading or writing a file failed.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The CSV reader reported malformed input.
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    /// A JSON document could not be parsed or serialized.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// The CSV file has no header row.
    #[error("the CSV file is empty")]
    EmptyFile,

    /// One or more mandatory columns are missing from the header.
    #[error("missing required columns: {columns}")]
    MissingColumns {
        /// Comma separated list of missing column names.
        columns: String,
    },

    /// A CSV row does not contain all mandatory columns.
    #[error("line {line} has {found} fields, but at least {expected} are required")]
    InvalidRecordWidth {
        /// Line number in the CSV file, starting at 1.
        line: u64,
        /// Number of fields found in the record.
        found: usize,
        /// Number of fields required by the header mapping.
        expected: usize,
    },

    /// A numeric column contains a value that is not a number.
    #[error("invalid number in column {column} on line {line}: {value}")]
    InvalidNumber {
        /// Name of the invalid column.
        column: &'static str,
        /// Line number in the CSV file, starting at 1.
        line: u64,
        /// The raw value that failed to parse.
        value: String,
    },

    /// The `%LOCALAPPDATA%` environment variable is not set.
    #[error("the LOCALAPPDATA environment variable is not set")]
    MissingLocalAppData,

    /// A locale code is not supported by the application.
    #[error("unsupported language code: {0}")]
    InvalidLanguage(String),

    /// The shared application state mutex was poisoned.
    #[error("the application state lock is poisoned")]
    StatePoisoned,
}
