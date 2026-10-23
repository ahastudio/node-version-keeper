use std::fmt::Display;

use crate::error::KeeperError;

pub trait ReleaseSource {
    fn fetch_text(&self, url: &str) -> Result<String, KeeperError>;
}

pub struct HttpReleaseSource;

impl ReleaseSource for HttpReleaseSource {
    fn fetch_text(&self, url: &str) -> Result<String, KeeperError> {
        let mut response = ureq::get(url).call().map_err(|error| network_error(url, error))?;
        response.body_mut().read_to_string().map_err(|error| network_error(url, error))
    }
}

fn network_error(url: &str, error: impl Display) -> KeeperError {
    KeeperError::new(format!("request to {url} failed: {error}"))
}
