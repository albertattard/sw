use crate::cli::FormatArgs;
use crate::runbook;
use std::fs;
use std::process::ExitCode;

pub fn run(args: FormatArgs) -> ExitCode {
    let loaded = match runbook::load_file_only(args.input_file) {
        Ok(loaded) => loaded,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };
    let path = loaded.path;
    let runbook = loaded.document;

    let format = match runbook::infer_supported_format(&path) {
        Ok(format) => format,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    let validation = runbook::validate(&runbook, &path);
    if !validation.valid {
        runbook::print_human_with_runbook(&validation, &path, Some(&runbook));
        return ExitCode::from(2);
    }

    let normalized_runbook;
    let render_input = if format == runbook::RunbookFormat::Yaml {
        normalized_runbook = runbook::normalize_document_for_yaml_authoring(&runbook);
        &normalized_runbook
    } else {
        &runbook
    };

    let rendered = match runbook::serialize(render_input, format) {
        Ok(rendered) => rendered,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    if let Err(err) = fs::write(&path, rendered) {
        eprintln!("Failed to write {}: {err}", path.display());
        return ExitCode::from(1);
    }

    println!("Formatted runbook: {}", path.display());
    ExitCode::SUCCESS
}
