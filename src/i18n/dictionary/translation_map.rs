//! In-memory translation dictionary.

use std::collections::HashMap;

/// Translation keys mapped to their localized text.
pub(crate) type Dictionary = HashMap<String, String>;
