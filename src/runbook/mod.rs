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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunbookFormat {
    Json,
    Yaml,
}

enum InputSource {
    File(PathBuf),
    Stdin(InputFormat),
}

const STDIN_INPUT_PATH: &str = "-";
const DEFAULT_RUNBOOK_CANDIDATES: [&str; 3] =
    ["sw-runbook.json", "sw-runbook.yaml", "sw-runbook.yml"];

pub fn load(
    input_file: Option<PathBuf>,
    input_format: Option<InputFormat>,
) -> Result<LoadedRunbook, String> {
    match resolve_input_source(input_file, input_format)? {
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

pub fn load_file_only(input_file: Option<PathBuf>) -> Result<LoadedRunbook, String> {
    let path = resolve_file_input_path(input_file)?;
    let document = read(&path)?;
    Ok(LoadedRunbook { path, document })
}

pub fn read(path: &Path) -> Result<Value, String> {
    let contents = fs::read_to_string(path)
        .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;

    match infer_format_from_path(path) {
        RunbookFormat::Yaml => serde_norway::from_str(&contents)
            .map_err(|err| format!("Invalid YAML in {}: {err}", path.display())),
        RunbookFormat::Json => serde_json::from_str(&contents)
            .map_err(|err| format!("Invalid JSON in {}: {err}", path.display())),
    }
}

pub fn infer_supported_format(path: &Path) -> Result<RunbookFormat, String> {
    match path.extension().and_then(|value| value.to_str()) {
        Some("json") => Ok(RunbookFormat::Json),
        Some("yaml" | "yml") => Ok(RunbookFormat::Yaml),
        _ => Err(format!(
            "Unsupported runbook format for {}. Use a .json, .yaml, or .yml file name.",
            path.display()
        )),
    }
}

pub fn serialize(document: &Value, format: RunbookFormat) -> Result<String, String> {
    let serialized = match format {
        RunbookFormat::Json => serde_json::to_string_pretty(document)
            .map_err(|err| format!("Failed to serialize runbook as JSON: {err}"))?,
        RunbookFormat::Yaml => serialize_yaml(document)?,
    };

    if serialized.ends_with('\n') {
        Ok(serialized)
    } else {
        Ok(format!("{serialized}\n"))
    }
}

fn serialize_yaml(document: &Value) -> Result<String, String> {
    let Some(root) = document.as_object() else {
        return serde_norway::to_string(document)
            .map_err(|err| format!("Failed to serialize runbook as YAML: {err}"));
    };

    let mut output = String::new();

    for (key, value) in root {
        if key == "entries" {
            write_yaml_entries_field(&mut output, value)?;
            continue;
        }

        output.push_str(&serialize_yaml_mapping_field(key, value)?);
        output.push('\n');
    }

    Ok(output)
}

fn write_yaml_entries_field(output: &mut String, value: &Value) -> Result<(), String> {
    let Some(entries) = value.as_array() else {
        output.push_str(&serialize_yaml_mapping_field("entries", value)?);
        output.push('\n');
        return Ok(());
    };

    if entries.is_empty() {
        output.push_str("entries: []\n");
        return Ok(());
    }

    output.push_str("entries:\n");

    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }
        write_yaml_sequence_item(output, entry)?;
    }

    Ok(())
}

fn serialize_yaml_mapping_field(key: &str, value: &Value) -> Result<String, String> {
    let mut field = serde_json::Map::new();
    field.insert(key.to_string(), value.clone());

    let serialized = serde_norway::to_string(&Value::Object(field))
        .map_err(|err| format!("Failed to serialize runbook as YAML: {err}"))?;

    Ok(serialized.trim_end_matches('\n').to_string())
}

fn write_yaml_sequence_item(output: &mut String, value: &Value) -> Result<(), String> {
    let serialized = serde_norway::to_string(value)
        .map_err(|err| format!("Failed to serialize runbook as YAML: {err}"))?;

    for (index, line) in serialized.trim_end_matches('\n').lines().enumerate() {
        if line.is_empty() {
            output.push('\n');
            continue;
        }

        if index == 0 {
            output.push_str("- ");
        } else {
            output.push_str("  ");
        }
        output.push_str(line);
        output.push('\n');
    }

    Ok(())
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
) -> Result<InputSource, String> {
    let Some(path) = input_file else {
        return Ok(InputSource::File(resolve_default_input_path()?));
    };

    if path == Path::new(STDIN_INPUT_PATH) {
        return Ok(InputSource::Stdin(
            input_format.unwrap_or(InputFormat::Json),
        ));
    }

    Ok(InputSource::File(path))
}

fn resolve_file_input_path(input_file: Option<PathBuf>) -> Result<PathBuf, String> {
    let Some(path) = input_file else {
        return resolve_default_input_path();
    };

    if path == Path::new(STDIN_INPUT_PATH) {
        return Err(
            "The format command does not accept --input-file=-. Provide a .json, .yaml, or .yml file path."
                .to_string(),
        );
    }

    Ok(path)
}

fn resolve_default_input_path() -> Result<PathBuf, String> {
    let existing_candidates = DEFAULT_RUNBOOK_CANDIDATES
        .iter()
        .map(PathBuf::from)
        .filter(|candidate| candidate.exists())
        .collect::<Vec<_>>();

    match existing_candidates.as_slice() {
        [path] => Ok(path.clone()),
        [] => Ok(PathBuf::from(DEFAULT_RUNBOOK_CANDIDATES[0])),
        _ => Err(format!(
            "Multiple default runbooks found: {}. Specify --input-file explicitly.",
            existing_candidates
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn infer_format_from_path(path: &Path) -> RunbookFormat {
    match path.extension().and_then(|value| value.to_str()) {
        Some("yaml" | "yml") => RunbookFormat::Yaml,
        _ => RunbookFormat::Json,
    }
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
