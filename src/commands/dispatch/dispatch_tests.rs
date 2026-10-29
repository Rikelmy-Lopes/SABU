//! Tests for the background command worker.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sabu_core::config::Language;

use crate::commands::test_support::state_in;
use crate::commands::{CommandDispatcher, CommandEvent, SetLanguageCommand};

#[test]
fn dispatcher_runs_commands_on_the_worker_thread() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let state = Arc::new(Mutex::new(state_in(&directory)));
    let dispatcher = CommandDispatcher::spawn(Arc::clone(&state));
    assert!(dispatcher.is_ok());
    let Ok(dispatcher) = dispatcher else {
        return;
    };

    assert!(
        dispatcher
            .execute(Box::new(SetLanguageCommand::new(Language::En)))
            .is_ok()
    );

    let event = wait_for_event(&dispatcher);
    assert!(matches!(
        event,
        Some(CommandEvent::Completed {
            label_key: "command.set_language"
        })
    ));
    assert_eq!(
        state.lock().ok().map(|state| state.config.language),
        Some(Language::En)
    );

    assert!(dispatcher.undo().is_ok());
    let event = wait_for_event(&dispatcher);
    assert!(matches!(
        event,
        Some(CommandEvent::Undone {
            label_key: "command.set_language"
        })
    ));
    assert_eq!(
        state.lock().ok().map(|state| state.config.language),
        Some(Language::PtBr)
    );
}

#[test]
fn dispatcher_reports_empty_history() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let state = Arc::new(Mutex::new(state_in(&directory)));
    let dispatcher = CommandDispatcher::spawn(state);
    assert!(dispatcher.is_ok());
    let Ok(dispatcher) = dispatcher else {
        return;
    };

    assert!(dispatcher.undo().is_ok());
    assert!(matches!(
        wait_for_event(&dispatcher),
        Some(CommandEvent::NothingToUndo)
    ));

    assert!(dispatcher.redo().is_ok());
    assert!(matches!(
        wait_for_event(&dispatcher),
        Some(CommandEvent::NothingToRedo)
    ));
}

fn wait_for_event(dispatcher: &CommandDispatcher) -> Option<CommandEvent> {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        let mut events = dispatcher.poll_events();
        if !events.is_empty() {
            return events.pop();
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    None
}
