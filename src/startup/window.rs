//! Native window creation.

use std::sync::{Arc, Mutex};

use eframe::egui;
use sabu_core::state::AppState;

use crate::commands::CommandDispatcher;
use crate::i18n::Translator;
use crate::ui::SabuApp;

use super::StartupError;

/// Opens the main window and blocks until it is closed.
pub(super) fn run(
    state: Arc<Mutex<AppState>>,
    dispatcher: CommandDispatcher,
    translator: Translator,
) -> Result<(), StartupError> {
    let title = translator.t("app.title").to_owned();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(title.clone())
            .with_inner_size([1100.0, 720.0])
            .with_min_inner_size([760.0, 480.0]),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        &title,
        native_options,
        Box::new(move |creation_context| {
            let app: Box<dyn eframe::App> = Box::new(SabuApp::new(
                creation_context,
                state,
                dispatcher,
                translator,
            ));
            Ok(app)
        }),
    )?;

    Ok(())
}
