use std::fs;
use std::path::Path;

use crate::error::core_error::{CoreError, Result};
use crate::model::bulletin_record::BulletinRecord;
use crate::parser::columns::EXPECTED_HEADERS;
use crate::parser::decode;
use crate::parser::record_mapper;

/// Stateless parser for ballot box bulletin CSV files.
#[derive(Debug, Default, Clone, Copy)]
pub struct CsvParser;

impl CsvParser {
    /// Reads and parses a file from disk.
    pub fn parse_file(path: &Path) -> Result<Vec<BulletinRecord>> {
        let bytes = fs::read(path)?;
        Self::parse_bytes(&bytes)
    }

    /// Parses an in-memory Latin-1 document.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Vec<BulletinRecord>> {
        let text = decode::latin1_to_utf8(bytes)?;
        Self::parse_str(&text)
    }

    /// Parses an in-memory UTF-8 document that uses `;` as separator.
    pub fn parse_str(text: &str) -> Result<Vec<BulletinRecord>> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(true)
            .flexible(true)
            .from_reader(text.as_bytes());

        validate_headers(reader.headers()?)?;

        let mut records = Vec::new();

        for row in reader.records() {
            let row = row?;
            records.push(record_mapper::map_record(&row)?);
        }

        Ok(records)
    }
}

fn validate_headers(headers: &csv::StringRecord) -> Result<()> {
    for (index, expected) in EXPECTED_HEADERS.iter().enumerate() {
        let found = headers.get(index).unwrap_or_default().trim();

        if found != *expected {
            return Err(CoreError::InvalidHeader {
                index,
                expected,
                found: found.to_owned(),
            });
        }
    }

    Ok(())
}
