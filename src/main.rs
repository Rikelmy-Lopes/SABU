//! SABU — Ballot Box Bulletin Analysis System.
//!
//! The root crate contains the command layer, i18n, and the `egui` interface.
//! Parsing and analytics live in the GUI-independent `sabu_core` crate.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod commands;
mod i18n;
mod startup;
mod ui;

fn main() -> std::process::ExitCode {
    match startup::run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            log::error!("startup failed: {error}");
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}
