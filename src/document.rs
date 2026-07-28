use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};

use crate::{Error, Record};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Document(Vec<Record>);

impl Document {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn write_record(&mut self, record: Record) {
        self.0.push(record);
    }

    pub fn records(&self) -> impl Iterator<Item = &Record> {
        self.0.iter()
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let filename = path
            .as_ref()
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or_default();

        let rdr: Box<dyn Read> = if filename.ends_with(".gz") {
            Box::new(GzDecoder::new(File::open(path)?))
        } else {
            Box::new(File::open(path)?)
        };

        Ok(serde_json::from_reader(rdr)?)
    }

    pub fn write_to<W: Write>(
        &self,
        wtr: W,
        pretty: bool,
    ) -> Result<(), Error> {
        if pretty {
            serde_json::to_writer_pretty(wtr, self)?
        } else {
            serde_json::to_writer(wtr, self)?;
        }

        Ok(())
    }
}
