//! CSV reader orchestration.

use std::io::Read;

use csv::ReaderBuilder;

use crate::error::CoreError;
use crate::model::BulletinRecord;

use super::columns::Columns;

/// Delimiter used by the electoral CSV files.
const DELIMITER: u8 = b';';

/// Parses CSV bytes into typed records.
pub fn parse_bytes(bytes: &[u8]) -> Result<Vec<BulletinRecord>, CoreError> {
    parse_reader(bytes)
}

/// Parses CSV data from any reader into typed records.
pub fn parse_reader<R: Read>(reader: R) -> Result<Vec<BulletinRecord>, CoreError> {
    let mut reader = ReaderBuilder::new()
        .delimiter(DELIMITER)
        .flexible(true)
        .has_headers(true)
        .from_reader(reader);

    let headers = reader.byte_headers()?.clone();
    if headers.is_empty() {
        return Err(CoreError::EmptyFile);
    }

    let columns = Columns::from_headers(&headers)?;
    let mut records = Vec::new();

    for (index, record) in reader.byte_records().enumerate() {
        let record = record?;
        let line = index as u64 + 2;
        records.push(columns.parse_record(&record, line)?);
    }

    Ok(records)
}
