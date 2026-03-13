pub mod check;
pub mod run;
pub mod validate;

use crate::cli::{Cli, Commands, RunArgs, RunOutputFormat};
use std::process::ExitCode;

pub fn run(cli: Cli) -> ExitCode {
    match cli.command {
        None => run::run(RunArgs {
            input_file: None,
            output_format: RunOutputFormat::Markdown,
            output_file: None,
        }),
        Some(Commands::Help) => match crate::cli::print_top_level_help() {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("Failed to print help: {err}");
                ExitCode::from(1)
            }
        },
        Some(Commands::Check(args)) => check::run(args),
        Some(Commands::Run(args)) => run::run(args),
        Some(Commands::Validate(args)) => validate::run(args),
    }
}
