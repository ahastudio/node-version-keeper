use std::fmt;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct KeeperError {
    message: String,
}

impl KeeperError {
    pub fn new(message: impl Into<String>) -> KeeperError {
        KeeperError { message: message.into() }
    }
}

impl fmt::Display for KeeperError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for KeeperError {}

// If an I/O result is a failure, prefix the error message with the target path
pub fn with_path<T>(path: &Path, result: std::io::Result<T>) -> Result<T, KeeperError> {
    result.map_err(|error| KeeperError::new(format!("{}: {error}", path.display())))
}
