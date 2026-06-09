use crate::cli::{InputFormat, RunArgs, RunDebugArgs, RunOutputFormat, VerboseMode};
use crate::runbook;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn run(
    args: RunArgs,
    verbose: bool,
    verbose_mode: VerboseMode,
    debug: bool,
    run_debug: RunDebugArgs,
) -> ExitCode {
    let output_path = args
        .output
        .output_file
        .clone()
        .unwrap_or_else(|| PathBuf::from("README.md"));
    let resume_base_command = resume_base_command(&args, verbose, verbose_mode, debug, &run_debug);
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
            let options = runbook::RenderOptions {
                verbose,
                verbose_mode,
                debug,
                preserve_on_failure: run_debug.preserve_on_failure,
                start_at: run_debug.start_at,
            };

            match runbook::render_markdown(&runbook, &execution_root, options) {
                Ok(markdown) => markdown,
                Err(error) => {
                    return handle_render_error(error, &output_path, &resume_base_command);
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

fn handle_render_error(
    error: runbook::RenderError,
    output_path: &Path,
    resume_base_command: &[String],
) -> ExitCode {
    match error {
        runbook::RenderError::Operational(message) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
        runbook::RenderError::CommandFailed(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
        runbook::RenderError::CleanupFailed { message, markdown } => {
            if let Err(write_error) = std::fs::write(output_path, &markdown) {
                eprintln!("Failed to write {}: {write_error}", output_path.display());
                return ExitCode::from(1);
            }
            eprintln!("{message}");
            ExitCode::from(2)
        }
        runbook::RenderError::Timeout {
            message,
            partial_markdown,
        } => {
            if let Err(write_error) = std::fs::write(output_path, &partial_markdown) {
                eprintln!("Failed to write {}: {write_error}", output_path.display());
                return ExitCode::from(1);
            }
            eprintln!("{message}");
            ExitCode::from(2)
        }
        runbook::RenderError::FailedAt {
            entry_number,
            error,
        } => handle_failed_at(*error, entry_number, output_path, resume_base_command),
    }
}

fn handle_failed_at(
    error: runbook::RenderError,
    entry_number: usize,
    output_path: &Path,
    resume_base_command: &[String],
) -> ExitCode {
    let exit_code = handle_render_error(error, output_path, resume_base_command);
    if exit_code == ExitCode::from(2) {
        eprintln!(
            "Run: {} --start-at {entry_number} to resume from this entry",
            resume_base_command.join(" ")
        );
    }
    exit_code
}

fn resume_base_command(
    args: &RunArgs,
    verbose: bool,
    verbose_mode: VerboseMode,
    debug: bool,
    run_debug: &RunDebugArgs,
) -> Vec<String> {
    let mut command = vec!["sw".to_string(), "run".to_string()];

    if verbose {
        command.push("--verbose".to_string());
    }
    if verbose_mode != VerboseMode::Auto {
        command.push(format!(
            "--verbose-mode={}",
            verbose_mode_value(verbose_mode)
        ));
    }
    if debug {
        command.push("--debug".to_string());
    }
    if let Some(input_file) = &args.input.input_file {
        command.push("--input-file".to_string());
        command.push(shell_arg(&input_file.display().to_string()));
    }
    if let Some(input_format) = args.input.input_format {
        command.push(format!(
            "--input-format={}",
            input_format_value(input_format)
        ));
    }
    if let Some(working_directory) = &args.input.working_directory {
        command.push("--working-directory".to_string());
        command.push(shell_arg(&working_directory.display().to_string()));
    }
    if let Some(output_format) = args.output.output_format {
        command.push(format!(
            "--output-format={}",
            run_output_format_value(output_format)
        ));
    }
    if let Some(output_file) = &args.output.output_file {
        command.push("--output-file".to_string());
        command.push(shell_arg(&output_file.display().to_string()));
    }
    if run_debug.preserve_on_failure {
        command.push("--preserve-on-failure".to_string());
    }

    command
}

fn verbose_mode_value(mode: VerboseMode) -> &'static str {
    match mode {
        VerboseMode::Auto => "auto",
        VerboseMode::Live => "live",
        VerboseMode::Plain => "plain",
    }
}

fn input_format_value(format: InputFormat) -> &'static str {
    match format {
        InputFormat::Json => "json",
        InputFormat::Yaml => "yaml",
    }
}

fn run_output_format_value(format: RunOutputFormat) -> &'static str {
    match format {
        RunOutputFormat::Markdown => "markdown",
    }
}

fn shell_arg(value: &str) -> String {
    if value.chars().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, '/' | '.' | '-' | '_' | '=')
    }) {
        return value.to_string();
    }

    format!("'{}'", value.replace('\'', "'\\''"))
}
