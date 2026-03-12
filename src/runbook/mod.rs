mod validate;

use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

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

        let rendered = match entry_type {
            "Heading" => render_heading(entry)?,
            "Markdown" => render_markdown_entry(entry)?,
            "Command" => match render_command(entry) {
                Ok(section) => section,
                Err(RenderError::Timeout {
                    message,
                    partial_markdown,
                }) => {
                    sections.push(partial_markdown);
                    let mut output = sections.join("\n\n");
                    if !output.ends_with('\n') {
                        output.push('\n');
                    }
                    return Err(RenderError::Timeout {
                        message,
                        partial_markdown: output,
                    });
                }
                Err(err) => return Err(err),
            },
            other => {
                return Err(RenderError::Operational(format!(
                    "Unsupported entry type `{other}`"
                )));
            }
        };

        sections.push(rendered);
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
    let execution = execute_command(&command_text, timeout_for_entry(entry)?)?;

    if execution.timed_out {
        if let Some(output) = entry.get("output") {
            if let Some(caption) = output.get("caption") {
                let caption_text = render_caption(caption)?;
                section.push_str("\n\n");
                section.push_str(&caption_text);
            }

            section.push_str("\n\n");
            section.push_str("```text\n");
            section.push_str(&execution.stdout);
            if !execution.stdout.ends_with('\n') && !execution.stdout.is_empty() {
                section.push('\n');
            }
            section.push_str("```");
        }

        return Err(RenderError::Timeout {
            message: format!(
                "Command timed out after {}",
                entry
                    .get("timeout")
                    .and_then(Value::as_str)
                    .unwrap_or("2 minutes")
            ),
            partial_markdown: section,
        });
    }

    ensure_expected_exit_code(entry, &execution)?;
    ensure_assert_checks(entry, &execution)?;

    if let Some(output) = entry.get("output") {
        if let Some(caption) = output.get("caption") {
            let caption_text = render_caption(caption)?;

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
    stderr: String,
    timed_out: bool,
}

fn execute_command(command: &str, timeout: Duration) -> Result<CommandExecution, RenderError> {
    let mut process = Command::new("sh");
    process.arg("-lc").arg(command);
    process.stdout(Stdio::piped()).stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        process.process_group(0);
    }

    let mut child = process
        .spawn()
        .map_err(|err| RenderError::Operational(format!("Failed to execute command: {err}")))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| RenderError::Operational("Failed to capture command stdout".to_string()))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| RenderError::Operational("Failed to capture command stderr".to_string()))?;

    let stdout_handle = thread::spawn(move || {
        let mut reader = stdout;
        let mut buffer = String::new();
        let _ = reader.read_to_string(&mut buffer);
        buffer
    });
    let stderr_handle = thread::spawn(move || {
        let mut reader = stderr;
        let mut buffer = String::new();
        let _ = reader.read_to_string(&mut buffer);
        buffer
    });

    let start = Instant::now();
    let mut timed_out = false;
    let exit_status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {
                if start.elapsed() >= timeout {
                    timed_out = true;
                    terminate_child(&mut child)?;
                    break child.wait().map_err(|err| {
                        RenderError::Operational(format!(
                            "Failed to wait for timed out command: {err}"
                        ))
                    })?;
                }
                thread::sleep(Duration::from_millis(25));
            }
            Err(err) => {
                return Err(RenderError::Operational(format!(
                    "Failed while waiting for command: {err}"
                )));
            }
        }
    };

    let stdout = stdout_handle
        .join()
        .map_err(|_| RenderError::Operational("Failed to collect command stdout".to_string()))?;
    let stderr = stderr_handle
        .join()
        .map_err(|_| RenderError::Operational("Failed to collect command stderr".to_string()))?;

    let exit_code = exit_status.code().unwrap_or(-1);

    Ok(CommandExecution {
        exit_code,
        stdout,
        stderr,
        timed_out,
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
        "{}{}",
        format!(
            "Command failed assertion: expected exit code {expected}, got {}",
            execution.exit_code
        ),
        if execution.stderr.trim().is_empty() {
            String::new()
        } else {
            format!(": {}", execution.stderr.trim())
        }
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

fn render_caption(caption: &Value) -> Result<String, RenderError> {
    match caption {
        Value::String(text) => Ok(text.to_string()),
        Value::Array(items) => {
            let mut lines = Vec::new();
            for item in items {
                lines.push(
                    item.as_str()
                        .ok_or_else(|| {
                            RenderError::Operational(
                                "Command output caption must contain only strings".to_string(),
                            )
                        })?
                        .to_string(),
                );
            }
            Ok(lines.join("\n"))
        }
        _ => Err(RenderError::Operational(
            "Command output caption must be a string or array of strings".to_string(),
        )),
    }
}

fn timeout_for_entry(entry: &Value) -> Result<Duration, RenderError> {
    let Some(timeout) = entry.get("timeout") else {
        return Ok(Duration::from_secs(120));
    };
    let timeout = timeout
        .as_str()
        .ok_or_else(|| RenderError::Operational("Command timeout must be a string".to_string()))?;
    parse_timeout(timeout).map_err(RenderError::Operational)
}

fn parse_timeout(timeout: &str) -> Result<Duration, String> {
    let parts: Vec<_> = timeout.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Command timeout must be a number followed by a unit".to_string());
    }

    let value: u64 = parts[0]
        .parse()
        .map_err(|_| "Command timeout must start with a whole number".to_string())?;

    let seconds = match parts[1].to_ascii_lowercase().as_str() {
        "second" | "seconds" | "sec" | "secs" | "s" => value,
        "minute" | "minutes" | "min" | "mins" | "m" => value
            .checked_mul(60)
            .ok_or_else(|| "Command timeout is too large".to_string())?,
        _ => {
            return Err(
                "Command timeout unit must be seconds or minutes (or a common abbreviation)"
                    .to_string(),
            );
        }
    };

    Ok(Duration::from_secs(seconds))
}

fn terminate_child(child: &mut std::process::Child) -> Result<(), RenderError> {
    #[cfg(unix)]
    {
        let status = Command::new("kill")
            .arg("-9")
            .arg(format!("-{}", child.id()))
            .status()
            .map_err(|err| {
                RenderError::Operational(format!("Failed to terminate timed out command: {err}"))
            })?;
        if !status.success() {
            child.kill().map_err(|err| {
                RenderError::Operational(format!("Failed to terminate timed out command: {err}"))
            })?;
        }
        return Ok(());
    }

    #[cfg(not(unix))]
    {
        child.kill().map_err(|err| {
            RenderError::Operational(format!("Failed to terminate timed out command: {err}"))
        })
    }
}
