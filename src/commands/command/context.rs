use sabu_core::config::ConfigStore;
use sabu_core::state::AppState;

/// Resources available to a running command.
pub struct CommandContext<'a> {
    /// Mutable application state.
    pub state: &'a mut AppState,
    /// Persistence for user configuration.
    pub config_store: &'a ConfigStore,
}
