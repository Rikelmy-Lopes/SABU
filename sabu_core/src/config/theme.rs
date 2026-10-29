//! Visual theme identifiers.

use serde::{Deserialize, Serialize};

/// Visual themes supported by the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    /// Dark theme.
    #[default]
    Dark,
    /// White (light) theme.
    Light,
}

impl Theme {
    /// Every theme known by the application.
    pub const ALL: [Theme; 2] = [Theme::Dark, Theme::Light];
}
