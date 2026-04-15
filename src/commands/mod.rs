pub mod check;
pub mod example;
pub mod explain;
pub mod import;
pub mod init;
pub mod run;
pub mod validate;
pub mod version;

use crate::cli::{Cli, Commands, ExplainArgs, HelpArgs, ImportArgs, RunArgs, RunOutputFormat};
use std::process::ExitCode;

pub fn run(cli: Cli) -> ExitCode {
    let Cli {
        verbose,
        verbose_mode,
        debug,
        default_run_input,
        command,
    } = cli;

    match command {
        None => run::run(
            RunArgs {
                input: default_run_input,
                output_format: RunOutputFormat::Markdown,
                output_file: None,
            },
            verbose,
            verbose_mode,
            debug,
        ),
        Some(Commands::Help(args)) => run_help(args),
        Some(Commands::Check(args)) => check::run(args),
        Some(Commands::Example(args)) => example::run(args),
        Some(Commands::Explain(args)) => run_explain(args),
        Some(Commands::Init(args)) => init::run(args),
        Some(Commands::Import(args)) => run_import(args),
        Some(Commands::Run(args)) => run::run(args, verbose, verbose_mode, debug),
        Some(Commands::Version) => version::run(),
        Some(Commands::Validate(args)) => validate::run(args),
    }
}

fn run_help(args: HelpArgs) -> ExitCode {
    if args.all {
        if let Some(topic) = args.topic {
            eprintln!("The help command does not accept both a topic and --all: {topic}");
            return ExitCode::from(1);
        }

        return match crate::cli::print_all_help() {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err}");
                ExitCode::from(1)
            }
        };
    }

    if let Some(topic) = args.topic {
        return match crate::cli::print_help_for_topic(&topic) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err}");
                ExitCode::from(1)
            }
        };
    }

    match crate::cli::print_top_level_help() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Failed to print help: {err}");
            ExitCode::from(1)
        }
    }
}

fn run_explain(args: ExplainArgs) -> ExitCode {
    explain::run(args)
}

fn run_import(args: ImportArgs) -> ExitCode {
    import::run(args)
}
