use serde::{Deserialize, Serialize};

/// Visual theme selected by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    /// Dark colors.
    #[default]
    Dark,
    /// Light colors.
    Light,
}
