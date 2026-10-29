//! Latin-1 field decoding.

/// Decodes Latin-1 bytes to UTF-8 and trims surrounding whitespace.
pub(super) fn decode_text(bytes: &[u8]) -> String {
    let (text, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);
    text.trim().to_owned()
}
