use crate::cli::{OutputFormat, ValidateArgs};
use crate::runbook;
use serde_json::json;
use std::process::ExitCode;

pub fn run(args: ValidateArgs) -> ExitCode {
    let path = runbook::resolve_input_path(args.input_file);

    let runbook = match runbook::read(&path) {
        Ok(runbook) => runbook,
        Err(message) => {
            if args.output_format == OutputFormat::Json {
                println!(
                    "{}",
                    json!({
                        "schema_version": "1",
                        "valid": false,
                        "errors": [{"path": "$", "message": message}],
                        "warnings": [],
                    })
                );
            } else {
                eprintln!("{message}");
            }
            return ExitCode::from(1);
        }
    };

    let result = runbook::validate(&runbook, &path);

    let print_result = match args.output_format {
        OutputFormat::Human => {
            runbook::print_human_with_runbook(&result, &path, Some(&runbook));
            Ok(())
        }
        OutputFormat::Json => runbook::print_json(&result),
    };

    if let Err(message) = print_result {
        eprintln!("{message}");
        return ExitCode::from(1);
    }

    if result.valid {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(2)
    }
}
