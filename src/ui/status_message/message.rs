//! Status message shown in the status bar.

/// A translated status line.
#[derive(Clone, Copy)]
pub(crate) struct StatusMessage {
    pub(crate) prefix_key: &'static str,
    pub(crate) message_key: &'static str,
    pub(crate) is_error: bool,
}

impl StatusMessage {
    /// Builds a success message.
    pub(crate) fn success(message_key: &'static str) -> Self {
        Self {
            prefix_key: "status.success",
            message_key,
            is_error: false,
        }
    }

    /// Builds an informational message.
    pub(crate) fn info(message_key: &'static str) -> Self {
        Self {
            prefix_key: "status.info",
            message_key,
            is_error: false,
        }
    }

    /// Builds an error message.
    pub(crate) fn error(message_key: &'static str) -> Self {
        Self {
            prefix_key: "status.error",
            message_key,
            is_error: true,
        }
    }
}
