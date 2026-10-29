//! Bounded copy of an analytics table.

use sabu_core::analytics::VoteGroup;

/// Maximum number of rows shown in each analytics table.
const MAX_TABLE_ROWS: usize = 20;

/// Bounded copy of an analytics table.
pub(crate) struct TableCache {
    pub(crate) rows: Vec<VoteGroup>,
    pub(crate) has_more: bool,
}

impl TableCache {
    /// Copies at most [`MAX_TABLE_ROWS`] groups.
    pub(crate) fn new(groups: &[VoteGroup]) -> Self {
        Self {
            rows: groups.iter().take(MAX_TABLE_ROWS).cloned().collect(),
            has_more: groups.len() > MAX_TABLE_ROWS,
        }
    }
}
