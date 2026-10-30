use std::io;

use thiserror::Error;

/// Result alias used by the core crate.
pub type Result<T> = std::result::Result<T, CoreError>;

/// Every failure that can happen inside the core layer.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Reading a file from disk failed.
    #[error("failed to read file: {0}")]
    Io(#[from] io::Error),

    /// The CSV data could not be decoded.
    #[error("failed to decode Latin-1 data: {0}")]
    Decode(String),

    /// The CSV reader reported a malformed document.
    #[error("failed to parse CSV: {0}")]
    Csv(#[from] csv::Error),

    /// A CSV header does not match the expected column layout.
    #[error("invalid CSV header at column {index}: expected '{expected}', found '{found}'")]
    InvalidHeader {
        /// Zero based column index.
        index: usize,
        /// Expected header name.
        expected: &'static str,
        /// Header name found in the file.
        found: String,
    },

    /// A row has fewer fields than the expected layout.
    #[error("missing column '{column}' on line {line}")]
    MissingColumn {
        /// Name of the missing column.
        column: &'static str,
        /// One based CSV line number.
        line: u64,
    },

    /// A numeric field contains a value that cannot be parsed.
    #[error("invalid numeric value '{value}' in column '{column}'")]
    InvalidNumber {
        /// CSV column name.
        column: &'static str,
        /// Raw value found in the file.
        value: String,
    },

    /// The configuration file is invalid or cannot be written.
    #[error("failed to handle configuration: {0}")]
    Configuration(String),

    /// The operating system does not expose a local application data folder.
    #[error("application data directory is not available")]
    DataDirectoryUnavailable,
}
