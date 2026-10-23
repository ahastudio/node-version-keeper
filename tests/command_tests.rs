mod common;

use std::path::Path;

use common::{
    FailingWriter, FakeReleaseSource, TODAY, read_file, run_command, write_file, write_messy_project,
};
use node_version_keeper::command::run;
use node_version_keeper::release_catalog::RELEASE_SCHEDULE_URL;

const MESSY_SUMMARY: &str =
    "Summary: 5 files checked, End of support 3, Version mismatch 2, To fix 1, Left alone 2";

fn text(path: &Path) -> &str {
    path.to_str().expect("path must be UTF-8")
}

#[test]
fn help_prints_the_usage() {
    let (outcome, output) = run_command(&["--help"], &FakeReleaseSource::sample());
    assert_eq!(outcome, Ok(0));
    assert!(output.starts_with("Usage: node-version-keeper <folder> [options]"), "{output}");
}

#[test]
fn preview_lists_everything_and_changes_nothing() {
    let root = tempfile::tempdir().unwrap();
    write_messy_project(root.path());
    let (outcome, output) = run_command(&[text(root.path())], &FakeReleaseSource::sample());
    assert_eq!(outcome, Ok(1));
    for expected in ["End of support\n", "Version mismatch\n", "To fix\n", "Left alone\n", MESSY_SUMMARY] {
        assert!(output.contains(expected), "{expected}\n{output}");
    }
    assert!(output.contains("--write to fix") && output.contains("--write and --upgrade-major to fix"));
    assert!(!output.contains("node_modules") && !output.contains("/dist/"), "{output}");
    assert_eq!(read_file(root.path(), "app/.nvmrc"), "24.11.0\n");
}

#[test]
fn write_alone_applies_only_same_major_fixes() {
    let root = tempfile::tempdir().unwrap();
    write_messy_project(root.path());
    let (outcome, output) = run_command(&[text(root.path()), "--write"], &FakeReleaseSource::sample());
    assert_eq!(outcome, Ok(1));
    assert!(output.contains("(fixed)") && output.contains("--upgrade-major required"), "{output}");
    assert_eq!(read_file(root.path(), "app/.nvmrc"), "24.22.1\n");
    assert_eq!(read_file(root.path(), "legacy/.nvmrc"), "18.20.4\n");
}

#[test]
fn write_with_upgrade_major_applies_everything_it_can() {
    let root = tempfile::tempdir().unwrap();
    write_messy_project(root.path());
    let arguments = [text(root.path()), "--write", "--upgrade-major"];
    let (outcome, output) = run_command(&arguments, &FakeReleaseSource::sample());
    assert_eq!(outcome, Ok(1));
    assert!(!output.contains("to fix)") && !output.contains("required)"), "{output}");
    assert_eq!(read_file(root.path(), "app/.nvmrc"), "24.22.1\n");
    assert_eq!(read_file(root.path(), "app/Dockerfile"), "FROM node:24.22.1-alpine AS build\nFROM node:24\n");
    assert!(read_file(root.path(), "app/package.json").contains("\"volta\": { \"node\": \"24.22.1\" }"));
    assert!(read_file(root.path(), "app/.github/workflows/ci.yml").contains("node-version: [24, 24.22.1]"));
    assert_eq!(read_file(root.path(), "legacy/.nvmrc"), "22.23.4\n");
}

#[test]
fn running_again_after_the_upgrade_leaves_only_the_reports() {
    let root = tempfile::tempdir().unwrap();
    write_messy_project(root.path());
    let arguments = [text(root.path()), "--write", "--upgrade-major"];
    run_command(&arguments, &FakeReleaseSource::sample()).0.unwrap();
    let (outcome, output) = run_command(&[text(root.path())], &FakeReleaseSource::sample());
    let expected = "Summary: 5 files checked, End of support 0, Version mismatch 0, To fix 0, Left alone 3";
    assert_eq!(outcome, Ok(1));
    assert!(output.contains(expected), "{output}");
}

#[test]
fn a_project_that_is_already_current_exits_zero() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), ".nvmrc", "24.22.1\n");
    let (outcome, output) = run_command(&[text(root.path())], &FakeReleaseSource::sample());
    let expected = "Summary: 1 files checked, End of support 0, Version mismatch 0, To fix 0, Left alone 0\n";
    assert_eq!((outcome, output.as_str()), (Ok(0), expected));
}

#[test]
fn exits_zero_when_writing_resolves_everything() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), ".nvmrc", "24.11.0\n");
    let (outcome, _) = run_command(&[text(root.path()), "--write"], &FakeReleaseSource::sample());
    assert_eq!(outcome, Ok(0));
    assert_eq!(read_file(root.path(), ".nvmrc"), "24.22.1\n");
}

#[test]
fn exclude_option_skips_the_named_folder_for_this_run() {
    let root = tempfile::tempdir().unwrap();
    write_messy_project(root.path());
    let arguments = [text(root.path()), "--exclude", "legacy"];
    let (_, output) = run_command(&arguments, &FakeReleaseSource::sample());
    assert!(output.contains("4 files checked"), "{output}");
}

#[test]
fn config_option_replaces_the_default_exclusions_but_not_the_fixed_ones() {
    let root = tempfile::tempdir().unwrap();
    write_messy_project(root.path());
    let config_root = tempfile::tempdir().unwrap();
    write_file(config_root.path(), "config.json", "{\"exclude\": [\"legacy\"]}");
    let config = config_root.path().join("config.json");
    let arguments = [text(root.path()), "--config", text(&config)];
    let (_, output) = run_command(&arguments, &FakeReleaseSource::sample());
    assert!(output.contains("5 files checked") && output.contains("/dist/.nvmrc"), "{output}");
    assert!(!output.contains("node_modules"), "{output}");
}

#[test]
fn an_unparsable_package_json_is_reported_and_exits_one() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "package.json", "{ broken file");
    let (outcome, output) = run_command(&[text(root.path())], &FakeReleaseSource::sample());
    assert_eq!(outcome, Ok(1));
    assert!(output.contains("could not read file"), "{output}");
}

#[test]
fn a_missing_folder_is_an_error() {
    let root = tempfile::tempdir().unwrap();
    let missing = root.path().join("missing-folder");
    assert!(run_command(&[text(&missing)], &FakeReleaseSource::sample()).0.is_err());
}

#[test]
fn a_failed_release_download_is_an_error_and_changes_nothing() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), ".nvmrc", "24.11.0\n");
    let source = FakeReleaseSource::failing(RELEASE_SCHEDULE_URL);
    let arguments = [text(root.path()), "--write"];
    assert!(run_command(&arguments, &source).0.is_err());
    assert_eq!(read_file(root.path(), ".nvmrc"), "24.11.0\n");
}

#[test]
fn a_bad_config_is_an_error() {
    let root = tempfile::tempdir().unwrap();
    let arguments = [text(root.path()), "--config", "missing-config.json"];
    assert!(run_command(&arguments, &FakeReleaseSource::sample()).0.is_err());
}

#[test]
fn an_unreadable_version_file_is_an_error() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join(".nvmrc"), [0xff, 0xfe, 0xfd]).unwrap();
    assert!(run_command(&[text(root.path())], &FakeReleaseSource::sample()).0.is_err());
}

#[test]
fn a_failing_output_is_an_error_for_help_and_for_reports() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), ".nvmrc", "24.22.1\n");
    let help = vec!["--help".to_string()];
    let run_arguments = vec![text(root.path()).to_string()];
    let source = FakeReleaseSource::sample();
    assert!(run(&help, &source, TODAY, &mut FailingWriter).is_err());
    assert!(run(&run_arguments, &source, TODAY, &mut FailingWriter).is_err());
}

#[cfg(unix)]
#[test]
fn a_file_that_cannot_be_written_is_an_error() {
    use std::os::unix::fs::PermissionsExt;
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), ".nvmrc", "24.11.0\n");
    let path = root.path().join(".nvmrc");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o444)).unwrap();
    let (outcome, _) = run_command(&[text(root.path()), "--write"], &FakeReleaseSource::sample());
    assert!(outcome.is_err());
}
