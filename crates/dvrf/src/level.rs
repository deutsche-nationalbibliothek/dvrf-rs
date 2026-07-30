use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// The level of an error (error, warning, or info).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    #[default]
    Error,
    Warning,
    Info,
}

/// An error type used for the [FromStr] implementation for [Level].
#[derive(Debug, PartialEq, Eq)]
pub struct ParseLevelError;

impl Display for ParseLevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "parse level error")
    }
}

impl std::error::Error for ParseLevelError {}

impl FromStr for Level {
    type Err = ParseLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "error" | "err" => Ok(Level::Error),
            "warning" | "warn" => Ok(Level::Warning),
            "info" => Ok(Level::Info),
            _ => Err(ParseLevelError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_from_str() {
        assert_eq!(Level::from_str("error").unwrap(), Level::Error);
        assert_eq!(Level::from_str("warning").unwrap(), Level::Warning);
        assert_eq!(Level::from_str("info").unwrap(), Level::Info);
        assert_eq!(Level::from_str("err").unwrap(), Level::Error);
        assert_eq!(Level::from_str("warn").unwrap(), Level::Warning);

        assert!(Level::from_str("fehler").is_err());
    }
}
