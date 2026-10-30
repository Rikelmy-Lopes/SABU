use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

use sabu_core::config::ConfigStore;
use sabu_core::state::AppState;

use crate::commands::command::CommandContext;
use crate::commands::dispatch::command_event::CommandEvent;
use crate::commands::dispatch::request::CommandRequest;

/// Worker loop: owns state and executes commands one at a time.
pub(crate) fn run(
    mut state: AppState,
    config_store: ConfigStore,
    requests: Receiver<CommandRequest>,
    events: Sender<CommandEvent>,
) {
    while let Ok(request) = requests.recv() {
        let id = request.command.id();
        log::info!("command started: {id}");

        if events.send(CommandEvent::Started).is_err() {
            break;
        }

        let result = {
            let mut context = CommandContext {
                state: &mut state,
                config_store: &config_store,
            };

            request.command.execute(&mut context)
        };

        let snapshot = Arc::new(state.clone());

        let event = match result {
            Ok(()) => {
                log::info!("command finished: {id}");
                CommandEvent::Succeeded {
                    id,
                    state: snapshot,
                }
            }
            Err(error) => {
                log::error!("command failed: {id}: {error}");
                CommandEvent::Failed {
                    id,
                    message: error.to_string(),
                    state: snapshot,
                }
            }
        };

        if events.send(event).is_err() {
            break;
        }
    }
}
