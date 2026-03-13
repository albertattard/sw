mod execute;
mod render;
mod validate;

use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

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

pub fn read(path: &Path) -> Result<Value, String> {
    let contents = fs::read_to_string(path)
        .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;

    serde_json::from_str(&contents)
        .map_err(|err| format!("Invalid JSON in {}: {err}", path.display()))
}

pub fn print_human_with_runbook(result: &ValidationResult, path: &Path, runbook: Option<&Value>) {
    if result.valid {
        println!("Runbook is valid: {}", path.display());
        return;
    }

    println!("Runbook is invalid: {}", path.display());
    for error in &result.errors {
        println!("- {}: {}", error.path, error.message);
    }

    let Some(runbook) = runbook else {
        return;
    };
    let Some(entries) = runbook.get("entries").and_then(Value::as_array) else {
        return;
    };

    let offending_entries = offending_entry_indices(&result.errors);
    if offending_entries.is_empty() {
        return;
    }

    println!();
    println!("Offending entries:");
    for index in offending_entries {
        let Some(entry) = entries.get(index) else {
            continue;
        };

        println!("- entries[{index}]:");
        for line in format_validation_entry(entry).lines() {
            println!("  {line}");
        }
    }
}

pub fn print_json(result: &ValidationResult) -> Result<(), String> {
    let output = serde_json::to_string_pretty(result)
        .map_err(|err| format!("Failed to serialize output: {err}"))?;
    println!("{output}");
    Ok(())
}

fn offending_entry_indices(errors: &[ValidationIssue]) -> Vec<usize> {
    let mut indices = Vec::new();
    for error in errors {
        let Some(index) = entry_index_from_path(&error.path) else {
            continue;
        };

        if !indices.contains(&index) {
            indices.push(index);
        }
    }
    indices
}

fn entry_index_from_path(path: &str) -> Option<usize> {
    let suffix = path.strip_prefix("entries[")?;
    let index_end = suffix.find(']')?;
    suffix[..index_end].parse().ok()
}

fn format_validation_entry(entry: &Value) -> String {
    serde_json::to_string_pretty(entry)
        .unwrap_or_else(|_| "<failed to serialize offending entry>".to_string())
}
