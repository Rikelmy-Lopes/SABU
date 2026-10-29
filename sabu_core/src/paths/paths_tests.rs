//! Tests for runtime path resolution.

use std::path::PathBuf;

use super::AppPaths;

#[test]
fn derives_config_and_log_paths_from_root() {
    let paths = AppPaths::with_root(PathBuf::from("C:/data/Sabu"));
    assert_eq!(paths.root(), PathBuf::from("C:/data/Sabu"));
    assert_eq!(
        paths.config_file(),
        PathBuf::from("C:/data/Sabu/config.json")
    );
    assert_eq!(paths.log_file(), PathBuf::from("C:/data/Sabu/logs.log"));
}

#[test]
fn ensure_creates_the_root_directory() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let root = directory.path().join("nested").join("Sabu");
    let paths = AppPaths::with_root(root.clone());
    assert!(paths.ensure().is_ok());
    assert!(root.is_dir());
}

#[test]
fn discover_requires_local_app_data() {
    // The test process normally has LOCALAPPDATA on Windows.
    if std::env::var_os("LOCALAPPDATA").is_some() {
        let paths = AppPaths::discover();
        assert!(paths.is_ok());
        if let Ok(paths) = paths {
            assert!(paths.root().ends_with("Sabu"));
        }
    }
}
