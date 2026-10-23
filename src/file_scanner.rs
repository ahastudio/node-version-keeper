use std::collections::HashSet;
use std::fs::{self, FileType};
use std::path::{Path, PathBuf};

use crate::detection::kind_of;
use crate::error::{KeeperError, with_path};

enum EntryKind {
    Directory,
    File,
    Other,
}

// Collects every file under the folder that is searched for versions. Symbolic links are not followed
pub fn collect_candidate_files(root: &Path, excluded: &HashSet<String>) -> Result<Vec<PathBuf>, KeeperError> {
    let mut files = collect_files(root, excluded)?;
    files.sort();
    Ok(files)
}

fn collect_files(directory: &Path, excluded: &HashSet<String>) -> Result<Vec<PathBuf>, KeeperError> {
    let mut files = Vec::new();
    for (path, file_type) in read_directory(directory)? {
        files.extend(files_of_entry(path, file_type, excluded)?);
    }
    Ok(files)
}

fn read_directory(directory: &Path) -> Result<Vec<(PathBuf, FileType)>, KeeperError> {
    let entries = with_path(directory, fs::read_dir(directory))?;
    with_path(
        directory,
        entries
            .map(|entry| entry.and_then(|entry| entry.file_type().map(|file_type| (entry.path(), file_type))))
            .collect(),
    )
}

fn files_of_entry(
    path: PathBuf,
    file_type: FileType,
    excluded: &HashSet<String>,
) -> Result<Vec<PathBuf>, KeeperError> {
    match kind_of_file_type(file_type) {
        EntryKind::Directory => files_of_directory(path, excluded),
        EntryKind::File => Ok(supported_file(path)),
        EntryKind::Other => Ok(Vec::new()),
    }
}

fn kind_of_file_type(file_type: FileType) -> EntryKind {
    match (file_type.is_dir(), file_type.is_file()) {
        (true, _) => EntryKind::Directory,
        (_, true) => EntryKind::File,
        _ => EntryKind::Other,
    }
}

fn files_of_directory(path: PathBuf, excluded: &HashSet<String>) -> Result<Vec<PathBuf>, KeeperError> {
    if is_excluded(&path, excluded) {
        return Ok(Vec::new());
    }
    collect_files(&path, excluded)
}

fn is_excluded(path: &Path, excluded: &HashSet<String>) -> bool {
    let name = path.file_name().and_then(|file_name| file_name.to_str());
    name.is_some_and(|name| excluded.contains(name))
}

fn supported_file(path: PathBuf) -> Vec<PathBuf> {
    if kind_of(&path).is_some() {
        return vec![path];
    }
    Vec::new()
}
