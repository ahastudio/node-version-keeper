mod common;

use common::{FakeReleaseSource, INDEX_JSON, SCHEDULE_JSON, TODAY, sample_catalog};
use node_version_keeper::release_catalog::{
    RELEASE_INDEX_URL, RELEASE_SCHEDULE_URL, ReleaseCatalog, load_catalog,
};
use node_version_keeper::version::NodeVersion;

fn version(major: u32, minor: u32, patch: u32) -> NodeVersion {
    NodeVersion { major, minor, patch }
}

#[test]
fn fixed_version_is_the_newest_security_release_of_each_major() {
    let catalog = sample_catalog();
    assert_eq!(catalog.fixed_version(26), Some(version(26, 12, 1)));
    assert_eq!(catalog.fixed_version(24), Some(version(24, 22, 1)));
    assert_eq!(catalog.fixed_version(22), Some(version(22, 23, 4)));
}

#[test]
fn fixed_version_is_missing_for_majors_without_a_security_release() {
    assert_eq!(sample_catalog().fixed_version(25), None);
}

#[test]
fn majors_past_their_end_date_are_end_of_support() {
    let catalog = sample_catalog();
    for major in [18, 20, 23] {
        assert!(catalog.is_end_of_support(major), "{major}");
    }
    for major in [22, 24, 26, 29, 25] {
        assert!(!catalog.is_end_of_support(major), "{major}");
    }
}

#[test]
fn the_end_date_itself_is_still_supported() {
    let on_end_date = ReleaseCatalog::from_documents(INDEX_JSON, SCHEDULE_JSON, "2027-04-30").unwrap();
    let after_end_date = ReleaseCatalog::from_documents(INDEX_JSON, SCHEDULE_JSON, "2027-05-01").unwrap();
    assert!(!on_end_date.is_end_of_support(22));
    assert!(after_end_date.is_end_of_support(22));
}

#[test]
fn upgrade_target_is_the_nearest_supported_lts_above() {
    let catalog = sample_catalog();
    assert_eq!(catalog.upgrade_target(18), Some(version(22, 23, 4)));
    assert_eq!(catalog.upgrade_target(23), Some(version(24, 22, 1)));
}

#[test]
fn upgrade_target_skips_lts_lines_that_have_not_started_or_have_no_fixed_version() {
    assert_eq!(sample_catalog().upgrade_target(24), None);
}

#[test]
fn reports_an_unreadable_release_index() {
    let error = ReleaseCatalog::from_documents("not json", SCHEDULE_JSON, TODAY).unwrap_err();
    assert!(error.to_string().starts_with("failed to parse release list"), "{error}");
}

#[test]
fn reports_an_unreadable_release_schedule() {
    let error = ReleaseCatalog::from_documents(INDEX_JSON, "not json", TODAY).unwrap_err();
    assert!(error.to_string().starts_with("failed to parse support schedule"), "{error}");
}

#[test]
fn loads_the_catalog_from_a_source() {
    let catalog = load_catalog(&FakeReleaseSource::sample(), TODAY).unwrap();
    assert_eq!(catalog.fixed_version(24), Some(version(24, 22, 1)));
}

#[test]
fn stops_when_the_release_index_cannot_be_fetched() {
    let error = load_catalog(&FakeReleaseSource::failing(RELEASE_INDEX_URL), TODAY).unwrap_err();
    assert!(error.to_string().contains(RELEASE_INDEX_URL), "{error}");
}

#[test]
fn stops_when_the_release_schedule_cannot_be_fetched() {
    let error = load_catalog(&FakeReleaseSource::failing(RELEASE_SCHEDULE_URL), TODAY).unwrap_err();
    assert!(error.to_string().contains(RELEASE_SCHEDULE_URL), "{error}");
}
