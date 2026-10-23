mod common;

use common::write_file;
use node_version_keeper::config::{ALWAYS_EXCLUDED, excluded_names, load_config};

#[test]
fn the_default_config_excludes_build_output_folders() {
    let config = load_config(None).unwrap();
    assert_eq!(config.exclude, vec!["dist", "build", ".next", "coverage", "vendor"]);
}

#[test]
fn loads_a_config_from_a_path() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "custom.json", "{\"exclude\": [\"generated\"]}");
    let config = load_config(Some(&root.path().join("custom.json"))).unwrap();
    assert_eq!(config.exclude, vec!["generated"]);
}

#[test]
fn a_missing_config_file_is_an_error() {
    let root = tempfile::tempdir().unwrap();
    assert!(load_config(Some(&root.path().join("missing-config.json"))).is_err());
}

#[test]
fn a_config_that_is_not_json_is_an_error() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "broken.json", "{ broken");
    let error = load_config(Some(&root.path().join("broken.json"))).unwrap_err();
    assert!(error.to_string().starts_with("failed to parse config file"), "{error}");
}

#[test]
fn a_config_without_an_exclude_list_is_an_error() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "empty.json", "{}");
    assert!(load_config(Some(&root.path().join("empty.json"))).is_err());
}

#[test]
fn excluded_names_combine_fixed_config_and_extra_names() {
    let config = load_config(None).unwrap();
    let names = excluded_names(&config, &["generated".to_string()]);
    for fixed in ALWAYS_EXCLUDED {
        assert!(names.contains(fixed), "{fixed}");
    }
    assert!(names.contains("dist"));
    assert!(names.contains("generated"));
}

#[test]
fn fixed_exclusions_stay_even_when_a_config_lists_nothing() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "none.json", "{\"exclude\": []}");
    let config = load_config(Some(&root.path().join("none.json"))).unwrap();
    let names = excluded_names(&config, &[]);
    assert_eq!(names.len(), ALWAYS_EXCLUDED.len());
}
