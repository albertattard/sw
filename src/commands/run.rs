use crate::cli::{RunArgs, RunOutputFormat, VerboseMode};
use crate::runbook;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn run(args: RunArgs, verbose: bool, verbose_mode: VerboseMode, debug: bool) -> ExitCode {
    let output_path = args
        .output_file
        .unwrap_or_else(|| PathBuf::from("README.md"));

    let loaded = match runbook::load(args.input.input_file, args.input.input_format) {
        Ok(loaded) => loaded,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };
    let input_path = loaded.path;
    let runbook = loaded.document;

    let validation_result = runbook::validate(&runbook, &input_path);
    if !validation_result.valid {
        runbook::print_human_with_runbook(&validation_result, &input_path, Some(&runbook));
        return ExitCode::from(2);
    }

    let markdown = match args.output_format {
        RunOutputFormat::Markdown => {
            match runbook::render_markdown(&runbook, &input_path, verbose, verbose_mode, debug) {
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
