use crate::cli::{RunArgs, RunOutputFormat};
use crate::runbook;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn run(args: RunArgs) -> ExitCode {
    let input_path = args
        .input_file
        .unwrap_or_else(|| PathBuf::from("sw-runbook.json"));
    let output_path = args
        .output_file
        .unwrap_or_else(|| PathBuf::from("readme.md"));

    let runbook = match runbook::read(&input_path) {
        Ok(runbook) => runbook,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    let validation_result = runbook::validate(&runbook);
    if !validation_result.valid {
        runbook::print_human(&validation_result, &input_path);
        return ExitCode::from(2);
    }

    let markdown = match args.output_format {
        RunOutputFormat::Markdown => match runbook::render_markdown(&runbook, &input_path) {
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
        },
    };

    if let Err(message) = std::fs::write(&output_path, markdown) {
        eprintln!("Failed to write {}: {message}", output_path.display());
        return ExitCode::from(1);
    }

    println!("Rendered runbook to {}", output_path.display());
    ExitCode::SUCCESS
}
