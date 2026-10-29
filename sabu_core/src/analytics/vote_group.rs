//! A named vote total.

/// A named vote total.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoteGroup {
    /// Office, votable, or party name.
    pub name: String,
    /// Sum of votes for this name.
    pub votes: u64,
}
