pub mod check;
pub mod convert;
pub mod example;
pub mod explain;
pub mod format;
pub mod import;
pub mod init;
pub mod run;
pub mod validate;
pub mod version;

use crate::cli::{
    Cli, Commands, ConvertArgs, ExplainArgs, FormatArgs, HelpArgs, ImportArgs, RunArgs,
};
use std::process::ExitCode;

pub fn run(cli: Cli) -> ExitCode {
    let Cli {
        verbose,
        verbose_mode,
        debug,
        default_run_input,
        default_run_output,
        default_run_debug,
        command,
    } = cli;

    match command {
        None => run::run(
            RunArgs {
                input: default_run_input,
                output: default_run_output,
            },
            verbose,
            verbose_mode,
            debug,
            default_run_debug,
        ),
        Some(Commands::Run(args)) => {
            run::run(args, verbose, verbose_mode, debug, default_run_debug)
        }
        Some(_) if default_run_input.has_values() => {
            eprintln!(
                "Run input options such as --input-file, --input-format, and --working-directory must be used with implicit `sw` or after the target subcommand"
            );
            eprintln!(
                "Try `sw run --input-file <path>` or `sw validate --input-file <path>` instead."
            );
            ExitCode::from(1)
        }
        Some(_) if default_run_output.has_values() => {
            eprintln!(
                "Run output options such as --output-file and --output-format must be used with implicit `sw` or after `sw run`"
            );
            eprintln!("Try `sw run --output-file <path>` instead.");
            ExitCode::from(1)
        }
        Some(_) if default_run_debug.has_values() => {
            eprintln!(
                "Run debugging options such as --preserve-on-failure and --start-at must be used with implicit `sw` or with `sw run`"
            );
            eprintln!("Try `sw run --preserve-on-failure --start-at <entry-number>` instead.");
            ExitCode::from(1)
        }
        Some(Commands::Help(args)) => run_help(args),
        Some(Commands::Check(args)) => check::run(args),
        Some(Commands::Convert(args)) => run_convert(args),
        Some(Commands::Example(args)) => example::run(args),
        Some(Commands::Explain(args)) => run_explain(args),
        Some(Commands::Format(args)) => run_format(args),
        Some(Commands::Init(args)) => init::run(args),
        Some(Commands::Import(args)) => run_import(args),
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

fn run_convert(args: ConvertArgs) -> ExitCode {
    convert::run(args)
}

fn run_import(args: ImportArgs) -> ExitCode {
    import::run(args)
}

fn run_format(args: FormatArgs) -> ExitCode {
    format::run(args)
}
