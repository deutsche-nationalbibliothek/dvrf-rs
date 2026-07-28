use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Error {
    inner: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner {
            ErrorKind::IO(ref e) => write!(f, "{e}"),
            ErrorKind::Json(ref e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
enum ErrorKind {
    IO(std::io::Error),
    Json(serde_json::Error),
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
