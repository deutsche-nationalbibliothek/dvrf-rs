use std::path::PathBuf;

use dvrf::Document;

use crate::prelude::*;
use crate::utils::WriterBuilder;

/// Concatenate records from multiple inputs
#[derive(Debug, clap::Parser)]
#[clap(visible_alias = "cat")]
pub(crate) struct Concat {
    /// Whether to write the output as pretty-printed JSON or not.
    #[arg(long)]
    pretty: bool,

    /// Whether to keep empty records or not.
    #[arg(long)]
    keep_empty: bool,

    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,

    /// Write output to \<filename\> instead of stdout.
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,
}

impl Concat {
    pub(crate) fn execute(self) -> CliResult {
        let mut doc = Document::new();
        let mut wtr = WriterBuilder::default()
            .try_from_path_or_stdout(self.output)?;

        for path in self.path {
            let document = Document::from_path(path)?;
            for record in document.records() {
                doc.write_record(record.clone());
            }
        }

        // Unless empty records are explicitly kept using the
        // `--keep-empty` option, they will be removed.
        if !self.keep_empty {
            doc.retain(|record| !record.is_empty());
        }

        doc.write_to(&mut wtr, self.pretty)?;
        wtr.finish()?;

        Ok(())
    }
}
