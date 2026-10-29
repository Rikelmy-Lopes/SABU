//! Command that changes and persists the visual theme.

use sabu_core::CoreError;
use sabu_core::config::Theme;
use sabu_core::state::AppState;

use crate::commands::Command;

/// Switches between the dark and light themes.
pub struct SetThemeCommand {
    next: Theme,
    previous: Option<Theme>,
}

impl SetThemeCommand {
    /// Creates a command for the target theme.
    pub fn new(next: Theme) -> Self {
        Self {
            next,
            previous: None,
        }
    }
}

impl Command for SetThemeCommand {
    fn label_key(&self) -> &'static str {
        "command.set_theme"
    }

    fn execute(&mut self, state: &mut AppState) -> Result<(), CoreError> {
        let previous = state.config.theme;
        self.previous = Some(previous);
        if previous == self.next {
            return Ok(());
        }

        state.config.theme = self.next;
        if let Err(error) = state.save_config() {
            state.config.theme = previous;
            return Err(error);
        }
        Ok(())
    }

    fn undo(&mut self, state: &mut AppState) -> Result<(), CoreError> {
        let Some(previous) = self.previous.take() else {
            return Ok(());
        };

        let current = state.config.theme;
        state.config.theme = previous;
        if let Err(error) = state.save_config() {
            state.config.theme = current;
            self.previous = Some(previous);
            return Err(error);
        }
        Ok(())
    }
}
