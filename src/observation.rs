use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::judgement::judge;
use crate::occurrence::Occurrence;
use crate::release_catalog::ReleaseCatalog;
use crate::verdict::{ReportReason, Verdict};
use crate::version_text::{VersionText, classify};

// One version value found in a file, with its verdict
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Observation {
    pub path: PathBuf,
    pub occurrence: Occurrence,
    pub classification: VersionText,
    pub verdict: Verdict,
}

impl Observation {
    pub fn observed(path: &Path, occurrence: &Occurrence, catalog: &ReleaseCatalog) -> Observation {
        let classification = classify(&occurrence.text);
        let verdict = judge(classification, catalog);
        Observation { path: path.to_path_buf(), occurrence: occurrence.clone(), classification, verdict }
    }

    // A reporting value that points to the whole file when the file could not be parsed
    pub fn unparsable(path: &Path, message: String) -> Observation {
        let file_name = path.file_name().and_then(OsStr::to_str).unwrap_or_default();
        let occurrence = Occurrence { start: 0, end: 0, line: 1, text: file_name.to_string() };
        let verdict = Verdict::Report(ReportReason::Unparsable(message));
        Observation { path: path.to_path_buf(), occurrence, classification: VersionText::Other, verdict }
    }
}
