use std::fs;
use std::path::Path;

use crate::error::{KeeperError, with_path};

pub fn read_text(path: &Path) -> Result<String, KeeperError> {
    with_path(path, fs::read_to_string(path))
}

pub fn write_text(path: &Path, text: &str) -> Result<(), KeeperError> {
    with_path(path, fs::write(path, text))
}
