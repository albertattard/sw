pub mod validate;

use crate::cli::{Cli, Commands};
use std::process::ExitCode;

pub fn run(cli: Cli) -> ExitCode {
    match cli.command {
        None | Some(Commands::Help) => match crate::cli::print_top_level_help() {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("Failed to print help: {err}");
                ExitCode::from(1)
            }
        },
        Some(Commands::Validate(args)) => validate::run(args),
    }
}
