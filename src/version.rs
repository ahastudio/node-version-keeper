use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl NodeVersion {
    // Reads the "24.11.0" and "v24.11.0" forms
    pub fn parse(text: &str) -> Option<NodeVersion> {
        let mut parts = text.strip_prefix('v').unwrap_or(text).split('.');
        let [major, minor, patch] = [parts.next(), parts.next(), parts.next()].map(parse_part);
        parts.next().is_none().then_some(())?;
        Some(NodeVersion { major: major?, minor: minor?, patch: patch? })
    }
}

impl fmt::Display for NodeVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

fn parse_part(part: Option<&str>) -> Option<u32> {
    parse_digits(part?)
}

// Reads a string made only of digits. Signs and empty strings are rejected
pub fn parse_digits(text: &str) -> Option<u32> {
    let all_digits = text.bytes().all(|byte| byte.is_ascii_digit());
    all_digits.then(|| text.parse().ok()).flatten()
}
