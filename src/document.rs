use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};

use crate::{Error, Record};

/// A collection of error records.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Document(Vec<Record>);

impl Document {
    /// Creates a new document with an empty list of error records.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Document;
    ///
    /// let doc = Document::new();
    /// assert_eq!(doc.records().count(), 0);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new record to the document.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::{Document, Record};
    ///
    /// let mut doc = Document::new();
    /// doc.write_record(Record::new().with_message("foo"));
    /// assert_eq!(doc.records().count(), 1);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn write_record(&mut self, record: Record) {
        self.0.push(record);
    }

    /// Returns an iterator over the records of the documents.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::{Document, Record};
    ///
    /// let mut doc = Document::new();
    /// doc.write_record(Record::new().with_message("foo"));
    /// assert_eq!(doc.records().count(), 1);
    ///
    /// let records: Vec<_> = doc.records().collect();
    /// assert_eq!(records[0].message().unwrap(), "foo");
    /// assert_eq!(records.len(), 1);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn records(&self) -> impl Iterator<Item = &Record> {
        self.0.iter()
    }

    /// Reads the document from the given path
    ///
    /// If the filename ends with the suffix `.gz` the file is
    /// automatically decompressed.)
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Document;
    ///
    /// let doc = Document::from_path("tests/data/example1.json.gz")?;
    /// assert_eq!(doc.records().count(), 1);
    ///
    /// let doc = Document::from_path("tests/data/example1.json")?;
    /// assert_eq!(doc.records().count(), 1);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
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

    /// Writes the document into the given writer.
    ///
    /// If the pretty flag is set the document is serialized as
    /// pretty-printed JSON.
    ///
    /// ```rust
    /// use std::io::Cursor;
    ///
    /// use dvrf::{Document, Level, Record};
    ///
    /// let mut doc = Document::new();
    /// doc.write_record(
    ///     Record::new()
    ///         .with_message("foo")
    ///         .with_position("line", "1")
    ///         .with_level(Level::Error),
    /// );
    ///
    /// let mut wtr = Cursor::new(Vec::<u8>::new());
    /// doc.write_to(&mut wtr, false)?;
    ///
    /// let out = String::from_utf8(wtr.into_inner())?;
    /// assert_eq!(out, "[{\"message\":\"foo\",\"level\":\"error\",\"position\":{\"line\":\"1\"}}]");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
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
