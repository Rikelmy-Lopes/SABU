use encoding_rs::WINDOWS_1252;

use crate::error::core_error::{CoreError, Result};

/// Decodes ballot box bulletin bytes from Latin-1 into UTF-8.
///
/// `encoding_rs` implements Latin-1 through the Windows-1252 decoder, which is
/// a superset of ISO-8859-1 for every printable character used by the files.
pub fn latin1_to_utf8(bytes: &[u8]) -> Result<String> {
    let (decoded, _, had_errors) = WINDOWS_1252.decode(bytes);

    if had_errors {
        return Err(CoreError::Decode(
            "the input contains bytes that cannot be decoded".to_owned(),
        ));
    }

    Ok(decoded.into_owned())
}
