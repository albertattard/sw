use crate::cli::CheckArgs;
use crate::runbook;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn run(args: CheckArgs) -> ExitCode {
    let input_path = args
        .input_file
        .unwrap_or_else(|| PathBuf::from("sw-runbook.json"));

    let runbook = match runbook::read(&input_path) {
        Ok(runbook) => runbook,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    let validation_result = runbook::validate(&runbook);
    if !validation_result.valid {
        runbook::print_human_with_runbook(&validation_result, &input_path, Some(&runbook));
        return ExitCode::from(1);
    }

    match runbook::check_prerequisites(&runbook) {
        Ok(()) => {
            println!("All prerequisite checks passed");
            ExitCode::SUCCESS
        }
        Err(runbook::RenderError::Operational(message)) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
        Err(runbook::RenderError::CommandFailed(message)) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
        Err(runbook::RenderError::CleanupFailed { message, .. }) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
        Err(runbook::RenderError::Timeout { message, .. }) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}
