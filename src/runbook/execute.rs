use super::RenderError;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

pub(crate) struct CommandExecution {
    pub(crate) exit_code: i32,
    pub(crate) stdout: String,
    pub(crate) stderr: String,
    pub(crate) timed_out: bool,
}

pub(crate) struct CleanupBlock {
    pub(crate) lines: Vec<String>,
    pub(crate) working_dir: PathBuf,
}

pub(crate) fn patch_target_path(
    entry: &Value,
    runbook_path: &Path,
) -> Result<(PathBuf, String), RenderError> {
    let relative_path = entry
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| RenderError::Operational("Patch entry is missing path".to_string()))?;
    let base_dir = runbook_base_dir(runbook_path);
    Ok((base_dir.join(relative_path), relative_path.to_string()))
}

pub(crate) fn snapshot_patch_target(path: &Path) -> Result<Vec<u8>, RenderError> {
    fs::read(path).map_err(|err| {
        RenderError::Operational(format!("Failed to read {}: {err}", path.display()))
    })
}

pub(crate) fn apply_patch_entry(
    runbook_path: &Path,
    relative_path: &str,
    patch_lines: &[String],
) -> Result<(), RenderError> {
    let base_dir = runbook_base_dir(runbook_path);
    let patch_text = patch_text_for(relative_path, patch_lines);
    let mut child = Command::new("patch")
        .arg("--strip=0")
        .arg("--quiet")
        .arg("--force")
        .arg("-V")
        .arg("none")
        .arg("--reject-file")
        .arg("-")
        .current_dir(base_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| RenderError::Operational(format!("Failed to execute patch: {err}")))?;

    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin.write_all(patch_text.as_bytes()).map_err(|err| {
            RenderError::Operational(format!("Failed to write patch stdin: {err}"))
        })?;
    }

    let output = child.wait_with_output().map_err(|err| {
        RenderError::Operational(format!("Failed to wait for patch command: {err}"))
    })?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let detail = if !stderr.is_empty() {
        stderr
    } else if !stdout.is_empty() {
        stdout
    } else {
        format!(
            "patch exited with code {}",
            output.status.code().unwrap_or(-1)
        )
    };

    Err(RenderError::Operational(format!(
        "Failed to apply patch for {relative_path}: {detail}"
    )))
}

fn uses_automatic_process_cleanup(entry: &Value) -> bool {
    entry.get("cleanup").is_none()
}

pub(crate) fn cleanup_block(
    entry: &Value,
    runbook_path: &Path,
) -> Result<Option<CleanupBlock>, RenderError> {
    let Some(cleanup) = entry.get("cleanup") else {
        return Ok(None);
    };

    Ok(Some(CleanupBlock {
        lines: string_lines(
            cleanup,
            "Command cleanup must be a string or an array of strings",
            "Command cleanup must contain only strings",
        )?,
        working_dir: resolve_command_working_dir(entry, runbook_path)?,
    }))
}

fn string_lines(
    value: &Value,
    type_message: &str,
    item_message: &str,
) -> Result<Vec<String>, RenderError> {
    match value {
        Value::String(text) => Ok(split_multiline_string(text)),
        Value::Array(items) => {
            let mut lines = Vec::new();
            for item in items {
                lines.push(
                    item.as_str()
                        .ok_or_else(|| RenderError::Operational(item_message.to_string()))?
                        .to_string(),
                );
            }
            Ok(lines)
        }
        _ => Err(RenderError::Operational(type_message.to_string())),
    }
}

fn split_multiline_string(value: &str) -> Vec<String> {
    value
        .split_terminator('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line).to_string())
        .collect()
}

pub(crate) fn execute_command(
    entry: &Value,
    command: &str,
    runbook_path: &Path,
) -> Result<CommandExecution, RenderError> {
    let timeout = timeout_for_entry(entry)?;
    let auto_cleanup_processes = uses_automatic_process_cleanup(entry);
    let working_dir = resolve_command_working_dir(entry, runbook_path)?;

    let mut process = Command::new("sh");
    process.arg("-lc").arg(command);
    process.current_dir(&working_dir);
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

    let (stdout_rx, stdout_handle) = spawn_stream_reader(stdout);
    let (stderr_rx, stderr_handle) = spawn_stream_reader(stderr);

    let start = Instant::now();
    let mut timed_out = false;
    let process_group_id = child.id();
    let mut exit_status: Option<ExitStatus> = None;
    let mut stdout = None;
    let mut stderr = None;
    let mut auto_cleanup_applied = false;

    loop {
        if exit_status.is_none() {
            match child.try_wait() {
                Ok(Some(status)) => exit_status = Some(status),
                Ok(None) => {}
                Err(err) => {
                    return Err(RenderError::Operational(format!(
                        "Failed while waiting for command: {err}"
                    )));
                }
            }
        }

        if exit_status.is_some() && auto_cleanup_processes && !timed_out && !auto_cleanup_applied {
            terminate_process_group(process_group_id)?;
            auto_cleanup_applied = true;
        }

        if stdout.is_none() {
            stdout = receive_stream_output(&stdout_rx, "stdout")?;
        }
        if stderr.is_none() {
            stderr = receive_stream_output(&stderr_rx, "stderr")?;
        }

        if exit_status.is_some() && stdout.is_some() && stderr.is_some() {
            let exit_status = exit_status.take().expect("missing exit status");
            let stdout = stdout.take().expect("missing stdout");
            let stderr = stderr.take().expect("missing stderr");

            stdout_handle.join().map_err(|_| {
                RenderError::Operational("Failed to collect command stdout".to_string())
            })?;
            stderr_handle.join().map_err(|_| {
                RenderError::Operational("Failed to collect command stderr".to_string())
            })?;

            return Ok(CommandExecution {
                exit_code: exit_status.code().unwrap_or(-1),
                stdout,
                stderr,
                timed_out,
            });
        }

        if !timed_out && start.elapsed() >= timeout {
            timed_out = true;
            terminate_timed_out_command(&mut child, process_group_id, exit_status.is_some())?;
        }

        thread::sleep(Duration::from_millis(25));
    }
}

fn spawn_stream_reader<T>(stream: T) -> (Receiver<String>, thread::JoinHandle<()>)
where
    T: Read + Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let mut reader = stream;
        let mut buffer = String::new();
        let _ = reader.read_to_string(&mut buffer);
        let _ = tx.send(buffer);
    });

    (rx, handle)
}

fn receive_stream_output(
    receiver: &Receiver<String>,
    stream_name: &str,
) -> Result<Option<String>, RenderError> {
    match receiver.try_recv() {
        Ok(output) => Ok(Some(output)),
        Err(TryRecvError::Empty) => Ok(None),
        Err(TryRecvError::Disconnected) => Err(RenderError::Operational(format!(
            "Failed to collect command {stream_name}"
        ))),
    }
}

fn terminate_timed_out_command(
    child: &mut std::process::Child,
    process_group_id: u32,
    child_exited: bool,
) -> Result<(), RenderError> {
    #[cfg(unix)]
    {
        if child_exited {
            terminate_process_group(process_group_id)?;
            return Ok(());
        }
    }

    terminate_child(child)
}

pub(crate) fn ensure_assertions(
    entry: &Value,
    execution: &CommandExecution,
    runbook_path: &Path,
) -> Result<(), RenderError> {
    ensure_expected_exit_code(entry, execution)?;
    ensure_assert_checks(entry, execution, runbook_path)?;
    Ok(())
}

pub(crate) fn timeout_label(entry: &Value) -> String {
    entry
        .get("timeout")
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .unwrap_or_else(|| default_timeout_label(entry).to_string())
}

pub(crate) fn run_cleanup_blocks(cleanups: &[CleanupBlock]) -> Vec<String> {
    let mut failures = Vec::new();

    for cleanup in cleanups.iter().rev() {
        if let Err(message) = run_cleanup_block(cleanup) {
            failures.push(message);
        }
    }

    failures
}

pub(crate) fn run_patch_restores(
    restore_stack: &[PathBuf],
    snapshots: &HashMap<PathBuf, Vec<u8>>,
) -> Vec<String> {
    let mut failures = Vec::new();

    for path in restore_stack.iter().rev() {
        let Some(original_bytes) = snapshots.get(path) else {
            failures.push(format!(
                "Patch restore failed for {}: missing original snapshot",
                path.display()
            ));
            continue;
        };

        if let Err(err) = fs::write(path, original_bytes) {
            failures.push(format!(
                "Patch restore failed for {}: {err}",
                path.display()
            ));
        }
    }

    failures
}

fn run_cleanup_block(cleanup: &CleanupBlock) -> Result<(), String> {
    let script = cleanup_script(&cleanup.lines);
    let output = Command::new("sh")
        .arg("-lc")
        .arg(script)
        .current_dir(&cleanup.working_dir)
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
    for chunk in cleanup_chunks(cleanup) {
        script.push_str("{\n");
        for line in chunk {
            script.push_str(&line);
            script.push('\n');
        }
        script.push_str("} || status=$?\n");
    }
    script.push_str("exit $status\n");
    script
}

fn cleanup_chunks(cleanup: &[String]) -> Vec<Vec<String>> {
    let mut chunks = Vec::new();
    let mut current = Vec::new();
    let mut compound_depth = 0usize;
    let mut brace_depth = 0usize;

    for line in cleanup {
        current.push(line.clone());
        let trimmed = line.trim();

        if starts_compound_block(trimmed) {
            compound_depth += 1;
        }
        brace_depth += opens_brace_block(trimmed);

        if closes_compound_block(trimmed) && compound_depth > 0 {
            compound_depth -= 1;
        }
        let closed_braces = closes_brace_block(trimmed);
        brace_depth = brace_depth.saturating_sub(closed_braces);

        if compound_depth == 0 && brace_depth == 0 {
            chunks.push(std::mem::take(&mut current));
        }
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

fn starts_compound_block(line: &str) -> bool {
    let token = leading_token(line);
    matches!(token, Some("if" | "for" | "while" | "until" | "case"))
}

fn closes_compound_block(line: &str) -> bool {
    let token = leading_token(line);
    matches!(token, Some("fi" | "done" | "esac"))
}

fn opens_brace_block(line: &str) -> usize {
    let token = leading_token(line);
    usize::from(matches!(token, Some("{")))
}

fn closes_brace_block(line: &str) -> usize {
    let token = leading_token(line);
    usize::from(matches!(token, Some("}")))
}

fn leading_token(line: &str) -> Option<&str> {
    line.split_whitespace().next()
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

    Err(RenderError::CommandFailed(format_command_failure(
        entry,
        execution,
        "Command failed assertion for entry",
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

fn ensure_assert_checks(
    entry: &Value,
    execution: &CommandExecution,
    runbook_path: &Path,
) -> Result<(), RenderError> {
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
        ensure_assert_check(entry, check, execution, runbook_path)?;
    }

    Ok(())
}

fn ensure_assert_check(
    entry: &Value,
    check: &Value,
    execution: &CommandExecution,
    runbook_path: &Path,
) -> Result<(), RenderError> {
    let source = check.get("source").and_then(Value::as_str).ok_or_else(|| {
        RenderError::Operational("Assertion check source must be a string".to_string())
    })?;

    match source {
        "stdout" => ensure_stdout_assert_check(entry, check, execution),
        "file" => ensure_file_assert_check(entry, check, execution, runbook_path),
        _ => Err(RenderError::Operational(format!(
            "Unsupported assertion check source `{source}`"
        ))),
    }
}

fn ensure_stdout_assert_check(
    entry: &Value,
    check: &Value,
    execution: &CommandExecution,
) -> Result<(), RenderError> {
    let expected = check
        .get("contains")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            RenderError::Operational("Assertion check contains must be a string".to_string())
        })?;

    if execution.stdout.contains(expected) {
        return Ok(());
    }

    Err(RenderError::CommandFailed(format_command_failure(
        entry,
        execution,
        "Command failed assertion for entry",
        &format!("stdout did not contain `{expected}`"),
    )))
}

fn ensure_file_assert_check(
    entry: &Value,
    check: &Value,
    execution: &CommandExecution,
    runbook_path: &Path,
) -> Result<(), RenderError> {
    let path = check.get("path").and_then(Value::as_str).ok_or_else(|| {
        RenderError::Operational("Assertion check path must be a string".to_string())
    })?;

    let assertion_path = resolve_assertion_path(entry, runbook_path, path)?;

    if check.get("exists").is_some() {
        return ensure_file_exists_assertion(entry, execution, &assertion_path, path);
    }

    let expected = check.get("sha256").and_then(Value::as_str).ok_or_else(|| {
        RenderError::Operational("Assertion check sha256 must be a string".to_string())
    })?;

    ensure_file_sha256_assertion(entry, execution, &assertion_path, path, expected)
}

fn ensure_file_exists_assertion(
    entry: &Value,
    execution: &CommandExecution,
    path: &Path,
    display_path: &str,
) -> Result<(), RenderError> {
    if path.exists() {
        return Ok(());
    }

    Err(RenderError::CommandFailed(format_command_failure(
        entry,
        execution,
        "Command failed assertion for entry",
        &format!("file `{display_path}` did not exist"),
    )))
}

fn ensure_file_sha256_assertion(
    entry: &Value,
    execution: &CommandExecution,
    path: &Path,
    display_path: &str,
    expected: &str,
) -> Result<(), RenderError> {
    let contents = fs::read(path).map_err(|_| {
        RenderError::CommandFailed(format_command_failure(
            entry,
            execution,
            "Command failed assertion for entry",
            &format!("file `{display_path}` did not exist"),
        ))
    })?;

    let actual = sha256_hex(&contents);
    if actual == expected {
        return Ok(());
    }

    Err(RenderError::CommandFailed(format_command_failure(
        entry,
        execution,
        "Command failed assertion for entry",
        &format!("file `{display_path}` had sha256 `{actual}` instead of `{expected}`"),
    )))
}

fn sha256_hex(contents: &[u8]) -> String {
    let digest = Sha256::digest(contents);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(crate) fn format_command_failure(
    entry: &Value,
    execution: &CommandExecution,
    header: &str,
    detail: &str,
) -> String {
    format!(
        "{header}:\n{}\nstdout:\n{}\nstderr:\n{}\n{detail}",
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
        return Ok(default_timeout_duration(entry));
    };
    let timeout = timeout
        .as_str()
        .ok_or_else(|| RenderError::Operational("Command timeout must be a string".to_string()))?;
    parse_timeout(timeout).map_err(RenderError::Operational)
}

fn default_timeout_duration(entry: &Value) -> Duration {
    match default_timeout_label(entry) {
        "5 seconds" => Duration::from_secs(5),
        _ => Duration::from_secs(30),
    }
}

fn default_timeout_label(entry: &Value) -> &'static str {
    match entry.get("kind").and_then(Value::as_str) {
        Some("command") => "5 seconds",
        _ => "30 seconds",
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }

    normalized
}

fn normalize_to_absolute(path: &Path) -> Result<PathBuf, RenderError> {
    if path.is_absolute() {
        return Ok(normalize_path(path));
    }

    let current_dir = std::env::current_dir().map_err(|err| {
        RenderError::Operational(format!("Failed to resolve current directory: {err}"))
    })?;
    Ok(normalize_path(&current_dir.join(path)))
}

fn resolve_command_working_dir(entry: &Value, runbook_path: &Path) -> Result<PathBuf, RenderError> {
    let base_dir = normalize_to_absolute(runbook_base_dir(runbook_path))?;

    let Some(working_dir) = entry.get("working_dir") else {
        return Ok(base_dir);
    };

    let working_dir = working_dir.as_str().ok_or_else(|| {
        RenderError::Operational("Command working_dir must be a string".to_string())
    })?;
    let working_dir_path = Path::new(working_dir);

    if working_dir_path.is_absolute() {
        return Err(RenderError::Operational(
            "Command working_dir must be a relative path".to_string(),
        ));
    }

    let resolved_dir = normalize_to_absolute(&base_dir.join(working_dir_path))?;
    if !resolved_dir.starts_with(&base_dir) {
        return Err(RenderError::Operational(
            "Command working_dir must stay within the runbook directory".to_string(),
        ));
    }

    if !resolved_dir.exists() {
        return Err(RenderError::Operational(format!(
            "Command working_dir `{working_dir}` did not exist"
        )));
    }

    if !resolved_dir.is_dir() {
        return Err(RenderError::Operational(format!(
            "Command working_dir `{working_dir}` was not a directory"
        )));
    }

    Ok(resolved_dir)
}

fn resolve_assertion_path(
    entry: &Value,
    runbook_path: &Path,
    assertion_path: &str,
) -> Result<PathBuf, RenderError> {
    let path = Path::new(assertion_path);
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    Ok(resolve_command_working_dir(entry, runbook_path)?.join(path))
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

fn runbook_base_dir(runbook_path: &Path) -> &Path {
    runbook_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

fn patch_text_for(relative_path: &str, patch_lines: &[String]) -> String {
    let patch_body = patch_lines.join("\n");
    if patch_starts_with_headers(&patch_body) {
        if patch_body.ends_with('\n') {
            patch_body
        } else {
            format!("{patch_body}\n")
        }
    } else {
        format!("--- {relative_path}\n+++ {relative_path}\n{patch_body}\n")
    }
}

fn patch_starts_with_headers(patch_body: &str) -> bool {
    patch_body.starts_with("--- ") || patch_body.starts_with("diff ")
}

fn terminate_child(child: &mut std::process::Child) -> Result<(), RenderError> {
    #[cfg(unix)]
    {
        terminate_process_group(child.id())?;

        for _ in 0..5 {
            match child.try_wait() {
                Ok(Some(_)) => return Ok(()),
                Ok(None) => thread::sleep(Duration::from_millis(20)),
                Err(err) => {
                    return Err(RenderError::Operational(format!(
                        "Failed to inspect timed out command state: {err}"
                    )));
                }
            }
        }

        child.kill().map_err(|err| {
            RenderError::Operational(format!("Failed to terminate timed out command: {err}"))
        })?;
        Ok(())
    }

    #[cfg(not(unix))]
    {
        child.kill().map_err(|err| {
            RenderError::Operational(format!("Failed to terminate timed out command: {err}"))
        })
    }
}

#[cfg(unix)]
fn terminate_process_group(process_group_id: u32) -> Result<bool, RenderError> {
    let output = Command::new("kill")
        .arg("-9")
        .arg(format!("-{process_group_id}"))
        .output()
        .map_err(|err| {
            RenderError::Operational(format!("Failed to terminate command processes: {err}"))
        })?;

    if output.status.success() {
        return Ok(true);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.contains("No such process") {
        return Ok(true);
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::{
        cleanup_block, cleanup_chunks, cleanup_script, patch_text_for, resolve_assertion_path,
        resolve_command_working_dir, run_patch_restores, split_multiline_string, timeout_for_entry,
        timeout_label,
    };
    use serde_json::json;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;
    use std::time::Duration;

    #[test]
    fn cleanup_chunks_keep_multiline_if_block_together() {
        let cleanup = vec![
            "NAME='cleanup context'".to_string(),
            "if [ -n \"$NAME\" ]; then".to_string(),
            "  printf '%s\\n' \"$NAME\" >> cleanup.txt".to_string(),
            "fi".to_string(),
            "printf 'done\\n' >> cleanup.txt".to_string(),
        ];

        let chunks = cleanup_chunks(&cleanup);

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], vec!["NAME='cleanup context'".to_string()]);
        assert_eq!(
            chunks[1],
            vec![
                "if [ -n \"$NAME\" ]; then".to_string(),
                "  printf '%s\\n' \"$NAME\" >> cleanup.txt".to_string(),
                "fi".to_string()
            ]
        );
        assert_eq!(
            chunks[2],
            vec!["printf 'done\\n' >> cleanup.txt".to_string()]
        );
    }

    #[test]
    fn cleanup_script_wraps_each_top_level_chunk() {
        let cleanup = vec![
            "false".to_string(),
            "if [ -f cleanup.txt ]; then".to_string(),
            "  printf 'cleanup\\n' >> cleanup.txt".to_string(),
            "fi".to_string(),
        ];

        let script = cleanup_script(&cleanup);

        assert!(script.contains("{\nfalse\n} || status=$?\n"));
        assert!(script.contains(
            "{\nif [ -f cleanup.txt ]; then\n  printf 'cleanup\\n' >> cleanup.txt\nfi\n} || status=$?\n"
        ));
    }

    #[test]
    fn cleanup_block_accepts_scalar_script_and_drops_only_terminator_blank_line() {
        let entry = json!({
            "type": "Command",
            "cleanup": "first\n\n"
        });

        let cleanup = match cleanup_block(&entry, Path::new("sw-runbook.json")) {
            Ok(cleanup) => cleanup,
            Err(_) => panic!("cleanup block should parse"),
        };

        assert_eq!(
            cleanup.map(|cleanup| cleanup.lines),
            Some(vec!["first".to_string(), "".to_string()])
        );
    }

    #[test]
    fn resolve_command_working_dir_uses_runbook_relative_directory() {
        let dir = std::env::temp_dir().join(format!("sw-working-dir-{}", std::process::id()));
        let nested = dir.join("nested/demo");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&nested).expect("create nested dir");

        let entry = json!({
            "type": "Command",
            "working_dir": "nested/demo"
        });

        let resolved = match resolve_command_working_dir(&entry, &dir.join("sw-runbook.yaml")) {
            Ok(path) => path,
            Err(_) => panic!("resolve dir"),
        };
        assert_eq!(resolved, nested);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn resolve_assertion_path_uses_command_working_dir_for_relative_paths() {
        let dir = std::env::temp_dir().join(format!("sw-assert-path-{}", std::process::id()));
        let nested = dir.join("nested/demo");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&nested).expect("create nested dir");

        let entry = json!({
            "type": "Command",
            "working_dir": "nested/demo"
        });

        let resolved = match resolve_assertion_path(&entry, &dir.join("sw-runbook.yaml"), "out.txt")
        {
            Ok(path) => path,
            Err(_) => panic!("resolve assertion path"),
        };
        assert_eq!(resolved, nested.join("out.txt"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn patch_text_adds_default_headers_when_missing() {
        let text = patch_text_for(
            "./src/main.rs",
            &[
                "@@ -1 +1 @@".to_string(),
                "-old".to_string(),
                "+new".to_string(),
            ],
        );

        assert!(text.starts_with("--- ./src/main.rs\n+++ ./src/main.rs\n"));
    }

    #[test]
    fn command_entries_default_to_thirty_seconds() {
        let entry = json!({
            "type": "Command",
            "commands": ["echo hi"]
        });

        assert_eq!(
            timeout_for_entry(&entry).unwrap_or_else(|_| panic!("timeout parse failed")),
            Duration::from_secs(30)
        );
        assert_eq!(timeout_label(&entry), "30 seconds");
    }

    #[test]
    fn command_prerequisite_checks_default_to_five_seconds() {
        let entry = json!({
            "kind": "command",
            "name": "Demo prerequisite",
            "commands": ["echo hi"]
        });

        assert_eq!(
            timeout_for_entry(&entry).unwrap_or_else(|_| panic!("timeout parse failed")),
            Duration::from_secs(5)
        );
        assert_eq!(timeout_label(&entry), "5 seconds");
    }

    #[test]
    fn explicit_timeout_overrides_prerequisite_default() {
        let entry = json!({
            "kind": "command",
            "name": "Demo prerequisite",
            "commands": ["echo hi"],
            "timeout": "9 seconds"
        });

        assert_eq!(
            timeout_for_entry(&entry).unwrap_or_else(|_| panic!("timeout parse failed")),
            Duration::from_secs(9)
        );
        assert_eq!(timeout_label(&entry), "9 seconds");
    }

    #[test]
    fn patch_restores_continue_after_a_failure() {
        let dir = std::env::temp_dir().join(format!("sw-patch-restore-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create dir");

        let good = dir.join("good.txt");
        let bad = dir.join("bad-dir");
        fs::write(&good, "patched").expect("write good");
        fs::create_dir_all(&bad).expect("create bad dir");

        let mut snapshots = HashMap::new();
        snapshots.insert(good.clone(), b"original".to_vec());
        snapshots.insert(bad.clone(), b"won't work".to_vec());

        let restore_stack = vec![good.clone(), bad.clone(), good.clone()];
        let failures = run_patch_restores(&restore_stack, &snapshots);

        assert!(!failures.is_empty());
        assert_eq!(
            fs::read_to_string(&good).expect("good restored"),
            "original"
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn split_multiline_string_drops_terminator_only_trailing_blank_line() {
        assert_eq!(
            split_multiline_string("first\nsecond\n"),
            vec!["first".to_string(), "second".to_string()]
        );
    }

    #[test]
    fn split_multiline_string_preserves_explicit_blank_line_before_terminator() {
        assert_eq!(
            split_multiline_string("first\n\n"),
            vec!["first".to_string(), "".to_string()]
        );
    }
}
