use crate::i18n::number_formatter::LocaleNumber;
use crate::i18n::translator::Translator;

/// Message shown in the bottom status bar.
#[derive(Debug, Clone, Default)]
pub enum StatusMessage {
    /// Nothing running.
    #[default]
    Ready,
    /// A command is running.
    Busy,
    /// A dataset was loaded.
    Loaded {
        /// Number of records loaded.
        records: usize,
    },
    /// The dataset was closed.
    Cleared,
    /// A command failed.
    Failed(String),
}

impl StatusMessage {
    /// Whether the status represents a running command.
    pub fn is_busy(&self) -> bool {
        matches!(self, Self::Busy)
    }

    /// Localized text for the status bar.
    pub fn text(&self, translator: &Translator) -> String {
        match self {
            Self::Ready => translator.text("status.ready"),
            Self::Busy => translator.text("status.busy"),
            Self::Loaded { records } => {
                let count = LocaleNumber::format_integer(translator.language(), *records as i64);

                translator.text_with("status.loaded", &[("count", count.as_str())])
            }
            Self::Cleared => translator.text("status.cleared"),
            Self::Failed(message) => {
                translator.text_with("status.error", &[("message", message.as_str())])
            }
        }
    }
}
