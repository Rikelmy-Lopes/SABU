//! Runtime file locations under `%LOCALAPPDATA%\Sabu`.

mod app_paths;

pub use app_paths::AppPaths;

#[cfg(test)]
#[path = "paths_tests.rs"]
mod paths_tests;
