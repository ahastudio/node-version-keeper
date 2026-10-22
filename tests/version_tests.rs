use node_version_keeper::version::{NodeVersion, parse_digits};
use node_version_keeper::version_text::{VersionShape, VersionText, classify, major_of, render, shape_of};

fn version(major: u32, minor: u32, patch: u32) -> NodeVersion {
    NodeVersion { major, minor, patch }
}

#[test]
fn parses_exact_versions_with_and_without_prefix() {
    assert_eq!(NodeVersion::parse("24.11.0"), Some(version(24, 11, 0)));
    assert_eq!(NodeVersion::parse("v24.11.0"), Some(version(24, 11, 0)));
}

#[test]
fn rejects_values_that_are_not_exact_versions() {
    for text in ["24", "24.11", "24.11.0.1", "24.x.0", "+24.11.0", "", "lts/*", "v"] {
        assert_eq!(NodeVersion::parse(text), None, "{text}");
    }
}

#[test]
fn compares_by_major_then_minor_then_patch() {
    assert!(version(24, 22, 1) > version(24, 11, 0));
    assert!(version(24, 0, 9) < version(25, 0, 0));
    assert!(version(24, 11, 3) > version(24, 11, 2));
}

#[test]
fn displays_as_dotted_numbers() {
    assert_eq!(version(24, 22, 1).to_string(), "24.22.1");
}

#[test]
fn parses_digits_only() {
    assert_eq!(parse_digits("24"), Some(24));
    for text in ["", "+1", "1.0", "x", "99999999999"] {
        assert_eq!(parse_digits(text), None, "{text}");
    }
}

#[test]
fn classifies_exact_versions_keeping_the_prefix_flag() {
    assert_eq!(classify("24.11.0"), VersionText::Exact { has_prefix: false, version: version(24, 11, 0) });
    assert_eq!(classify("v24.11.0"), VersionText::Exact { has_prefix: true, version: version(24, 11, 0) });
}

#[test]
fn classifies_major_only_values() {
    assert_eq!(classify("24"), VersionText::MajorOnly { has_prefix: false, major: 24 });
    assert_eq!(classify("v24"), VersionText::MajorOnly { has_prefix: true, major: 24 });
}

#[test]
fn classifies_ranges_aliases_and_variables_as_other() {
    for text in [">=22", "^22.0.0", "lts/*", "node", "24.11", "22.x", "${NODE_VERSION}", ""] {
        assert_eq!(classify(text), VersionText::Other, "{text}");
    }
}

#[test]
fn reads_shape_and_major_from_a_classification() {
    let exact = classify("v24.11.0");
    let major_only = classify("22");
    assert_eq!(shape_of(exact), Some((true, VersionShape::Exact)));
    assert_eq!(shape_of(major_only), Some((false, VersionShape::MajorOnly)));
    assert_eq!(shape_of(VersionText::Other), None);
    assert_eq!(major_of(exact), Some(24));
    assert_eq!(major_of(major_only), Some(22));
    assert_eq!(major_of(VersionText::Other), None);
}

#[test]
fn renders_a_version_in_the_original_shape() {
    let target = version(24, 22, 1);
    assert_eq!(render(false, VersionShape::Exact, target), "24.22.1");
    assert_eq!(render(true, VersionShape::Exact, target), "v24.22.1");
    assert_eq!(render(false, VersionShape::MajorOnly, target), "24");
    assert_eq!(render(true, VersionShape::MajorOnly, target), "v24");
}
