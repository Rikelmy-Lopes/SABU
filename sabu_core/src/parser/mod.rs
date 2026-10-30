//! CSV parsing for ballot box bulletin files.

pub mod columns;
pub mod csv_parser;
pub mod decode;
pub mod numbers;
pub mod record_mapper;

pub use csv_parser::CsvParser;

#[cfg(test)]
mod parser_tests;
