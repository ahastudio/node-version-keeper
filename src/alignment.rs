use std::collections::HashMap;
use std::path::PathBuf;

use crate::analysis::FileAnalysis;
use crate::detection::project_folder;
use crate::observation::Observation;
use crate::release_catalog::ReleaseCatalog;
use crate::verdict::{UpgradeCause, Verdict};
use crate::version::NodeVersion;
use crate::version_text::{classify, major_of, render, shape_of};

// In the list of analysis results, (file index, value index within that file)
type Member = (usize, usize);

// If majors differ within the same project, align to the highest major
pub fn align_projects(analyses: &mut [FileAnalysis], catalog: &ReleaseCatalog) {
    for members in group_by_project(analyses).into_values() {
        align_group(analyses, &members, catalog);
    }
}

fn group_by_project(analyses: &[FileAnalysis]) -> HashMap<PathBuf, Vec<Member>> {
    let mut groups: HashMap<PathBuf, Vec<Member>> = HashMap::new();
    for (analysis_index, analysis) in analyses.iter().enumerate() {
        add_members(&mut groups, analysis_index, analysis);
    }
    groups
}

fn add_members(groups: &mut HashMap<PathBuf, Vec<Member>>, analysis_index: usize, analysis: &FileAnalysis) {
    let members = groups.entry(project_folder(&analysis.path)).or_default();
    members.extend((0..analysis.observations.len()).map(|position| (analysis_index, position)));
}

fn align_group(analyses: &mut [FileAnalysis], members: &[Member], catalog: &ReleaseCatalog) {
    let target = highest_major(analyses, members).and_then(|highest| catalog.fixed_version(highest));
    if let Some(target) = target {
        align_members(analyses, members, target);
    }
}

fn highest_major(analyses: &[FileAnalysis], members: &[Member]) -> Option<u32> {
    let majors =
        members.iter().filter_map(|(file, position)| final_major(&analyses[*file].observations[*position]));
    majors.max()
}

// The major this value will have after its individual verdict is applied. None if the value is not a version
fn final_major(observation: &Observation) -> Option<u32> {
    match &observation.verdict {
        Verdict::Fix(text) | Verdict::MajorUpgrade { replacement: text, .. } => major_of(classify(text)),
        _ => major_of(observation.classification),
    }
}

fn align_members(analyses: &mut [FileAnalysis], members: &[Member], target: NodeVersion) {
    for (file, position) in members {
        align_one(&mut analyses[*file].observations[*position], target);
    }
}

fn align_one(observation: &mut Observation, target: NodeVersion) {
    if let Some(verdict) = upgraded_verdict_if_lower(observation, target) {
        observation.verdict = verdict;
    }
}

fn upgraded_verdict_if_lower(observation: &Observation, target: NodeVersion) -> Option<Verdict> {
    let lower = final_major(observation).is_some_and(|major| major < target.major);
    let (has_prefix, shape) = shape_of(observation.classification).filter(|_| lower)?;
    let cause = upgrade_cause(&observation.verdict);
    Some(Verdict::MajorUpgrade { replacement: render(has_prefix, shape, target), cause })
}

fn upgrade_cause(verdict: &Verdict) -> UpgradeCause {
    match verdict {
        Verdict::MajorUpgrade { cause, .. } => *cause,
        _ => UpgradeCause::Mismatch,
    }
}
