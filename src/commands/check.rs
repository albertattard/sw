use crate::cli::CheckArgs;
use crate::runbook;
use std::process::ExitCode;

pub fn run(args: CheckArgs) -> ExitCode {
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
        return ExitCode::from(1);
    }

    match runbook::check_prerequisites(&runbook, &execution_root) {
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
        Err(runbook::RenderError::FailedAt { error, .. }) => match *error {
            runbook::RenderError::Operational(message) => {
                eprintln!("{message}");
                ExitCode::from(1)
            }
            runbook::RenderError::CommandFailed(message) => {
                eprintln!("{message}");
                ExitCode::from(2)
            }
            runbook::RenderError::CleanupFailed { message, .. } => {
                eprintln!("{message}");
                ExitCode::from(2)
            }
            runbook::RenderError::Timeout { message, .. } => {
                eprintln!("{message}");
                ExitCode::from(2)
            }
            runbook::RenderError::FailedAt { .. } => unreachable!(),
        },
    }
}
