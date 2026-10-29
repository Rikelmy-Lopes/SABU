//! Parsing ballot box bulletin CSV files.

mod columns;
mod csv_parser;
mod decode;
mod numbers;

pub use csv_parser::{parse_bytes, parse_reader};

#[cfg(test)]
#[path = "parser_tests.rs"]
mod parser_tests;
