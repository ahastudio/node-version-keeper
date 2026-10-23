mod common;

use common::{analyze_project, sample_catalog, verdicts_of, write_file};
use node_version_keeper::verdict::{ReportReason, UpgradeCause, Verdict};

fn mismatch_upgrade(replacement: &str) -> Verdict {
    Verdict::MajorUpgrade { replacement: replacement.to_string(), cause: UpgradeCause::Mismatch }
}

fn end_of_support_upgrade(replacement: &str) -> Verdict {
    Verdict::MajorUpgrade { replacement: replacement.to_string(), cause: UpgradeCause::EndOfSupport }
}

#[test]
fn aligns_every_lower_major_in_a_project_to_the_highest_major() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "app/.nvmrc", "24.11.0\n");
    write_file(root.path(), "app/Dockerfile", "FROM node:22.4.0-alpine\n");
    write_file(root.path(), "app/package.json", "{\"volta\":{\"node\":\"20.10.0\"}}");
    let files = ["app/.nvmrc", "app/Dockerfile", "app/package.json"];
    let analyses = analyze_project(root.path(), &files, &sample_catalog());
    assert_eq!(verdicts_of(&analyses[0]), vec![Verdict::Fix("24.22.1".to_string())]);
    assert_eq!(verdicts_of(&analyses[1]), vec![mismatch_upgrade("24.22.1")]);
    assert_eq!(verdicts_of(&analyses[2]), vec![end_of_support_upgrade("24.22.1")]);
}

#[test]
fn aligns_a_value_that_is_already_current_and_keeps_the_prefix() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), ".nvmrc", "v22.23.4\n");
    write_file(root.path(), "Dockerfile", "FROM node:24.22.1\n");
    let analyses = analyze_project(root.path(), &[".nvmrc", "Dockerfile"], &sample_catalog());
    assert_eq!(verdicts_of(&analyses[0]), vec![mismatch_upgrade("v24.22.1")]);
    assert_eq!(verdicts_of(&analyses[1]), vec![Verdict::Current]);
}

#[test]
fn aligns_a_major_only_value_keeping_its_shape() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), ".nvmrc", "24.22.1\n");
    write_file(root.path(), "Dockerfile", "FROM node:22-alpine\n");
    let analyses = analyze_project(root.path(), &[".nvmrc", "Dockerfile"], &sample_catalog());
    assert_eq!(verdicts_of(&analyses[1]), vec![mismatch_upgrade("24")]);
}

#[test]
fn does_not_align_across_projects() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "first/.nvmrc", "24.22.1\n");
    write_file(root.path(), "second/.nvmrc", "22.4.0\n");
    let analyses = analyze_project(root.path(), &["first/.nvmrc", "second/.nvmrc"], &sample_catalog());
    assert_eq!(verdicts_of(&analyses[1]), vec![Verdict::Fix("22.23.4".to_string())]);
}

#[test]
fn a_workflow_belongs_to_the_project_holding_dot_github() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "app/.nvmrc", "24.22.1\n");
    write_file(root.path(), "app/.github/workflows/ci.yml", "node-version: 22.4.0\n");
    let files = ["app/.nvmrc", "app/.github/workflows/ci.yml"];
    let analyses = analyze_project(root.path(), &files, &sample_catalog());
    assert_eq!(verdicts_of(&analyses[1]), vec![mismatch_upgrade("24.22.1")]);
}

#[test]
fn does_not_align_when_the_highest_major_has_no_fixed_version() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), ".nvmrc", "25.0.0\n");
    write_file(root.path(), "Dockerfile", "FROM node:22.4.0\n");
    let analyses = analyze_project(root.path(), &[".nvmrc", "Dockerfile"], &sample_catalog());
    assert_eq!(verdicts_of(&analyses[1]), vec![Verdict::Fix("22.23.4".to_string())]);
}

#[test]
fn ignores_ranges_aliases_and_unreadable_files_when_comparing_majors() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), ".nvmrc", "lts/*\n");
    write_file(root.path(), "package.json", "{ broken file");
    write_file(root.path(), "Dockerfile", "FROM node:22.4.0\n");
    let analyses = analyze_project(root.path(), &[".nvmrc", "package.json", "Dockerfile"], &sample_catalog());
    assert_eq!(verdicts_of(&analyses[0]), vec![Verdict::Report(ReportReason::NotExact)]);
    assert!(matches!(verdicts_of(&analyses[1])[0], Verdict::Report(ReportReason::Unparsable(_))));
    assert_eq!(verdicts_of(&analyses[2]), vec![Verdict::Fix("22.23.4".to_string())]);
}
