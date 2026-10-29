//! The command worker loop.

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use sabu_core::CoreError;
use sabu_core::state::AppState;

use super::command_event::CommandEvent;
use super::request::Request;
use crate::commands::CommandHistory;

/// Runs requests until shutdown, executing commands on this thread.
pub(super) fn run_worker(
    state: &Arc<Mutex<AppState>>,
    request_rx: &Receiver<Request>,
    event_tx: &Sender<CommandEvent>,
) {
    let mut history = CommandHistory::new();

    while let Ok(request) = request_rx.recv() {
        match request {
            Request::Shutdown => break,
            Request::Execute(command) => {
                let label_key = command.label_key();
                let result = match state.lock() {
                    Ok(mut state) => history.execute(command, &mut state),
                    Err(_) => Err(CoreError::StatePoisoned),
                };

                let event = match result {
                    Ok(()) => CommandEvent::Completed { label_key },
                    Err(error) => CommandEvent::Failed { label_key, error },
                };
                if event_tx.send(event).is_err() {
                    break;
                }
            }
            Request::Undo => {
                let result = match state.lock() {
                    Ok(mut state) => history.undo(&mut state),
                    Err(_) => Err(CoreError::StatePoisoned),
                };

                let event = match result {
                    Ok(Some(label_key)) => CommandEvent::Undone { label_key },
                    Ok(None) => CommandEvent::NothingToUndo,
                    Err(error) => CommandEvent::Failed {
                        label_key: "command.undo",
                        error,
                    },
                };
                if event_tx.send(event).is_err() {
                    break;
                }
            }
            Request::Redo => {
                let result = match state.lock() {
                    Ok(mut state) => history.redo(&mut state),
                    Err(_) => Err(CoreError::StatePoisoned),
                };

                let event = match result {
                    Ok(Some(label_key)) => CommandEvent::Redone { label_key },
                    Ok(None) => CommandEvent::NothingToRedo,
                    Err(error) => CommandEvent::Failed {
                        label_key: "command.redo",
                        error,
                    },
                };
                if event_tx.send(event).is_err() {
                    break;
                }
            }
        }
    }
}
