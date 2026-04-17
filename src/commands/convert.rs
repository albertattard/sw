use crate::cli::{ConvertArgs, ConvertOutputFormat};
use crate::runbook::{self, RunbookFormat};
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;

const STDIN_INPUT_PATH: &str = "-";

pub fn run(args: ConvertArgs) -> ExitCode {
    let input_path = match resolve_input_path(args.input_file) {
        Ok(path) => path,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    let input_format = match runbook::infer_supported_format(&input_path) {
        Ok(format) => format,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    let document = match runbook::read(&input_path) {
        Ok(document) => document,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    let (output_path, output_format) = match resolve_output_target(
        &input_path,
        input_format,
        args.output_file,
        args.output_format,
    ) {
        Ok(target) => target,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    if output_path.exists() && !args.force {
        eprintln!(
            "Refusing to overwrite existing output file: {}",
            output_path.display()
        );
        return ExitCode::from(1);
    }

    let validation = runbook::validate(&document, &input_path);
    if !validation.valid {
        runbook::print_human_with_runbook(&validation, &input_path, Some(&document));
        return ExitCode::from(2);
    }

    let rendered = match runbook::serialize(&document, output_format) {
        Ok(rendered) => rendered,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    if let Err(err) = fs::write(&output_path, rendered) {
        eprintln!("Failed to write {}: {err}", output_path.display());
        return ExitCode::from(1);
    }

    println!(
        "Converted {} to {}",
        input_path.display(),
        output_path.display()
    );
    ExitCode::SUCCESS
}

fn resolve_input_path(input_file: Option<PathBuf>) -> Result<PathBuf, String> {
    let Some(path) = input_file else {
        return runbook::resolve_single_existing_default_input_path();
    };

    if path == Path::new(STDIN_INPUT_PATH) {
        return Err(
            "The convert command does not accept --input-file=-. Provide a .json, .yaml, or .yml file path."
                .to_string(),
        );
    }

    Ok(path)
}

fn resolve_output_target(
    input_path: &Path,
    input_format: RunbookFormat,
    output_file: Option<PathBuf>,
    output_format: Option<ConvertOutputFormat>,
) -> Result<(PathBuf, RunbookFormat), String> {
    if let Some(path) = output_file.as_ref()
        && normalize_path_for_comparison(input_path)? == normalize_path_for_comparison(path)?
    {
        return Err(format!(
            "The convert command does not support in-place conversion: {}",
            path.display()
        ));
    }

    let inferred_output_format = output_file
        .as_ref()
        .map(|path| infer_output_format_from_path(path))
        .transpose()?;

    if let (Some(explicit), Some(inferred), Some(path)) =
        (output_format, inferred_output_format, output_file.as_ref())
        && to_runbook_format(explicit) != inferred
    {
        return Err(format!(
            "Output file extension does not match --output-format: {}",
            path.display()
        ));
    }

    let resolved_output_format = output_format
        .map(to_runbook_format)
        .or(inferred_output_format)
        .unwrap_or_else(|| opposite_format(input_format));

    if resolved_output_format == input_format {
        return Err(format!(
            "The convert command requires the opposite format. Source is already {}.",
            format_name(input_format)
        ));
    }

    let resolved_output_path =
        output_file.unwrap_or_else(|| derive_output_path(input_path, resolved_output_format));

    if normalize_path_for_comparison(input_path)?
        == normalize_path_for_comparison(&resolved_output_path)?
    {
        return Err(format!(
            "The convert command does not support in-place conversion: {}",
            resolved_output_path.display()
        ));
    }

    Ok((resolved_output_path, resolved_output_format))
}

fn infer_output_format_from_path(path: &Path) -> Result<RunbookFormat, String> {
    runbook::infer_supported_format(path)
}

fn to_runbook_format(output_format: ConvertOutputFormat) -> RunbookFormat {
    match output_format {
        ConvertOutputFormat::Json => RunbookFormat::Json,
        ConvertOutputFormat::Yaml => RunbookFormat::Yaml,
    }
}

fn opposite_format(format: RunbookFormat) -> RunbookFormat {
    match format {
        RunbookFormat::Json => RunbookFormat::Yaml,
        RunbookFormat::Yaml => RunbookFormat::Json,
    }
}

fn derive_output_path(input_path: &Path, output_format: RunbookFormat) -> PathBuf {
    input_path.with_extension(match output_format {
        RunbookFormat::Json => "json",
        RunbookFormat::Yaml => "yaml",
    })
}

fn format_name(format: RunbookFormat) -> &'static str {
    match format {
        RunbookFormat::Json => "JSON",
        RunbookFormat::Yaml => "YAML",
    }
}

fn normalize_path_for_comparison(path: &Path) -> Result<PathBuf, String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .map_err(|err| format!("Failed to resolve current directory: {err}"))?
            .join(path)
    };

    Ok(absolute
        .components()
        .fold(PathBuf::new(), |mut normalized, component| {
            match component {
                Component::CurDir => {}
                Component::ParentDir => {
                    normalized.pop();
                }
                Component::RootDir | Component::Prefix(_) | Component::Normal(_) => {
                    normalized.push(component.as_os_str());
                }
            }
            normalized
        }))
}
