#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod i18n;
mod startup;
mod ui;

fn main() {
    if let Err(error) = startup::run() {
        log::error!("startup failed: {error}");
        eprintln!("Sabu failed to start: {error}");
        std::process::exit(1);
    }
}
