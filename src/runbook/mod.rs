mod validate;

use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

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

pub fn read(path: &Path) -> Result<Value, String> {
    let contents = fs::read_to_string(path)
        .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;

    serde_json::from_str(&contents)
        .map_err(|err| format!("Invalid JSON in {}: {err}", path.display()))
}

pub fn print_human(result: &ValidationResult, path: &Path) {
    if result.valid {
        println!("Runbook is valid: {}", path.display());
        return;
    }

    println!("Runbook is invalid: {}", path.display());
    for error in &result.errors {
        println!("- {}: {}", error.path, error.message);
    }
}

pub fn print_json(result: &ValidationResult) -> Result<(), String> {
    let output = serde_json::to_string_pretty(result)
        .map_err(|err| format!("Failed to serialize output: {err}"))?;
    println!("{output}");
    Ok(())
}

pub fn render_markdown(runbook: &Value) -> Result<String, String> {
    let entries = runbook
        .get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| "Runbook is missing an entries array".to_string())?;

    let mut sections = Vec::new();

    for entry in entries {
        let entry_type = entry
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| "Runbook entry is missing a type".to_string())?;

        sections.push(match entry_type {
            "Heading" => render_heading(entry)?,
            "Markdown" => render_markdown_entry(entry)?,
            "Command" => render_command(entry)?,
            other => return Err(format!("Unsupported entry type `{other}`")),
        });
    }

    let mut output = sections.join("\n\n");
    if !output.ends_with('\n') {
        output.push('\n');
    }
    Ok(output)
}

fn render_heading(entry: &Value) -> Result<String, String> {
    let level = entry
        .get("level")
        .and_then(Value::as_str)
        .ok_or_else(|| "Heading entry is missing level".to_string())?;
    let title = entry
        .get("title")
        .and_then(Value::as_str)
        .ok_or_else(|| "Heading entry is missing title".to_string())?;

    let marker = match level {
        "H1" => "#",
        "H2" => "##",
        "H3" => "###",
        "H4" => "####",
        "H5" => "#####",
        "H6" => "######",
        other => return Err(format!("Unsupported heading level `{other}`")),
    };

    Ok(format!("{marker} {title}"))
}

fn render_markdown_entry(entry: &Value) -> Result<String, String> {
    let contents = entry
        .get("contents")
        .and_then(Value::as_array)
        .ok_or_else(|| "Markdown entry is missing contents".to_string())?;

    let mut lines = Vec::new();
    for item in contents {
        lines.push(
            item.as_str()
                .ok_or_else(|| "Markdown contents must contain only strings".to_string())?
                .to_string(),
        );
    }

    Ok(lines.join("\n"))
}

fn render_command(entry: &Value) -> Result<String, String> {
    let commands = entry
        .get("commands")
        .and_then(Value::as_array)
        .ok_or_else(|| "Command entry is missing commands".to_string())?;

    let mut command_lines = Vec::new();
    for item in commands {
        command_lines.push(
            item.as_str()
                .ok_or_else(|| "Command list must contain only strings".to_string())?
                .to_string(),
        );
    }

    let mut section = format!("```shell\n{}\n```", command_lines.join("\n"));

    if let Some(output) = entry.get("output") {
        let caption = output
            .get("caption")
            .ok_or_else(|| "Command output section is missing caption".to_string())?;
        let caption_text = match caption {
            Value::String(text) => text.to_string(),
            Value::Array(items) => {
                let mut lines = Vec::new();
                for item in items {
                    lines.push(
                        item.as_str()
                            .ok_or_else(|| {
                                "Command output caption must contain only strings".to_string()
                            })?
                            .to_string(),
                    );
                }
                lines.join("\n")
            }
            _ => {
                return Err(
                    "Command output caption must be a string or array of strings".to_string(),
                );
            }
        };

        section.push_str("\n\n");
        section.push_str(&caption_text);
    }

    Ok(section)
}
