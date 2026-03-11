mod cli;
mod commands;
mod runbook;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    commands::run(cli::Cli::parse())
}
