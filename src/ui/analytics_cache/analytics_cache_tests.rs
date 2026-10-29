//! Tests for the bounded analytics table cache.

use sabu_core::analytics::VoteGroup;

use super::TableCache;

fn groups(count: usize) -> Vec<VoteGroup> {
    (0..count)
        .map(|index| VoteGroup {
            name: format!("item {index}"),
            votes: index as u64,
        })
        .collect()
}

#[test]
fn copies_at_most_twenty_rows() {
    let table = TableCache::new(&groups(25));
    assert_eq!(table.rows.len(), 20);
    assert!(table.has_more);
}

#[test]
fn marks_short_tables_as_complete() {
    let table = TableCache::new(&groups(3));
    assert_eq!(table.rows.len(), 3);
    assert!(!table.has_more);
}

#[test]
fn handles_empty_tables() {
    let table = TableCache::new(&[]);
    assert!(table.rows.is_empty());
    assert!(!table.has_more);
}
