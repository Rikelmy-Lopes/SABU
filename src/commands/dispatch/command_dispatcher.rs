//! Handle used by the UI to send commands to the worker.

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use sabu_core::state::AppState;

use super::DispatcherError;
use super::command_event::CommandEvent;
use super::request::Request;
use super::worker::run_worker;
use crate::commands::Command;

/// Handle used by the UI to send commands to the worker.
pub struct CommandDispatcher {
    request_tx: Sender<Request>,
    event_rx: Receiver<CommandEvent>,
    worker: Option<JoinHandle<()>>,
}

impl CommandDispatcher {
    /// Starts the worker thread.
    pub fn spawn(state: Arc<Mutex<AppState>>) -> Result<Self, DispatcherError> {
        let (request_tx, request_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        let worker = thread::Builder::new()
            .name("sabu-command-worker".to_owned())
            .spawn(move || run_worker(&state, &request_rx, &event_tx))?;

        Ok(Self {
            request_tx,
            event_rx,
            worker: Some(worker),
        })
    }

    /// Queues a command for execution.
    pub fn execute(&self, command: Box<dyn Command>) -> Result<(), DispatcherError> {
        self.request_tx
            .send(Request::Execute(command))
            .map_err(|_| DispatcherError::WorkerStopped)
    }

    /// Queues an undo operation.
    pub fn undo(&self) -> Result<(), DispatcherError> {
        self.request_tx
            .send(Request::Undo)
            .map_err(|_| DispatcherError::WorkerStopped)
    }

    /// Queues a redo operation.
    pub fn redo(&self) -> Result<(), DispatcherError> {
        self.request_tx
            .send(Request::Redo)
            .map_err(|_| DispatcherError::WorkerStopped)
    }

    /// Drains every event that finished since the last call.
    pub fn poll_events(&self) -> Vec<CommandEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.event_rx.try_recv() {
            events.push(event);
        }
        events
    }
}

impl Drop for CommandDispatcher {
    fn drop(&mut self) {
        let _ = self.request_tx.send(Request::Shutdown);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
