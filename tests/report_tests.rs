use node_version_keeper::report::describe;
use node_version_keeper::verdict::{ApplyMode, ReportReason, UpgradeCause, Verdict};

const PREVIEW: ApplyMode = ApplyMode { write: false, upgrade_major: false };
const WRITE_ONLY: ApplyMode = ApplyMode { write: true, upgrade_major: false };
const WRITE_AND_UPGRADE: ApplyMode = ApplyMode { write: true, upgrade_major: true };

fn major_upgrade() -> Verdict {
    Verdict::MajorUpgrade { replacement: "24.22.1".to_string(), cause: UpgradeCause::Mismatch }
}

#[test]
fn describes_a_value_that_needs_nothing() {
    assert_eq!(describe(&Verdict::Current, PREVIEW), "nothing to fix");
}

#[test]
fn describes_a_same_major_fix_by_mode() {
    let fix = Verdict::Fix("24.22.1".to_string());
    assert_eq!(describe(&fix, PREVIEW), "→ 24.22.1 (--write to fix)");
    assert_eq!(describe(&fix, WRITE_ONLY), "→ 24.22.1 (fixed)");
}

#[test]
fn describes_a_major_upgrade_by_mode() {
    assert_eq!(
        describe(&major_upgrade(), PREVIEW),
        "→ 24.22.1 (major upgrade, --write and --upgrade-major to fix)"
    );
    assert_eq!(describe(&major_upgrade(), WRITE_ONLY), "→ 24.22.1 (major upgrade, --upgrade-major required)");
    assert_eq!(describe(&major_upgrade(), WRITE_AND_UPGRADE), "→ 24.22.1 (major upgrade, fixed)");
}

#[test]
fn describes_every_report_reason() {
    let end_of_support = Verdict::Report(ReportReason::EndOfSupport { major: 18 });
    let major_only = Verdict::Report(ReportReason::MajorOnly { major: 24 });
    let not_exact = Verdict::Report(ReportReason::NotExact);
    let unparsable = Verdict::Report(ReportReason::Unparsable("broken".to_string()));
    assert_eq!(
        describe(&end_of_support, PREVIEW),
        "Node.js 18.x is end of support and has no line to upgrade to"
    );
    assert_eq!(describe(&major_only, PREVIEW), "only major 24 is given, left unchanged");
    assert_eq!(
        describe(&not_exact, PREVIEW),
        "not an exact version (range, alias, or variable), left unchanged"
    );
    assert_eq!(describe(&unparsable, PREVIEW), "could not read file: broken");
}

#[test]
fn only_applies_what_the_mode_allows() {
    let fix = Verdict::Fix("24.22.1".to_string());
    assert_eq!(fix.applied_replacement(PREVIEW), None);
    assert_eq!(fix.applied_replacement(WRITE_ONLY), Some("24.22.1"));
    assert_eq!(major_upgrade().applied_replacement(WRITE_ONLY), None);
    assert_eq!(major_upgrade().applied_replacement(WRITE_AND_UPGRADE), Some("24.22.1"));
    assert_eq!(Verdict::Current.applied_replacement(WRITE_AND_UPGRADE), None);
}
