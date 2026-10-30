/// One aggregated vote line, such as a party or a votable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoteGroup {
    /// Human readable group name.
    pub key: String,
    /// Total votes received by the group.
    pub votes: u64,
}

impl VoteGroup {
    /// Creates an empty group for `key`.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            votes: 0,
        }
    }
}
