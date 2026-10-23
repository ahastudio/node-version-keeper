use std::path::{Path, PathBuf};

use crate::detection::{FileKind, kind_of};
use crate::error::KeeperError;
use crate::observation::Observation;
use crate::occurrence::Occurrence;
use crate::release_catalog::ReleaseCatalog;
use crate::text_file::read_text;

// The result of reading one file. Also holds the file content for use when fixing
#[derive(Debug)]
pub struct FileAnalysis {
    pub path: PathBuf,
    pub content: String,
    pub observations: Vec<Observation>,
}

pub fn analyze_file(path: &Path, catalog: &ReleaseCatalog) -> Result<FileAnalysis, KeeperError> {
    let kind = kind_of(path).ok_or_else(|| unsupported_file(path))?;
    let content = read_text(path)?;
    let observations = observe(path, kind, &content, catalog);
    Ok(FileAnalysis { path: path.to_path_buf(), content, observations })
}

fn unsupported_file(path: &Path) -> KeeperError {
    KeeperError::new(format!("{}: not a file to look for versions in", path.display()))
}

fn observe(path: &Path, kind: FileKind, content: &str, catalog: &ReleaseCatalog) -> Vec<Observation> {
    match (kind.find)(content) {
        Ok(occurrences) => observe_all(path, &occurrences, catalog),
        Err(message) => vec![Observation::unparsable(path, message)],
    }
}

fn observe_all(path: &Path, occurrences: &[Occurrence], catalog: &ReleaseCatalog) -> Vec<Observation> {
    let observations = occurrences.iter().map(|occurrence| Observation::observed(path, occurrence, catalog));
    observations.collect()
}
