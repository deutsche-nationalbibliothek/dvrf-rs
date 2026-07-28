use std::fmt::{self, Display};

pub(crate) type CliResult = Result<(), CliError>;

#[derive(Debug)]
pub(crate) enum CliError {
    IO(std::io::Error),
    Dvrf(dvrf::Error),
}

impl Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::IO(e) => write!(f, "{e}"),
            CliError::Dvrf(e) => write!(f, "{e}"),
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<dvrf::Error> for CliError {
    fn from(e: dvrf::Error) -> Self {
        Self::Dvrf(e)
    }
}
