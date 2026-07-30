use std::fmt::{self, Display};

use crate::{CondenseError, ParseLevelError};

/// An error that can occur in this crate.
#[derive(Debug)]
pub struct Error {
    inner: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner {
            ErrorKind::Condense(ref e) => write!(f, "{e}"),
            ErrorKind::Level(ref e) => write!(f, "{e}"),
            ErrorKind::Json(ref e) => write!(f, "{e}"),
            ErrorKind::IO(ref e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
enum ErrorKind {
    Condense(CondenseError),
    Level(ParseLevelError),
    Json(serde_json::Error),
    IO(std::io::Error),
}

impl From<ParseLevelError> for Error {
    fn from(e: ParseLevelError) -> Self {
        Error {
            inner: ErrorKind::Level(e),
        }
    }
}

impl From<CondenseError> for Error {
    fn from(e: CondenseError) -> Self {
        Error {
            inner: ErrorKind::Condense(e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error {
            inner: ErrorKind::IO(e),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error {
            inner: ErrorKind::Json(e),
        }
    }
}
