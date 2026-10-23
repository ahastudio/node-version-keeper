use crate::analysis::FileAnalysis;
use crate::observation::Observation;
use crate::verdict::{ApplyMode, ReportReason, UpgradeCause, Verdict};

type Belongs = fn(&Verdict) -> bool;

const SECTIONS: [(&str, Belongs); 4] = [
    ("End of support", is_end_of_support),
    ("Version mismatch", is_mismatch),
    ("To fix", is_same_major_fix),
    ("Left alone", is_left_alone),
];

pub fn render_report(analyses: &[FileAnalysis], mode: ApplyMode) -> String {
    let observations = problem_observations(analyses);
    let mut blocks = section_blocks(&observations, mode);
    blocks.push(summary(analyses.len(), &observations));
    blocks.join("\n\n") + "\n"
}

// 1 if any value is still unresolved, otherwise 0
pub fn exit_code(analyses: &[FileAnalysis], mode: ApplyMode) -> i32 {
    let observations = problem_observations(analyses);
    let unresolved =
        observations.iter().any(|observation| observation.verdict.applied_replacement(mode).is_none());
    i32::from(unresolved)
}

fn problem_observations(analyses: &[FileAnalysis]) -> Vec<&Observation> {
    let observations = analyses.iter().flat_map(|analysis| analysis.observations.iter());
    observations.filter(|observation| observation.verdict != Verdict::Current).collect()
}

fn section_blocks(observations: &[&Observation], mode: ApplyMode) -> Vec<String> {
    let blocks = SECTIONS.iter().filter_map(|(title, belongs)| section(title, observations, *belongs, mode));
    blocks.collect()
}

fn section(title: &str, observations: &[&Observation], belongs: Belongs, mode: ApplyMode) -> Option<String> {
    let lines = lines_of(observations, belongs, mode);
    (!lines.is_empty()).then(|| format!("{title}\n{}", lines.join("\n")))
}

fn lines_of(observations: &[&Observation], belongs: Belongs, mode: ApplyMode) -> Vec<String> {
    let members = observations.iter().filter(|observation| belongs(&observation.verdict));
    members.map(|observation| line_of(observation, mode)).collect()
}

fn line_of(observation: &Observation, mode: ApplyMode) -> String {
    let location = format!("{}:{}", observation.path.display(), observation.occurrence.line);
    format!("  {location}  {}  {}", observation.occurrence.text, describe(&observation.verdict, mode))
}

pub fn describe(verdict: &Verdict, mode: ApplyMode) -> String {
    match verdict {
        Verdict::Fix(replacement) => format!("→ {replacement} ({})", fix_status(mode)),
        Verdict::MajorUpgrade { replacement, .. } => describe_upgrade(replacement, mode),
        other => describe_without_replacement(other),
    }
}

fn describe_upgrade(replacement: &str, mode: ApplyMode) -> String {
    format!("→ {replacement} (major upgrade, {})", upgrade_status(mode))
}

fn describe_without_replacement(verdict: &Verdict) -> String {
    match verdict {
        Verdict::Report(reason) => reason.describe(),
        _ => "nothing to fix".to_string(),
    }
}

fn fix_status(mode: ApplyMode) -> &'static str {
    if mode.write { "fixed" } else { "--write to fix" }
}

fn upgrade_status(mode: ApplyMode) -> &'static str {
    match (mode.write, mode.upgrade_major) {
        (true, true) => "fixed",
        (true, false) => "--upgrade-major required",
        (false, _) => "--write and --upgrade-major to fix",
    }
}

fn summary(file_count: usize, observations: &[&Observation]) -> String {
    format!("Summary: {file_count} files checked, {}", counts_text(observations))
}

fn counts_text(observations: &[&Observation]) -> String {
    let phrases = SECTIONS.iter().map(|(title, belongs)| count_phrase(title, observations, *belongs));
    phrases.collect::<Vec<_>>().join(", ")
}

fn count_phrase(title: &str, observations: &[&Observation], belongs: Belongs) -> String {
    let count = observations.iter().filter(|observation| belongs(&observation.verdict)).count();
    format!("{title} {count}")
}

fn is_end_of_support(verdict: &Verdict) -> bool {
    matches!(
        verdict,
        Verdict::Report(ReportReason::EndOfSupport { .. })
            | Verdict::MajorUpgrade { cause: UpgradeCause::EndOfSupport, .. }
    )
}

fn is_mismatch(verdict: &Verdict) -> bool {
    matches!(verdict, Verdict::MajorUpgrade { cause: UpgradeCause::Mismatch, .. })
}

fn is_same_major_fix(verdict: &Verdict) -> bool {
    matches!(verdict, Verdict::Fix(_))
}

fn is_left_alone(verdict: &Verdict) -> bool {
    matches!(verdict, Verdict::Report(reason) if !matches!(reason, ReportReason::EndOfSupport { .. }))
}
