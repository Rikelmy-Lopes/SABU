//! Command that changes and persists the active language.

use sabu_core::CoreError;
use sabu_core::config::Language;
use sabu_core::state::AppState;

use crate::commands::Command;

/// Switches the interface language.
pub struct SetLanguageCommand {
    next: Language,
    previous: Option<Language>,
}

impl SetLanguageCommand {
    /// Creates a command for the target language.
    pub fn new(next: Language) -> Self {
        Self {
            next,
            previous: None,
        }
    }
}

impl Command for SetLanguageCommand {
    fn label_key(&self) -> &'static str {
        "command.set_language"
    }

    fn execute(&mut self, state: &mut AppState) -> Result<(), CoreError> {
        let previous = state.config.language;
        self.previous = Some(previous);
        if previous == self.next {
            return Ok(());
        }

        state.config.language = self.next;
        if let Err(error) = state.save_config() {
            state.config.language = previous;
            return Err(error);
        }
        Ok(())
    }

    fn undo(&mut self, state: &mut AppState) -> Result<(), CoreError> {
        let Some(previous) = self.previous.take() else {
            return Ok(());
        };

        let current = state.config.language;
        state.config.language = previous;
        if let Err(error) = state.save_config() {
            state.config.language = current;
            self.previous = Some(previous);
            return Err(error);
        }
        Ok(())
    }
}
