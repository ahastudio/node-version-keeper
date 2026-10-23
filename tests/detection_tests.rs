use std::path::{Path, PathBuf};

use node_version_keeper::detection::{kind_of, project_folder};

fn is_supported(path: &str) -> bool {
    kind_of(Path::new(path)).is_some()
}

#[test]
fn recognizes_files_that_declare_node_versions() {
    for path in [
        "app/.nvmrc",
        "app/.node-version",
        "app/.tool-versions",
        "app/package.json",
        "app/Dockerfile",
        "app/Dockerfile.production",
        "app/web.Dockerfile",
        "app/web.dockerfile",
        "app/.github/workflows/ci.yml",
        "app/.github/workflows/release.yaml",
    ] {
        assert!(is_supported(path), "{path}");
    }
}

#[test]
fn ignores_other_files() {
    for path in [
        "app/README.md",
        "app/workflows/ci.yml",
        "app/.github/ci.yml",
        "app/.github/workflows/ci.txt",
        "ci.yml",
        "..",
    ] {
        assert!(!is_supported(path), "{path}");
    }
}

#[test]
fn project_folder_is_the_folder_of_the_file() {
    assert_eq!(project_folder(Path::new("app/service/.nvmrc")), PathBuf::from("app/service"));
}

#[test]
fn project_folder_of_a_workflow_is_the_folder_holding_dot_github() {
    assert_eq!(project_folder(Path::new("app/.github/workflows/ci.yml")), PathBuf::from("app"));
}

#[test]
fn project_folder_of_a_root_path_is_itself() {
    assert_eq!(project_folder(Path::new("/")), PathBuf::from("/"));
}

#[cfg(unix)]
#[test]
fn a_file_name_that_is_not_utf8_is_not_searched() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    assert!(kind_of(Path::new(OsStr::from_bytes(b"\xff.nvmrc"))).is_none());
}
