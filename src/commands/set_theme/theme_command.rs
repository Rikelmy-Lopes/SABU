use sabu_core::config::Theme;

use crate::commands::command::{Command, CommandContext, CommandError};

/// Stable id of the command that changes the theme.
pub const SET_THEME: &str = "settings.set_theme";

/// Changes the visual theme and persists the configuration.
pub struct SetThemeCommand {
    theme: Theme,
}

impl SetThemeCommand {
    /// Creates the command for `theme`.
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }
}

impl Command for SetThemeCommand {
    fn id(&self) -> &'static str {
        SET_THEME
    }

    fn execute(self: Box<Self>, context: &mut CommandContext<'_>) -> Result<(), CommandError> {
        if context.state.config().theme == self.theme {
            return Ok(());
        }

        context.state.set_theme(self.theme);
        context.config_store.save(context.state.config())?;

        Ok(())
    }
}
