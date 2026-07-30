//! This project provides a library (this crate) for processing the
//! [Data Validation Report Format (DVRF)] in the Rust programming
//! language. In addition to the library, it also provides the [dvrf]
//! command-line tool, which offers useful commands for analysis and
//! further processing.
//!
//! [Data Validation Report Format (DVRF)]: https://gbv.github.io/data-validation-report-format
//! [dvrf]: https://crates.io/crates/dvrf-cli

mod document;
mod error;
mod level;
mod locator;
mod position;
mod record;

pub use document::Document;
pub use error::Error;
pub use level::{Level, ParseLevelError};
pub use locator::{CondenseError, Locator, LocatorMap, Locators};
pub use position::Position;
pub use record::Record;
