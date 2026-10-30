use super::context::CommandContext;
use super::error::CommandError;

/// One user action or state change.
///
/// Commands run on a background worker, never on the UI thread.
pub trait Command: Send + 'static {
    /// Stable identifier, in the `domain.action` format.
    fn id(&self) -> &'static str;

    /// Executes the action against the application state.
    fn execute(self: Box<Self>, context: &mut CommandContext<'_>) -> Result<(), CommandError>;
}
