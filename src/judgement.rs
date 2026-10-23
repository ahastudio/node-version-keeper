use crate::release_catalog::ReleaseCatalog;
use crate::verdict::{ReportReason, Verdict};
use crate::version::NodeVersion;
use crate::version_text::{VersionShape, VersionText, render};

// Decides, for one value in a file, whether to leave it, upgrade it, or just report it
pub fn judge(classification: VersionText, catalog: &ReleaseCatalog) -> Verdict {
    match classification {
        VersionText::Exact { has_prefix, version } => judge_exact(has_prefix, version, catalog),
        VersionText::MajorOnly { has_prefix, major } => judge_major_only(has_prefix, major, catalog),
        VersionText::Other => Verdict::Report(ReportReason::NotExact),
    }
}

fn judge_exact(has_prefix: bool, version: NodeVersion, catalog: &ReleaseCatalog) -> Verdict {
    if catalog.is_end_of_support(version.major) {
        return end_of_support_verdict(version.major, has_prefix, VersionShape::Exact, catalog);
    }
    same_line_verdict(has_prefix, version, catalog)
}

fn same_line_verdict(has_prefix: bool, version: NodeVersion, catalog: &ReleaseCatalog) -> Verdict {
    match catalog.fixed_version(version.major).filter(|fixed| version < *fixed) {
        Some(fixed) => Verdict::Fix(render(has_prefix, VersionShape::Exact, fixed)),
        None => Verdict::Current,
    }
}

fn judge_major_only(has_prefix: bool, major: u32, catalog: &ReleaseCatalog) -> Verdict {
    if catalog.is_end_of_support(major) {
        return end_of_support_verdict(major, has_prefix, VersionShape::MajorOnly, catalog);
    }
    Verdict::Report(ReportReason::MajorOnly { major })
}

fn end_of_support_verdict(
    major: u32,
    has_prefix: bool,
    shape: VersionShape,
    catalog: &ReleaseCatalog,
) -> Verdict {
    match catalog.upgrade_target(major) {
        Some(target) => Verdict::end_of_support_upgrade(render(has_prefix, shape, target)),
        None => Verdict::Report(ReportReason::EndOfSupport { major }),
    }
}
