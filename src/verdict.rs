#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeCause {
    EndOfSupport,
    Mismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportReason {
    EndOfSupport { major: u32 },
    MajorOnly { major: u32 },
    NotExact,
    Unparsable(String),
}

// The verdict on one value. Current means there is nothing to fix
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Current,
    Fix(String),
    MajorUpgrade { replacement: String, cause: UpgradeCause },
    Report(ReportReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyMode {
    pub write: bool,
    pub upgrade_major: bool,
}

impl ApplyMode {
    pub fn allows_major_upgrade(&self) -> bool {
        self.write && self.upgrade_major
    }
}

impl Verdict {
    pub fn end_of_support_upgrade(replacement: String) -> Verdict {
        Verdict::MajorUpgrade { replacement, cause: UpgradeCause::EndOfSupport }
    }

    // The value actually written to the file in this run mode. None if nothing is written
    pub fn applied_replacement(&self, mode: ApplyMode) -> Option<&str> {
        match self {
            Verdict::Fix(replacement) if mode.write => Some(replacement),
            Verdict::MajorUpgrade { replacement, .. } => mode.allows_major_upgrade().then_some(replacement),
            _ => None,
        }
    }
}

impl ReportReason {
    pub fn describe(&self) -> String {
        match self {
            Self::EndOfSupport { major } => {
                format!("Node.js {major}.x is end of support and has no line to upgrade to")
            }
            Self::MajorOnly { major } => format!("only major {major} is given, left unchanged"),
            other => other.describe_unfixable(),
        }
    }

    fn describe_unfixable(&self) -> String {
        match self {
            Self::Unparsable(message) => format!("could not read file: {message}"),
            _ => "not an exact version (range, alias, or variable), left unchanged".to_string(),
        }
    }
}
