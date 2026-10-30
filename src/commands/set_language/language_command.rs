use sabu_core::config::Language;

use crate::commands::command::{Command, CommandContext, CommandError};

/// Stable id of the command that changes the language.
pub const SET_LANGUAGE: &str = "settings.set_language";

/// Changes the interface language and persists the configuration.
pub struct SetLanguageCommand {
    language: Language,
}

impl SetLanguageCommand {
    /// Creates the command for `language`.
    pub fn new(language: Language) -> Self {
        Self { language }
    }
}

impl Command for SetLanguageCommand {
    fn id(&self) -> &'static str {
        SET_LANGUAGE
    }

    fn execute(self: Box<Self>, context: &mut CommandContext<'_>) -> Result<(), CommandError> {
        if context.state.config().language == self.language {
            return Ok(());
        }

        context.state.set_language(self.language);
        context.config_store.save(context.state.config())?;

        Ok(())
    }
}
