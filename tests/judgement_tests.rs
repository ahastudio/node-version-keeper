mod common;

use common::{INDEX_JSON, TODAY, analyze_project, sample_catalog, verdicts_of, write_file};
use node_version_keeper::analysis::analyze_file;
use node_version_keeper::release_catalog::ReleaseCatalog;
use node_version_keeper::verdict::{ReportReason, UpgradeCause, Verdict};

fn verdicts_for_nvmrc(content: &str) -> Vec<Verdict> {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), ".nvmrc", content);
    let analyses = analyze_project(root.path(), &[".nvmrc"], &sample_catalog());
    verdicts_of(&analyses[0])
}

fn fix(replacement: &str) -> Verdict {
    Verdict::Fix(replacement.to_string())
}

fn end_of_support_upgrade(replacement: &str) -> Verdict {
    Verdict::MajorUpgrade { replacement: replacement.to_string(), cause: UpgradeCause::EndOfSupport }
}

#[test]
fn raises_an_exact_version_below_the_fixed_version_within_its_major() {
    assert_eq!(verdicts_for_nvmrc("24.11.0\n"), vec![fix("24.22.1")]);
    assert_eq!(verdicts_for_nvmrc("v24.11.0\n"), vec![fix("v24.22.1")]);
    assert_eq!(verdicts_for_nvmrc("26.12.0\n"), vec![fix("26.12.1")]);
}

#[test]
fn leaves_an_exact_version_at_or_above_the_fixed_version() {
    assert_eq!(verdicts_for_nvmrc("24.22.1\n"), vec![Verdict::Current]);
    assert_eq!(verdicts_for_nvmrc("24.30.0\n"), vec![Verdict::Current]);
}

#[test]
fn leaves_a_major_without_a_known_security_release() {
    assert_eq!(verdicts_for_nvmrc("25.0.0\n"), vec![Verdict::Current]);
}

#[test]
fn moves_an_end_of_support_exact_version_to_the_nearest_supported_lts() {
    assert_eq!(verdicts_for_nvmrc("18.20.4\n"), vec![end_of_support_upgrade("22.23.4")]);
    assert_eq!(verdicts_for_nvmrc("v18.20.4\n"), vec![end_of_support_upgrade("v22.23.4")]);
}

#[test]
fn moves_an_end_of_support_major_only_value_keeping_its_shape() {
    assert_eq!(verdicts_for_nvmrc("18\n"), vec![end_of_support_upgrade("22")]);
    assert_eq!(verdicts_for_nvmrc("v18\n"), vec![end_of_support_upgrade("v22")]);
}

#[test]
fn only_reports_a_major_only_value_on_a_supported_line() {
    assert_eq!(verdicts_for_nvmrc("24\n"), vec![Verdict::Report(ReportReason::MajorOnly { major: 24 })]);
}

#[test]
fn only_reports_ranges_and_aliases() {
    assert_eq!(verdicts_for_nvmrc("lts/*\n"), vec![Verdict::Report(ReportReason::NotExact)]);
}

#[test]
fn reports_an_end_of_support_line_when_no_supported_lts_is_above() {
    let index = r#"[{"version":"v18.20.8","security":true}]"#;
    let schedule = r#"{"v18":{"lts":"2022-10-25","end":"2025-04-30"}}"#;
    let catalog = ReleaseCatalog::from_documents(index, schedule, TODAY).unwrap();
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), ".nvmrc", "18.20.4\n");
    let analysis = analyze_file(&root.path().join(".nvmrc"), &catalog).unwrap();
    assert_eq!(verdicts_of(&analysis), vec![Verdict::Report(ReportReason::EndOfSupport { major: 18 })]);
    assert!(INDEX_JSON.contains("v18.20.8"));
}

#[test]
fn an_unparsable_package_json_is_reported_for_the_whole_file() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "package.json", "{ broken file");
    let analyses = analyze_project(root.path(), &["package.json"], &sample_catalog());
    let observation = &analyses[0].observations[0];
    assert!(matches!(&observation.verdict, Verdict::Report(ReportReason::Unparsable(_))));
    assert_eq!((observation.occurrence.line, observation.occurrence.text.as_str()), (1, "package.json"));
}

#[test]
fn analyzing_a_file_that_is_not_a_version_file_is_an_error() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "README.md", "# Description\n");
    let error = analyze_file(&root.path().join("README.md"), &sample_catalog()).unwrap_err();
    assert!(error.to_string().contains("not a file to look for versions in"), "{error}");
}

#[test]
fn analyzing_a_file_that_cannot_be_read_is_an_error() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join(".nvmrc"), [0xff, 0xfe, 0xfd]).unwrap();
    assert!(analyze_file(&root.path().join(".nvmrc"), &sample_catalog()).is_err());
}

#[test]
fn analyzing_a_missing_file_is_an_error() {
    let root = tempfile::tempdir().unwrap();
    assert!(analyze_file(&root.path().join(".nvmrc"), &sample_catalog()).is_err());
}
