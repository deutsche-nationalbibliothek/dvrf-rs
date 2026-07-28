use std::io::ErrorKind;
use std::process::ExitCode;

use clap::Parser;

use crate::cli::{Args, Command};
use crate::error::CliError;

mod cli;
mod commands;
mod error;
pub(crate) mod prelude;
mod utils;

fn main() -> ExitCode {
    let args = Args::parse();

    let result = match *args.cmd {
        Command::Concat(cmd) => cmd.execute(),
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(CliError::IO(e)) if e.kind() == ErrorKind::BrokenPipe => {
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}
