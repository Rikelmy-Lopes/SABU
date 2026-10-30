use crate::error::core_error::{CoreError, Result};

/// Numeric representation of `#NULO`.
pub const NULL_NUMERIC: i64 = -1;

/// Numeric representation of `#NE` (not recorded in that election year).
pub const NOT_RECORDED_NUMERIC: i64 = -3;

fn normalized(raw: &str) -> &str {
    raw.trim()
}

fn null_marker(value: &str) -> Option<i64> {
    match value {
        "" | "#NULO" | "#NULO#" => Some(NULL_NUMERIC),
        "#NE" | "#NE#" => Some(NOT_RECORDED_NUMERIC),
        _ => None,
    }
}

/// Parses a numeric code, keeping the official null sentinels.
pub fn parse_code(column: &'static str, raw: &str) -> Result<i64> {
    let value = normalized(raw);

    if let Some(sentinel) = null_marker(value) {
        return Ok(sentinel);
    }

    value.parse::<i64>().map_err(|_| CoreError::InvalidNumber {
        column,
        value: value.to_owned(),
    })
}

/// Parses an optional quantity, mapping null sentinels to `None`.
pub fn parse_quantity(column: &'static str, raw: &str) -> Result<Option<i64>> {
    let value = normalized(raw);

    if null_marker(value).is_some() {
        return Ok(None);
    }

    let parsed = value.parse::<i64>().map_err(|_| CoreError::InvalidNumber {
        column,
        value: value.to_owned(),
    })?;

    if parsed == NULL_NUMERIC || parsed == NOT_RECORDED_NUMERIC {
        Ok(None)
    } else {
        Ok(Some(parsed))
    }
}

/// Parses a text field, mapping null markers to an empty string.
pub fn parse_text(raw: &str) -> String {
    let value = normalized(raw);

    if null_marker(value).is_some() {
        String::new()
    } else {
        value.to_owned()
    }
}

/// Parses an optional text field, mapping null markers to `None`.
pub fn parse_optional_text(raw: &str) -> Option<String> {
    let value = normalized(raw);

    if null_marker(value).is_some() {
        None
    } else {
        Some(value.to_owned())
    }
}
