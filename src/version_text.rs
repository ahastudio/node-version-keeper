use crate::version::{NodeVersion, parse_digits};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionText {
    Exact { has_prefix: bool, version: NodeVersion },
    MajorOnly { has_prefix: bool, major: u32 },
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionShape {
    Exact,
    MajorOnly,
}

// Classifies a version string in a file as an exact version, a major-only value, or anything else
pub fn classify(text: &str) -> VersionText {
    if let Some(version) = NodeVersion::parse(text) {
        return VersionText::Exact { has_prefix: text.starts_with('v'), version };
    }
    major_only_or_other(text)
}

fn major_only_or_other(text: &str) -> VersionText {
    let has_prefix = text.starts_with('v');
    match parse_digits(text.strip_prefix('v').unwrap_or(text)) {
        Some(major) => VersionText::MajorOnly { has_prefix, major },
        None => VersionText::Other,
    }
}

pub fn shape_of(classification: VersionText) -> Option<(bool, VersionShape)> {
    match classification {
        VersionText::Exact { has_prefix, .. } => Some((has_prefix, VersionShape::Exact)),
        VersionText::MajorOnly { has_prefix, .. } => Some((has_prefix, VersionShape::MajorOnly)),
        VersionText::Other => None,
    }
}

pub fn major_of(classification: VersionText) -> Option<u32> {
    match classification {
        VersionText::Exact { version, .. } => Some(version.major),
        VersionText::MajorOnly { major, .. } => Some(major),
        VersionText::Other => None,
    }
}

// Writes the new version, keeping the shape of the original value (v prefix, exact version, or major only)
pub fn render(has_prefix: bool, shape: VersionShape, version: NodeVersion) -> String {
    let prefix = if has_prefix { "v" } else { "" };
    match shape {
        VersionShape::Exact => format!("{prefix}{version}"),
        VersionShape::MajorOnly => format!("{prefix}{}", version.major),
    }
}
