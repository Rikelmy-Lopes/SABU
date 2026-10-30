use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use sabu_core::config::ConfigStore;
use sabu_core::state::AppState;

use crate::commands::command::Command;
use crate::commands::dispatch::command_event::CommandEvent;
use crate::commands::dispatch::dispatcher_error::DispatcherError;
use crate::commands::dispatch::request::CommandRequest;
use crate::commands::dispatch::worker;

/// Handle used by the GUI to queue commands and receive state snapshots.
pub struct CommandDispatcher {
    request_sender: Sender<CommandRequest>,
    event_receiver: Receiver<CommandEvent>,
    pending: Arc<AtomicUsize>,
}

impl CommandDispatcher {
    /// Starts the worker thread.
    pub fn spawn(state: AppState, config_store: ConfigStore) -> Result<Self, DispatcherError> {
        let (request_sender, request_receiver) = mpsc::channel::<CommandRequest>();
        let (event_sender, event_receiver) = mpsc::channel::<CommandEvent>();
        let pending = Arc::new(AtomicUsize::new(0));
        let worker_pending = Arc::clone(&pending);

        let worker = thread::Builder::new()
            .name("sabu-command-worker".to_owned())
            .spawn(move || {
                let requests = request_receiver;
                let events = event_sender;

                let executions = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    worker::run(state, config_store, requests, events)
                }));

                if executions.is_err() {
                    log::error!("the command worker stopped unexpectedly");
                }

                worker_pending.store(0, Ordering::Relaxed);
            })
            .map_err(|error| DispatcherError::Spawn(error.to_string()))?;

        // The handle is detached: dropping the dispatcher closes the request
        // channel and the worker exits after the queued commands finish.
        drop(worker);

        Ok(Self {
            request_sender,
            event_receiver,
            pending,
        })
    }

    /// Queues a command for background execution.
    pub fn dispatch(&self, command: Box<dyn Command>) -> Result<(), DispatcherError> {
        self.pending.fetch_add(1, Ordering::Relaxed);

        if self
            .request_sender
            .send(CommandRequest { command })
            .is_err()
        {
            self.pending.fetch_sub(1, Ordering::Relaxed);
            return Err(DispatcherError::Unavailable);
        }

        Ok(())
    }

    /// Receives the next worker event without blocking.
    pub fn try_recv(&self) -> Option<CommandEvent> {
        self.event_receiver.try_recv().ok()
    }

    /// Number of commands queued or running.
    pub fn pending(&self) -> usize {
        self.pending.load(Ordering::Relaxed)
    }
}
