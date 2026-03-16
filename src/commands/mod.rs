pub mod check;
pub mod example;
pub mod run;
pub mod validate;

use crate::cli::{Cli, Commands, RunArgs, RunOutputFormat};
use std::process::ExitCode;

pub fn run(cli: Cli) -> ExitCode {
    let Cli { verbose, command } = cli;

    match command {
        None => run::run(
            RunArgs {
                input_file: None,
                output_format: RunOutputFormat::Markdown,
                output_file: None,
            },
            verbose,
        ),
        Some(Commands::Help) => match crate::cli::print_top_level_help() {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("Failed to print help: {err}");
                ExitCode::from(1)
            }
        },
        Some(Commands::Check(args)) => check::run(args),
        Some(Commands::Example(args)) => example::run(args),
        Some(Commands::Run(args)) => run::run(args, verbose),
        Some(Commands::Validate(args)) => validate::run(args),
    }
}
