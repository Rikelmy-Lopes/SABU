use std::path::PathBuf;

use crate::config::language::Language;
use crate::config::theme::Theme;
use crate::config::user_config::UserConfig;
use crate::dataset::ballot_dataset::BallotDataset;
use crate::state::app_state::AppState;

fn example_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../example/data.csv")
}

#[test]
fn starts_without_dataset() {
    let state = AppState::new(UserConfig::default());

    assert!(!state.has_dataset());
    assert!(state.dataset().is_none());
}

#[test]
fn updates_configuration() {
    let mut state = AppState::new(UserConfig::default());

    state.set_theme(Theme::Light);
    state.set_language(Language::PtBr);

    assert_eq!(state.config().theme, Theme::Light);
    assert_eq!(state.config().language, Language::PtBr);
}

#[test]
fn replaces_and_clears_dataset() {
    let mut state = AppState::new(UserConfig::default());
    let dataset = BallotDataset::load(example_path()).unwrap();

    state.replace_dataset(dataset);
    assert!(state.has_dataset());

    state.clear_dataset();
    assert!(!state.has_dataset());
}
