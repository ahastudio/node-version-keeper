use std::path::{Path, PathBuf};

use crate::occurrence::Finder;
use crate::{dockerfile_detector, package_json_detector, plain_version_detector};
use crate::{tool_versions_detector, workflow_detector};

#[derive(Clone, Copy)]
pub struct FileKind {
    pub find: Finder,
}

const PLAIN_VERSION: FileKind = FileKind { find: plain_version_detector::find };
const TOOL_VERSIONS: FileKind = FileKind { find: tool_versions_detector::find };
const PACKAGE_JSON: FileKind = FileKind { find: package_json_detector::find };
const DOCKERFILE: FileKind = FileKind { find: dockerfile_detector::find };
const WORKFLOW: FileKind = FileKind { find: workflow_detector::find };

const NAMED_KINDS: [(&str, FileKind); 4] = [
    (".nvmrc", PLAIN_VERSION),
    (".node-version", PLAIN_VERSION),
    (".tool-versions", TOOL_VERSIONS),
    ("package.json", PACKAGE_JSON),
];

// Decides whether the tool looks for versions in this file. None if it does not apply
pub fn kind_of(path: &Path) -> Option<FileKind> {
    let name = path.file_name()?.to_str()?;
    match_file_name(name).or_else(|| match_dockerfile(name)).or_else(|| match_workflow(path, name))
}

// The folder that groups files into one project. A workflow belongs to the folder that contains `.github`
pub fn project_folder(path: &Path) -> PathBuf {
    let levels = if in_workflows_folder(path) { 3 } else { 1 };
    path.ancestors().nth(levels).unwrap_or(path).to_path_buf()
}

fn match_file_name(name: &str) -> Option<FileKind> {
    NAMED_KINDS.iter().find(|(candidate, _)| *candidate == name).map(|(_, kind)| *kind)
}

fn match_dockerfile(name: &str) -> Option<FileKind> {
    let is_dockerfile = name.starts_with("Dockerfile") || name.to_lowercase().ends_with(".dockerfile");
    is_dockerfile.then_some(DOCKERFILE)
}

fn match_workflow(path: &Path, name: &str) -> Option<FileKind> {
    let is_yaml = name.ends_with(".yml") || name.ends_with(".yaml");
    (is_yaml && in_workflows_folder(path)).then_some(WORKFLOW)
}

fn in_workflows_folder(path: &Path) -> bool {
    let folders = path.ancestors().skip(1).take(2).filter_map(|ancestor| ancestor.file_name()?.to_str());
    folders.eq(["workflows", ".github"])
}
