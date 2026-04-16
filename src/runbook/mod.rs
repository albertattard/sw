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
    let mut output = String::new();
    write_yaml_document(&mut output, document, 0)?;

    Ok(output)
}

fn write_yaml_document(output: &mut String, value: &Value, indent: usize) -> Result<(), String> {
    if let Some(map) = value.as_object() {
        return write_yaml_mapping(output, map, indent);
    }

    if let Some(array) = value.as_array() {
        return write_yaml_array(output, array, indent, false);
    }

    write_yaml_scalar_lines(output, indent, None, &yaml_scalar_value(value)?)
}

fn write_yaml_mapping(
    output: &mut String,
    map: &serde_json::Map<String, Value>,
    indent: usize,
) -> Result<(), String> {
    for (key, value) in map {
        write_yaml_field(output, indent, key, value)?;
    }

    Ok(())
}

fn write_yaml_field(
    output: &mut String,
    indent: usize,
    key: &str,
    value: &Value,
) -> Result<(), String> {
    if let Some(array) = value.as_array() {
        if array.is_empty() {
            output.push_str(&" ".repeat(indent));
            output.push_str(key);
            output.push_str(": []\n");
            return Ok(());
        }

        output.push_str(&" ".repeat(indent));
        output.push_str(key);
        output.push_str(":\n");
        let separate_entries = indent == 0 && key == "entries";
        return write_yaml_array(output, array, indent + 2, separate_entries);
    }

    if let Some(map) = value.as_object() {
        if map.is_empty() {
            output.push_str(&" ".repeat(indent));
            output.push_str(key);
            output.push_str(": {}\n");
            return Ok(());
        }

        output.push_str(&" ".repeat(indent));
        output.push_str(key);
        output.push_str(":\n");
        return write_yaml_mapping(output, map, indent + 2);
    }

    write_yaml_scalar_lines(output, indent, Some(key), &yaml_scalar_value(value)?)
}

fn write_yaml_array(
    output: &mut String,
    array: &[Value],
    indent: usize,
    separate_items: bool,
) -> Result<(), String> {
    for (index, value) in array.iter().enumerate() {
        if separate_items && index > 0 {
            output.push('\n');
        }
        write_yaml_array_item(output, indent, value)?;
    }

    Ok(())
}

fn write_yaml_array_item(output: &mut String, indent: usize, value: &Value) -> Result<(), String> {
    if let Some(map) = value.as_object() {
        if map.is_empty() {
            output.push_str(&" ".repeat(indent));
            output.push_str("- {}\n");
            return Ok(());
        }

        let mut fields = map.iter();
        let Some((first_key, first_value)) = fields.next() else {
            output.push_str(&" ".repeat(indent));
            output.push_str("- {}\n");
            return Ok(());
        };

        if can_inline_array_mapping_field(first_value) {
            write_yaml_inline_array_mapping_field(output, indent, first_key, first_value)?;
            for (key, field_value) in fields {
                write_yaml_field(output, indent + 2, key, field_value)?;
            }
            return Ok(());
        }

        output.push_str(&" ".repeat(indent));
        output.push_str("-\n");
        return write_yaml_mapping(output, map, indent + 2);
    }

    if let Some(array) = value.as_array() {
        if array.is_empty() {
            output.push_str(&" ".repeat(indent));
            output.push_str("- []\n");
            return Ok(());
        }

        output.push_str(&" ".repeat(indent));
        output.push_str("-\n");
        return write_yaml_array(output, array, indent + 2, false);
    }

    write_yaml_scalar_lines(output, indent, None, &yaml_scalar_value(value)?)
}

fn write_yaml_inline_array_mapping_field(
    output: &mut String,
    indent: usize,
    key: &str,
    value: &Value,
) -> Result<(), String> {
    output.push_str(&" ".repeat(indent));
    output.push_str("- ");
    output.push_str(key);

    if matches!(value, Value::Array(array) if array.is_empty()) {
        output.push_str(": []\n");
        return Ok(());
    }

    if matches!(value, Value::Object(map) if map.is_empty()) {
        output.push_str(": {}\n");
        return Ok(());
    }

    output.push_str(": ");
    let rendered = yaml_scalar_value(value)?;
    let mut lines = rendered.lines();
    let Some(first_line) = lines.next() else {
        output.push('\n');
        return Ok(());
    };
    output.push_str(first_line);
    output.push('\n');

    for line in lines {
        if line.is_empty() {
            output.push('\n');
            continue;
        }
        output.push_str(&" ".repeat(indent + 2));
        output.push_str(line);
        output.push('\n');
    }

    Ok(())
}

fn write_yaml_scalar_lines(
    output: &mut String,
    indent: usize,
    key: Option<&str>,
    rendered: &str,
) -> Result<(), String> {
    let mut lines = rendered.lines();
    let Some(first_line) = lines.next() else {
        if let Some(key) = key {
            output.push_str(&" ".repeat(indent));
            output.push_str(key);
            output.push_str(": ''\n");
        } else {
            output.push_str(&" ".repeat(indent));
            output.push_str("- ''\n");
        }
        return Ok(());
    };

    output.push_str(&" ".repeat(indent));
    match key {
        Some(key) => {
            output.push_str(key);
            output.push_str(": ");
        }
        None => output.push_str("- "),
    }
    output.push_str(first_line);
    output.push('\n');

    for line in lines {
        if line.is_empty() {
            output.push('\n');
            continue;
        }

        output.push_str(&" ".repeat(indent));
        output.push_str(line);
        output.push('\n');
    }

    Ok(())
}

fn can_inline_array_mapping_field(value: &Value) -> bool {
    match value {
        Value::Array(array) => array.is_empty(),
        Value::Object(map) => map.is_empty(),
        Value::String(value) => !value.contains('\n'),
        _ => true,
    }
}

fn yaml_scalar_value(value: &Value) -> Result<String, String> {
    let rendered = serde_norway::to_string(value)
        .map_err(|err| format!("Failed to serialize runbook as YAML: {err}"))?;

    Ok(rendered.trim_end_matches('\n').to_string())
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
