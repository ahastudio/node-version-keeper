use std::cmp::Reverse;

use crate::analysis::FileAnalysis;
use crate::error::KeeperError;
use crate::observation::Observation;
use crate::text_file::write_text;
use crate::verdict::ApplyMode;

struct Edit {
    start: usize,
    end: usize,
    replacement: String,
}

// Writes the values chosen to be fixed in this run mode to the file
pub fn apply_all(analyses: &[FileAnalysis], mode: ApplyMode) -> Result<(), KeeperError> {
    analyses.iter().try_for_each(|analysis| apply_to_file(analysis, mode))
}

fn apply_to_file(analysis: &FileAnalysis, mode: ApplyMode) -> Result<(), KeeperError> {
    let edits = edits_of(&analysis.observations, mode);
    if edits.is_empty() {
        return Ok(());
    }
    write_text(&analysis.path, &apply_edits(&analysis.content, &edits))
}

// Sorts so fixes start from the back. Fixing the front first would shift the later offsets
fn edits_of(observations: &[Observation], mode: ApplyMode) -> Vec<Edit> {
    let mut edits: Vec<Edit> =
        observations.iter().filter_map(|observation| edit_of(observation, mode)).collect();
    edits.sort_by_key(|edit| Reverse(edit.start));
    edits
}

fn edit_of(observation: &Observation, mode: ApplyMode) -> Option<Edit> {
    let replacement = observation.verdict.applied_replacement(mode)?;
    let occurrence = &observation.occurrence;
    Some(Edit { start: occurrence.start, end: occurrence.end, replacement: replacement.to_string() })
}

fn apply_edits(content: &str, edits: &[Edit]) -> String {
    let mut updated = content.to_string();
    for edit in edits {
        updated.replace_range(edit.start..edit.end, &edit.replacement);
    }
    updated
}
