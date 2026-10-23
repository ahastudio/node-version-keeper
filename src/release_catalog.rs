use std::collections::{BTreeSet, HashMap, HashSet};

use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::error::KeeperError;
use crate::http_source::ReleaseSource;
use crate::version::{NodeVersion, parse_digits};

pub const RELEASE_INDEX_URL: &str = "https://nodejs.org/dist/index.json";
pub const RELEASE_SCHEDULE_URL: &str = "https://raw.githubusercontent.com/nodejs/Release/main/schedule.json";

#[derive(Deserialize)]
struct IndexEntry {
    version: String,
    security: bool,
}

#[derive(Deserialize)]
struct ScheduleEntry {
    lts: Option<String>,
    end: Option<String>,
}

// Patch version and support status per line, taken from the official release list and schedule
#[derive(Debug)]
pub struct ReleaseCatalog {
    fixed_versions: HashMap<u32, NodeVersion>,
    ended_majors: HashSet<u32>,
    supported_lts_majors: BTreeSet<u32>,
}

pub fn load_catalog(source: &dyn ReleaseSource, today: &str) -> Result<ReleaseCatalog, KeeperError> {
    let index_text = source.fetch_text(RELEASE_INDEX_URL)?;
    let schedule_text = source.fetch_text(RELEASE_SCHEDULE_URL)?;
    ReleaseCatalog::from_documents(&index_text, &schedule_text, today)
}

impl ReleaseCatalog {
    pub fn from_documents(
        index_text: &str,
        schedule_text: &str,
        today: &str,
    ) -> Result<ReleaseCatalog, KeeperError> {
        let index: Vec<IndexEntry> = parse_document(index_text, "release list")?;
        let schedule: HashMap<String, ScheduleEntry> = parse_document(schedule_text, "support schedule")?;
        let fixed_versions = collect_fixed_versions(&index);
        let (ended_majors, supported_lts_majors) = collect_schedule(&schedule, today);
        Ok(ReleaseCatalog { fixed_versions, ended_majors, supported_lts_majors })
    }

    // The most recent security release of that major
    pub fn fixed_version(&self, major: u32) -> Option<NodeVersion> {
        self.fixed_versions.get(&major).copied()
    }

    pub fn is_end_of_support(&self, major: u32) -> bool {
        self.ended_majors.contains(&major)
    }

    // Where to go from an end-of-support line: the patch version of the nearest supported LTS line above it
    pub fn upgrade_target(&self, major: u32) -> Option<NodeVersion> {
        let candidates = self.supported_lts_majors.range(major + 1..);
        candidates.into_iter().find_map(|candidate| self.fixed_version(*candidate))
    }
}

fn parse_document<T: DeserializeOwned>(text: &str, name: &str) -> Result<T, KeeperError> {
    serde_json::from_str(text).map_err(|error| KeeperError::new(format!("failed to parse {name}: {error}")))
}

fn collect_fixed_versions(index: &[IndexEntry]) -> HashMap<u32, NodeVersion> {
    let secured = index.iter().filter(|entry| entry.security);
    let versions = secured.filter_map(|entry| NodeVersion::parse(&entry.version));
    versions.fold(HashMap::new(), keep_highest)
}

fn keep_highest(mut fixed: HashMap<u32, NodeVersion>, version: NodeVersion) -> HashMap<u32, NodeVersion> {
    let highest = fixed.entry(version.major).or_insert(version);
    *highest = (*highest).max(version);
    fixed
}

fn collect_schedule(schedule: &HashMap<String, ScheduleEntry>, today: &str) -> (HashSet<u32>, BTreeSet<u32>) {
    let ended = majors_where(schedule, |entry| is_ended(entry, today));
    let supported_lts = majors_where(schedule, |entry| is_supported_lts(entry, today));
    (ended, supported_lts)
}

fn majors_where<T: FromIterator<u32>>(
    schedule: &HashMap<String, ScheduleEntry>,
    condition: impl Fn(&ScheduleEntry) -> bool,
) -> T {
    let matching = schedule.iter().filter(|(_, entry)| condition(entry));
    matching.filter_map(|(key, _)| parse_major_key(key)).collect()
}

fn parse_major_key(key: &str) -> Option<u32> {
    key.strip_prefix('v').and_then(parse_digits)
}

// Supported through the end-of-support date itself
fn is_ended(entry: &ScheduleEntry, today: &str) -> bool {
    entry.end.as_deref().is_some_and(|end| end < today)
}

fn is_supported_lts(entry: &ScheduleEntry, today: &str) -> bool {
    let started = entry.lts.as_deref().is_some_and(|start| start <= today);
    started && !is_ended(entry, today)
}
