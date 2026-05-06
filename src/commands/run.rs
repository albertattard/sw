use crate::cli::{RunArgs, RunOutputFormat, VerboseMode};
use crate::runbook;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn run(args: RunArgs, verbose: bool, verbose_mode: VerboseMode, debug: bool) -> ExitCode {
    let output_path = args
        .output
        .output_file
        .unwrap_or_else(|| PathBuf::from("README.md"));
    let input_args = args.input;

    let loaded = match runbook::load(input_args.input_file, input_args.input_format) {
        Ok(loaded) => loaded,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };
    let input_path = loaded.path;
    let runbook = loaded.document;
    let execution_root =
        match runbook::resolve_execution_root(&input_path, input_args.working_directory) {
            Ok(root) => root,
            Err(message) => {
                eprintln!("{message}");
                return ExitCode::from(1);
            }
        };

    let validation_result = runbook::validate_with_execution_root(&runbook, &execution_root);
    if !validation_result.valid {
        runbook::print_human_with_runbook(&validation_result, &input_path, Some(&runbook));
        return ExitCode::from(2);
    }

    let markdown = match args
        .output
        .output_format
        .unwrap_or(RunOutputFormat::Markdown)
    {
        RunOutputFormat::Markdown => {
            match runbook::render_markdown(&runbook, &execution_root, verbose, verbose_mode, debug)
            {
                Ok(markdown) => markdown,
                Err(runbook::RenderError::Operational(message)) => {
                    eprintln!("{message}");
                    return ExitCode::from(1);
                }
                Err(runbook::RenderError::CommandFailed(message)) => {
                    eprintln!("{message}");
                    return ExitCode::from(2);
                }
                Err(runbook::RenderError::CleanupFailed { message, markdown }) => {
                    if let Err(write_error) = std::fs::write(&output_path, &markdown) {
                        eprintln!("Failed to write {}: {write_error}", output_path.display());
                        return ExitCode::from(1);
                    }
                    eprintln!("{message}");
                    return ExitCode::from(2);
                }
                Err(runbook::RenderError::Timeout {
                    message,
                    partial_markdown,
                }) => {
                    if let Err(write_error) = std::fs::write(&output_path, &partial_markdown) {
                        eprintln!("Failed to write {}: {write_error}", output_path.display());
                        return ExitCode::from(1);
                    }
                    eprintln!("{message}");
                    return ExitCode::from(2);
                }
            }
        }
    };

    if let Err(message) = std::fs::write(&output_path, markdown) {
        eprintln!("Failed to write {}: {message}", output_path.display());
        return ExitCode::from(1);
    }

    println!("Rendered runbook to {}", output_path.display());
    ExitCode::SUCCESS
}
