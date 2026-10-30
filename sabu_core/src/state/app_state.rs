use std::sync::Arc;

use crate::config::language::Language;
use crate::config::theme::Theme;
use crate::config::user_config::UserConfig;
use crate::dataset::ballot_dataset::BallotDataset;

/// State mutated only through commands.
#[derive(Debug, Clone, Default)]
pub struct AppState {
    config: UserConfig,
    dataset: Option<Arc<BallotDataset>>,
}

impl AppState {
    /// Creates a state with the persisted configuration.
    pub fn new(config: UserConfig) -> Self {
        Self {
            config,
            dataset: None,
        }
    }

    /// Current configuration.
    pub fn config(&self) -> &UserConfig {
        &self.config
    }

    /// Loaded dataset, when any.
    pub fn dataset(&self) -> Option<&BallotDataset> {
        self.dataset.as_deref()
    }

    /// Whether a dataset is loaded.
    pub fn has_dataset(&self) -> bool {
        self.dataset.is_some()
    }

    /// Replaces the active dataset.
    pub fn replace_dataset(&mut self, dataset: BallotDataset) {
        self.dataset = Some(Arc::new(dataset));
    }

    /// Removes the active dataset.
    pub fn clear_dataset(&mut self) {
        self.dataset = None;
    }

    /// Updates the selected theme.
    pub fn set_theme(&mut self, theme: Theme) {
        self.config.theme = theme;
    }

    /// Updates the selected language.
    pub fn set_language(&mut self, language: Language) {
        self.config.language = language;
    }
}
