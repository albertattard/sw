use super::RenderError;
use serde_json::Value;
use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

pub(crate) struct CommandExecution {
    pub(crate) exit_code: i32,
    pub(crate) stdout: String,
    pub(crate) stderr: String,
    pub(crate) timed_out: bool,
}

pub(crate) fn cleanup_block(entry: &Value) -> Result<Option<Vec<String>>, RenderError> {
    let Some(cleanup) = entry.get("cleanup") else {
        return Ok(None);
    };

    let cleanup_lines = cleanup
        .as_array()
        .ok_or_else(|| RenderError::Operational("Command cleanup must be an array".to_string()))?;

    let mut lines = Vec::new();
    for item in cleanup_lines {
        lines.push(
            item.as_str()
                .ok_or_else(|| {
                    RenderError::Operational(
                        "Command cleanup must contain only strings".to_string(),
                    )
                })?
                .to_string(),
        );
    }

    Ok(Some(lines))
}

pub(crate) fn execute_command(
    entry: &Value,
    command: &str,
) -> Result<CommandExecution, RenderError> {
    let timeout = timeout_for_entry(entry)?;

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

    Ok(CommandExecution {
        exit_code: exit_status.code().unwrap_or(-1),
        stdout,
        stderr,
        timed_out,
    })
}

pub(crate) fn ensure_assertions(
    entry: &Value,
    execution: &CommandExecution,
) -> Result<(), RenderError> {
    ensure_expected_exit_code(entry, execution)?;
    ensure_assert_checks(entry, execution)?;
    Ok(())
}

pub(crate) fn timeout_label(entry: &Value) -> String {
    entry
        .get("timeout")
        .and_then(Value::as_str)
        .unwrap_or("2 minutes")
        .to_string()
}

pub(crate) fn run_cleanup_blocks(cleanups: &[Vec<String>]) -> Vec<String> {
    let mut failures = Vec::new();

    for cleanup in cleanups.iter().rev() {
        if let Err(message) = run_cleanup_block(cleanup) {
            failures.push(message);
        }
    }

    failures
}

fn run_cleanup_block(cleanup: &[String]) -> Result<(), String> {
    let script = cleanup_script(cleanup);
    let output = Command::new("sh")
        .arg("-lc")
        .arg(script)
        .output()
        .map_err(|err| format!("Failed to execute cleanup: {err}"))?;

    if output.status.success() {
        return Ok(());
    }

    let exit_code = output.status.code().unwrap_or(-1);
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if stderr.is_empty() {
        Err(format!("Cleanup failed with exit code {exit_code}"))
    } else {
        Err(format!(
            "Cleanup failed with exit code {exit_code}: {stderr}"
        ))
    }
}

fn cleanup_script(cleanup: &[String]) -> String {
    let mut script = String::from("status=0\n");
    for line in cleanup {
        script.push_str("{ ");
        script.push_str(line);
        script.push_str("; } || status=$?\n");
    }
    script.push_str("exit $status\n");
    script
}

fn ensure_expected_exit_code(
    entry: &Value,
    execution: &CommandExecution,
) -> Result<(), RenderError> {
    let expected = expected_exit_code(entry)?;
    if execution.exit_code == expected {
        return Ok(());
    }

    let suffix = if execution.stderr.trim().is_empty() {
        String::new()
    } else {
        format!(": {}", execution.stderr.trim())
    };

    Err(RenderError::CommandFailed(format_assertion_failure(
        entry,
        execution,
        &format!(
            "expected exit code {expected}, got {}{suffix}",
            execution.exit_code
        ),
    )))
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
        ensure_assert_check(entry, check, execution)?;
    }

    Ok(())
}

fn ensure_assert_check(
    entry: &Value,
    check: &Value,
    execution: &CommandExecution,
) -> Result<(), RenderError> {
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

    Err(RenderError::CommandFailed(format_assertion_failure(
        entry,
        execution,
        &format!("stdout did not contain `{expected}`"),
    )))
}

fn format_assertion_failure(entry: &Value, execution: &CommandExecution, detail: &str) -> String {
    format!(
        "Command failed assertion for entry:\n{}\nstdout:\n{}\nstderr:\n{}\n{detail}",
        format_command_entry(entry),
        format_command_stream(&execution.stdout),
        format_command_stream(&execution.stderr),
    )
}

fn format_command_entry(entry: &Value) -> String {
    serde_json::to_string_pretty(entry)
        .unwrap_or_else(|_| "<failed to serialize command entry>".to_string())
}

fn format_command_stream(stream: &str) -> String {
    if stream.is_empty() {
        "(empty)".to_string()
    } else {
        stream.to_string()
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
        Ok(())
    }

    #[cfg(not(unix))]
    {
        child.kill().map_err(|err| {
            RenderError::Operational(format!("Failed to terminate timed out command: {err}"))
        })
    }
}
