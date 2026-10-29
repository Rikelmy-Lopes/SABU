//! Parsing nullable electoral numeric fields.

use crate::error::CoreError;

use super::decode::decode_text;

/// Parses an optional non-negative integer.
///
/// `#NULO`, `#NE`, empty values, and negative numbers are treated as missing.
pub(super) fn parse_optional_number(
    bytes: &[u8],
    column: &'static str,
    line: u64,
) -> Result<Option<u64>, CoreError> {
    let text = decode_text(bytes);
    if text.is_empty() || text.eq_ignore_ascii_case("#NULO") || text.eq_ignore_ascii_case("#NE") {
        return Ok(None);
    }

    match text.parse::<i64>() {
        Ok(value) if value >= 0 => Ok(Some(value as u64)),
        Ok(_) => Ok(None),
        Err(_) => Err(CoreError::InvalidNumber {
            column,
            line,
            value: text,
        }),
    }
}

/// Parses an optional 16-bit integer.
pub(super) fn parse_optional_u16(
    bytes: &[u8],
    column: &'static str,
    line: u64,
) -> Result<Option<u16>, CoreError> {
    match parse_optional_number(bytes, column, line)? {
        Some(value) => match u16::try_from(value) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Err(invalid_range(value, column, line)),
        },
        None => Ok(None),
    }
}

/// Parses an optional 32-bit integer.
pub(super) fn parse_optional_u32(
    bytes: &[u8],
    column: &'static str,
    line: u64,
) -> Result<Option<u32>, CoreError> {
    match parse_optional_number(bytes, column, line)? {
        Some(value) => match u32::try_from(value) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Err(invalid_range(value, column, line)),
        },
        None => Ok(None),
    }
}

fn invalid_range(value: u64, column: &'static str, line: u64) -> CoreError {
    CoreError::InvalidNumber {
        column,
        line,
        value: value.to_string(),
    }
}
