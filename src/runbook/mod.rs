mod execute;
mod render;
mod validate;

use crate::cli::InputFormat;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub(crate) use render::{check_prerequisites, render_markdown};
pub use validate::validate;

#[derive(Debug, Serialize)]
pub struct ValidationIssue {
    pub path: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationResult {
    pub schema_version: &'static str,
    pub valid: bool,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
}

pub enum RenderError {
    Operational(String),
    CommandFailed(String),
    CleanupFailed {
        message: String,
        markdown: String,
    },
    Timeout {
        message: String,
        partial_markdown: String,
    },
}

pub struct LoadedRunbook {
    pub path: PathBuf,
    pub document: Value,
}

enum InputSource {
    File(PathBuf),
    Stdin(InputFormat),
}

const STDIN_INPUT_PATH: &str = "-";

pub fn load(
    input_file: Option<PathBuf>,
    input_format: Option<InputFormat>,
) -> Result<LoadedRunbook, String> {
    match resolve_input_source(input_file, input_format) {
        InputSource::File(path) => {
            let document = read(&path)?;
            Ok(LoadedRunbook { path, document })
        }
        InputSource::Stdin(format) => {
            let document = read_stdin(format)?;
            Ok(LoadedRunbook {
                path: PathBuf::from(STDIN_INPUT_PATH),
                document,
            })
        }
    }
}

pub fn read(path: &Path) -> Result<Value, String> {
    let contents = fs::read_to_string(path)
        .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;

    match path.extension().and_then(|value| value.to_str()) {
        Some("yaml" | "yml") => serde_norway::from_str(&contents)
            .map_err(|err| format!("Invalid YAML in {}: {err}", path.display())),
        _ => serde_json::from_str(&contents)
            .map_err(|err| format!("Invalid JSON in {}: {err}", path.display())),
    }
}

fn read_stdin(format: InputFormat) -> Result<Value, String> {
    let mut contents = String::new();
    io::stdin()
        .read_to_string(&mut contents)
        .map_err(|err| format!("Failed to read stdin: {err}"))?;

    match format {
        InputFormat::Json => {
            serde_json::from_str(&contents).map_err(|err| format!("Invalid JSON in stdin: {err}"))
        }
        InputFormat::Yaml => {
            serde_norway::from_str(&contents).map_err(|err| format!("Invalid YAML in stdin: {err}"))
        }
    }
}

fn resolve_input_source(
    input_file: Option<PathBuf>,
    input_format: Option<InputFormat>,
) -> InputSource {
    let Some(path) = input_file else {
        return InputSource::File(resolve_default_input_path());
    };

    if path == Path::new(STDIN_INPUT_PATH) {
        return InputSource::Stdin(input_format.unwrap_or(InputFormat::Json));
    }

    InputSource::File(path)
}

fn resolve_default_input_path() -> PathBuf {
    for candidate in [
        PathBuf::from("sw-runbook.json"),
        PathBuf::from("sw-runbook.yaml"),
        PathBuf::from("sw-runbook.yml"),
    ] {
        if candidate.exists() {
            return candidate;
        }
    }

    PathBuf::from("sw-runbook.json")
}

pub fn print_human_with_runbook(result: &ValidationResult, path: &Path, runbook: Option<&Value>) {
    if result.valid {
        println!("Runbook is valid: {}", path.display());
        for warning in &result.warnings {
            println!("- warning {}: {}", warning.path, warning.message);
        }
        return;
    }

    println!("Runbook is invalid: {}", path.display());
    for error in &result.errors {
        println!("- {}: {}", error.path, error.message);
        let Some(runbook) = runbook else {
            continue;
        };
        let Some(context) = validation_context_for_error(runbook, &error.path) else {
            continue;
        };

        println!("  Offending block:");
        for line in format_validation_entry(context).lines() {
            println!("    {line}");
        }
    }
    for warning in &result.warnings {
        println!("- warning {}: {}", warning.path, warning.message);
    }
}

pub fn print_json(result: &ValidationResult) -> Result<(), String> {
    let output = serde_json::to_string_pretty(result)
        .map_err(|err| format!("Failed to serialize output: {err}"))?;
    println!("{output}");
    Ok(())
}

fn format_validation_entry(entry: &Value) -> String {
    serde_json::to_string_pretty(entry)
        .unwrap_or_else(|_| "<failed to serialize offending entry>".to_string())
}

fn validation_context_for_error<'a>(runbook: &'a Value, path: &str) -> Option<&'a Value> {
    let mut tokens = parse_validation_path(path)?;
    while !tokens.is_empty() {
        tokens.pop();
        let value = resolve_validation_path(runbook, &tokens)?;
        if value.is_object() {
            return Some(value);
        }
    }
    None
}

#[derive(Clone)]
enum ValidationPathToken {
    Key(String),
    Index(usize),
}

fn parse_validation_path(path: &str) -> Option<Vec<ValidationPathToken>> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = path.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        if chars[index] == '.' {
            index += 1;
            continue;
        }

        if chars[index] == '[' {
            index += 1;
            let start = index;
            while index < chars.len() && chars[index] != ']' {
                index += 1;
            }
            let value: usize = chars[start..index]
                .iter()
                .collect::<String>()
                .parse()
                .ok()?;
            tokens.push(ValidationPathToken::Index(value));
            index += 1;
            continue;
        }

        let start = index;
        while index < chars.len() && chars[index] != '.' && chars[index] != '[' {
            index += 1;
        }
        let key = chars[start..index].iter().collect::<String>();
        tokens.push(ValidationPathToken::Key(key));
    }

    Some(tokens)
}

fn resolve_validation_path<'a>(
    mut value: &'a Value,
    tokens: &[ValidationPathToken],
) -> Option<&'a Value> {
    for token in tokens {
        value = match token {
            ValidationPathToken::Key(key) => value.get(key)?,
            ValidationPathToken::Index(index) => value.get(*index)?,
        };
    }
    Some(value)
}
