//! Graphical interface built with `egui`/`eframe`.

mod analytics_cache;
mod content_panel;
mod file_bar;
mod menu_bar;
mod sabu_app;
mod status_bar;
mod status_message;
mod summary_grid;
mod theme;
mod top_bar;
mod vote_table;

pub(crate) use analytics_cache::{AnalyticsCache, TableCache};
pub(crate) use content_panel::content_panel;
pub(crate) use file_bar::file_bar;
pub(crate) use menu_bar::{MenuAction, menu_bar};
pub use sabu_app::SabuApp;
pub(crate) use status_bar::status_bar;
pub(crate) use status_message::StatusMessage;
pub(crate) use summary_grid::summary_grid;
pub(crate) use theme::apply_theme;
pub(crate) use top_bar::top_bar;
pub(crate) use vote_table::vote_table;
