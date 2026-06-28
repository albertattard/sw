mod cli;
mod commands;
mod runbook;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let _ = rustls::crypto::ring::default_provider().install_default();
    commands::run(cli::Cli::parse())
}
