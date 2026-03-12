mod validate;

use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

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

pub fn render_markdown(runbook: &Value) -> Result<String, RenderError> {
    let entries = runbook
        .get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            RenderError::Operational("Runbook is missing an entries array".to_string())
        })?;

    let mut sections = Vec::new();

    for entry in entries {
        let entry_type = entry.get("type").and_then(Value::as_str).ok_or_else(|| {
            RenderError::Operational("Runbook entry is missing a type".to_string())
        })?;

        sections.push(match entry_type {
            "Heading" => render_heading(entry)?,
            "Markdown" => render_markdown_entry(entry)?,
            "Command" => render_command(entry)?,
            other => {
                return Err(RenderError::Operational(format!(
                    "Unsupported entry type `{other}`"
                )));
            }
        });
    }

    let mut output = sections.join("\n\n");
    if !output.ends_with('\n') {
        output.push('\n');
    }
    Ok(output)
}

fn render_heading(entry: &Value) -> Result<String, RenderError> {
    let level = entry
        .get("level")
        .and_then(Value::as_str)
        .ok_or_else(|| RenderError::Operational("Heading entry is missing level".to_string()))?;
    let title = entry
        .get("title")
        .and_then(Value::as_str)
        .ok_or_else(|| RenderError::Operational("Heading entry is missing title".to_string()))?;

    let marker = match level {
        "H1" => "#",
        "H2" => "##",
        "H3" => "###",
        "H4" => "####",
        "H5" => "#####",
        "H6" => "######",
        other => {
            return Err(RenderError::Operational(format!(
                "Unsupported heading level `{other}`"
            )));
        }
    };

    Ok(format!("{marker} {title}"))
}

fn render_markdown_entry(entry: &Value) -> Result<String, RenderError> {
    let contents = entry
        .get("contents")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            RenderError::Operational("Markdown entry is missing contents".to_string())
        })?;

    let mut lines = Vec::new();
    for item in contents {
        lines.push(
            item.as_str()
                .ok_or_else(|| {
                    RenderError::Operational(
                        "Markdown contents must contain only strings".to_string(),
                    )
                })?
                .to_string(),
        );
    }

    Ok(lines.join("\n"))
}

fn render_command(entry: &Value) -> Result<String, RenderError> {
    let commands = entry
        .get("commands")
        .and_then(Value::as_array)
        .ok_or_else(|| RenderError::Operational("Command entry is missing commands".to_string()))?;

    let mut command_lines = Vec::new();
    for item in commands {
        command_lines.push(
            item.as_str()
                .ok_or_else(|| {
                    RenderError::Operational("Command list must contain only strings".to_string())
                })?
                .to_string(),
        );
    }

    let command_text = command_lines.join("\n");
    let mut section = format!("```shell\n{command_text}\n```");
    let execution = execute_command(&command_text)?;
    ensure_expected_exit_code(entry, &execution)?;
    ensure_assert_checks(entry, &execution)?;

    if let Some(output) = entry.get("output") {
        if let Some(caption) = output.get("caption") {
            let caption_text = match caption {
                Value::String(text) => text.to_string(),
                Value::Array(items) => {
                    let mut lines = Vec::new();
                    for item in items {
                        lines.push(
                            item.as_str()
                                .ok_or_else(|| {
                                    RenderError::Operational(
                                        "Command output caption must contain only strings"
                                            .to_string(),
                                    )
                                })?
                                .to_string(),
                        );
                    }
                    lines.join("\n")
                }
                _ => {
                    return Err(RenderError::Operational(
                        "Command output caption must be a string or array of strings".to_string(),
                    ));
                }
            };

            section.push_str("\n\n");
            section.push_str(&caption_text);
        };

        section.push_str("\n\n");
        section.push_str("```text\n");
        section.push_str(&execution.stdout);
        if !execution.stdout.ends_with('\n') {
            section.push('\n');
        }
        section.push_str("```");
    }

    Ok(section)
}

struct CommandExecution {
    exit_code: i32,
    stdout: String,
}

fn execute_command(command: &str) -> Result<CommandExecution, RenderError> {
    let output = Command::new("sh")
        .arg("-lc")
        .arg(command)
        .output()
        .map_err(|err| RenderError::Operational(format!("Failed to execute command: {err}")))?;

    let exit_code = output.status.code().unwrap_or(-1);

    Ok(CommandExecution {
        exit_code,
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
    })
}

fn expected_exit_code(entry: &Value) -> Result<i32, RenderError> {
    let Some(assertion) = entry.get("assert") else {
        return Ok(0);
    };

    let Some(exit_code_value) = assertion.get("exit_code") else {
        return Ok(0);
    };

    let exit_code = exit_code_value.as_i64().ok_or_else(|| {
        RenderError::Operational("Command assert.exit_code must be an integer".to_string())
    })?;

    i32::try_from(exit_code).map_err(|_| {
        RenderError::Operational(
            "Command assert.exit_code is outside the supported range".to_string(),
        )
    })
}

fn ensure_expected_exit_code(
    entry: &Value,
    execution: &CommandExecution,
) -> Result<(), RenderError> {
    let expected = expected_exit_code(entry)?;
    if execution.exit_code == expected {
        return Ok(());
    }

    Err(RenderError::CommandFailed(format!(
        "Command failed assertion: expected exit code {expected}, got {}",
        execution.exit_code
    )))
}

fn ensure_assert_checks(entry: &Value, execution: &CommandExecution) -> Result<(), RenderError> {
    let Some(assertion) = entry.get("assert") else {
        return Ok(());
    };
    let Some(checks) = assertion.get("checks") else {
        return Ok(());
    };
    let checks = checks.as_array().ok_or_else(|| {
        RenderError::Operational("Command assert.checks must be an array".to_string())
    })?;

    for check in checks {
        ensure_assert_check(check, execution)?;
    }

    Ok(())
}

fn ensure_assert_check(check: &Value, execution: &CommandExecution) -> Result<(), RenderError> {
    let source = check.get("source").and_then(Value::as_str).ok_or_else(|| {
        RenderError::Operational("Assertion check source must be a string".to_string())
    })?;

    if source != "stdout" {
        return Err(RenderError::Operational(format!(
            "Unsupported assertion check source `{source}`"
        )));
    }

    let expected = check
        .get("contains")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            RenderError::Operational("Assertion check contains must be a string".to_string())
        })?;

    if execution.stdout.contains(expected) {
        return Ok(());
    }

    Err(RenderError::CommandFailed(format!(
        "Command failed assertion: stdout did not contain `{expected}`"
    )))
}
