#![allow(dead_code)]

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use node_version_keeper::alignment::align_projects;
use node_version_keeper::analysis::{FileAnalysis, analyze_file};
use node_version_keeper::command::run;
use node_version_keeper::error::KeeperError;
use node_version_keeper::http_source::ReleaseSource;
use node_version_keeper::release_catalog::{RELEASE_INDEX_URL, RELEASE_SCHEDULE_URL, ReleaseCatalog};
use node_version_keeper::verdict::Verdict;

pub const TODAY: &str = "2026-10-22";

pub const INDEX_JSON: &str = r#"[
  {"version":"v26.12.1","date":"2026-10-21","security":true},
  {"version":"v26.12.0","date":"2026-10-14","security":false},
  {"version":"v24.22.1","date":"2026-10-21","security":true},
  {"version":"v24.22.0","date":"2026-10-01","security":false},
  {"version":"v24.11.0","date":"2025-10-28","security":false},
  {"version":"v22.23.4","date":"2026-10-21","security":true},
  {"version":"v22.23.3","date":"2026-09-01","security":true},
  {"version":"v20.20.2","date":"2026-03-01","security":true},
  {"version":"v18.20.8","date":"2025-03-27","security":true},
  {"version":"v0.10.48","date":"2016-10-01","security":true},
  {"version":"nightly","date":"2026-10-22","security":true}
]"#;

pub const SCHEDULE_JSON: &str = r#"{
  "v0.10": {"start":"2013-03-11","end":"2016-10-31"},
  "v18": {"start":"2022-04-19","lts":"2022-10-25","maintenance":"2023-10-18","end":"2025-04-30"},
  "v20": {"start":"2023-04-18","lts":"2023-10-24","maintenance":"2024-10-22","end":"2026-04-30"},
  "v22": {"start":"2024-04-24","lts":"2024-10-29","maintenance":"2025-10-21","end":"2027-04-30"},
  "v23": {"start":"2024-10-16","maintenance":"2025-04-01","end":"2025-06-01"},
  "v24": {"start":"2025-05-06","lts":"2025-10-28","maintenance":"2026-10-20","end":"2028-04-30"},
  "v26": {"start":"2026-04-22","lts":"2026-10-28","maintenance":"2027-10-20","end":"2029-04-30"},
  "v28": {"start":"2026-12-01","lts":"2026-01-01","end":"2030-01-01"},
  "v29": {"start":"2027-01-01"}
}"#;

pub fn sample_catalog() -> ReleaseCatalog {
    ReleaseCatalog::from_documents(INDEX_JSON, SCHEDULE_JSON, TODAY).expect("sample documents should parse")
}

pub struct FakeReleaseSource {
    pub index_text: String,
    pub schedule_text: String,
    pub failing_url: Option<&'static str>,
}

impl FakeReleaseSource {
    pub fn sample() -> FakeReleaseSource {
        FakeReleaseSource {
            index_text: INDEX_JSON.to_string(),
            schedule_text: SCHEDULE_JSON.to_string(),
            failing_url: None,
        }
    }

    pub fn failing(url: &'static str) -> FakeReleaseSource {
        FakeReleaseSource { failing_url: Some(url), ..FakeReleaseSource::sample() }
    }
}

impl ReleaseSource for FakeReleaseSource {
    fn fetch_text(&self, url: &str) -> Result<String, KeeperError> {
        if Some(url) == self.failing_url {
            return Err(KeeperError::new(format!("connection to {url} failed")));
        }
        match url {
            RELEASE_INDEX_URL => Ok(self.index_text.clone()),
            RELEASE_SCHEDULE_URL => Ok(self.schedule_text.clone()),
            other => Err(KeeperError::new(format!("unknown address: {other}"))),
        }
    }
}

pub struct FailingWriter;

impl Write for FailingWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("cannot write"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn write_file(root: &Path, relative_path: &str, content: &str) {
    let path = root.join(relative_path);
    fs::create_dir_all(path.parent().expect("should have a parent folder"))
        .expect("should create the folder");
    fs::write(path, content).expect("should write the file");
}

pub fn read_file(root: &Path, relative_path: &str) -> String {
    fs::read_to_string(root.join(relative_path)).expect("should read the file")
}

pub fn analyze_project(root: &Path, relative_paths: &[&str], catalog: &ReleaseCatalog) -> Vec<FileAnalysis> {
    let mut analyses: Vec<FileAnalysis> = relative_paths
        .iter()
        .map(|relative_path| analyze_file(&root.join(relative_path), catalog).expect("should analyze"))
        .collect();
    align_projects(&mut analyses, catalog);
    analyses
}

pub fn verdicts_of(analysis: &FileAnalysis) -> Vec<Verdict> {
    analysis.observations.iter().map(|observation| observation.verdict.clone()).collect()
}

pub fn run_command(arguments: &[&str], source: &dyn ReleaseSource) -> (Result<i32, KeeperError>, String) {
    let owned: Vec<String> = arguments.iter().map(|argument| argument.to_string()).collect();
    let mut output = Vec::new();
    let outcome = run(&owned, source, TODAY, &mut output);
    (outcome, String::from_utf8(output).expect("output must be UTF-8"))
}

// A project used for preview descriptions. Its majors differ, and it mixes range expressions with major-only values
pub fn write_messy_project(root: &Path) {
    write_file(root, "app/.nvmrc", "24.11.0\n");
    write_file(root, "app/Dockerfile", "FROM node:22.4.0-alpine AS build\nFROM node:24\n");
    write_file(
        root,
        "app/package.json",
        "{\n  \"name\": \"app\",\n  \"engines\": { \"node\": \">=22\" },\n  \"volta\": { \"node\": \"20.10.0\" }\n}\n",
    );
    write_file(
        root,
        "app/.github/workflows/ci.yml",
        "jobs:\n  test:\n    steps:\n      - uses: actions/setup-node@v4\n        with:\n          node-version: [18, 22.4.0]\n",
    );
    write_file(root, "legacy/.nvmrc", "18.20.4\n");
    write_file(root, "app/node_modules/x/.nvmrc", "18.20.4\n");
    write_file(root, "app/dist/.nvmrc", "18.20.4\n");
}
