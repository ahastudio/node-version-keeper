use std::collections::HashSet;
use std::path::Path;

use serde::Deserialize;

use crate::error::KeeperError;
use crate::text_file::read_text;

// Folders that are always excluded, whatever the config
pub const ALWAYS_EXCLUDED: [&str; 2] = ["node_modules", ".git"];

const DEFAULT_CONFIG_TEXT: &str = include_str!("../node-version-keeper-config.json");

#[derive(Debug, Deserialize)]
pub struct Config {
    pub exclude: Vec<String>,
}

// Without a path, uses the default config file bundled with the tool
pub fn load_config(path: Option<&Path>) -> Result<Config, KeeperError> {
    let text = match path {
        Some(config_path) => read_text(config_path)?,
        None => DEFAULT_CONFIG_TEXT.to_string(),
    };
    parse_config(&text)
}

pub fn excluded_names(config: &Config, extra: &[String]) -> HashSet<String> {
    let built_in = ALWAYS_EXCLUDED.iter().map(|name| name.to_string());
    built_in.chain(config.exclude.iter().cloned()).chain(extra.iter().cloned()).collect()
}

fn parse_config(text: &str) -> Result<Config, KeeperError> {
    serde_json::from_str(text)
        .map_err(|error| KeeperError::new(format!("failed to parse config file: {error}")))
}
