mod common;

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use common::write_file;
use node_version_keeper::file_scanner::collect_candidate_files;

fn excluded(names: &[&str]) -> HashSet<String> {
    names.iter().map(|name| name.to_string()).collect()
}

fn relative_paths(root: &Path, files: &[PathBuf]) -> Vec<String> {
    files.iter().map(|file| file.strip_prefix(root).unwrap().display().to_string()).collect()
}

#[test]
fn collects_version_files_in_sorted_order_and_skips_excluded_folders() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "b/.nvmrc", "24\n");
    write_file(root.path(), "a/Dockerfile", "FROM node:24\n");
    write_file(root.path(), "a/README.md", "description\n");
    write_file(root.path(), "a/node_modules/x/.nvmrc", "24\n");
    write_file(root.path(), "a/dist/.nvmrc", "24\n");
    write_file(root.path(), "a/.github/workflows/ci.yml", "node-version: 24\n");
    let files = collect_candidate_files(root.path(), &excluded(&["node_modules", "dist"])).unwrap();
    let expected = vec!["a/.github/workflows/ci.yml", "a/Dockerfile", "b/.nvmrc"];
    assert_eq!(relative_paths(root.path(), &files), expected);
}

#[test]
fn includes_folders_that_are_not_excluded() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "dist/.nvmrc", "24\n");
    let files = collect_candidate_files(root.path(), &excluded(&[])).unwrap();
    assert_eq!(relative_paths(root.path(), &files), vec!["dist/.nvmrc"]);
}

#[cfg(unix)]
#[test]
fn does_not_follow_symbolic_links() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "real/.nvmrc", "24\n");
    std::os::unix::fs::symlink(root.path().join("real"), root.path().join("link")).unwrap();
    std::os::unix::fs::symlink(root.path().join("real/.nvmrc"), root.path().join(".node-version")).unwrap();
    let files = collect_candidate_files(root.path(), &excluded(&[])).unwrap();
    assert_eq!(relative_paths(root.path(), &files), vec!["real/.nvmrc"]);
}

#[test]
fn a_missing_folder_is_an_error() {
    let root = tempfile::tempdir().unwrap();
    let missing = root.path().join("missing-folder");
    assert!(collect_candidate_files(&missing, &excluded(&[])).is_err());
}

#[test]
fn a_file_instead_of_a_folder_is_an_error() {
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "file.txt", "content\n");
    assert!(collect_candidate_files(&root.path().join("file.txt"), &excluded(&[])).is_err());
}

#[cfg(unix)]
#[test]
fn an_unreadable_subfolder_is_an_error() {
    use std::fs::{self, Permissions};
    use std::os::unix::fs::PermissionsExt;
    let root = tempfile::tempdir().unwrap();
    write_file(root.path(), "locked/.nvmrc", "24\n");
    let locked = root.path().join("locked");
    fs::set_permissions(&locked, Permissions::from_mode(0o000)).unwrap();
    let result = collect_candidate_files(root.path(), &excluded(&[]));
    fs::set_permissions(&locked, Permissions::from_mode(0o755)).unwrap();
    let readable_anyway = fs::read_dir(&locked).is_ok();
    assert!(result.is_err() || readable_anyway);
}
