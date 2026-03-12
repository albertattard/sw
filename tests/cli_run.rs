use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn unique_temp_dir() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("sw-test-{}-{nanos}-{id}", std::process::id()))
}

fn prepare_workspace() -> PathBuf {
    let dir = unique_temp_dir();
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

fn run_in_dir(args: &[&str], dir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sw"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to execute sw")
}

fn write_runbook(dir: &Path, fixture_name: &str, target_name: &str) -> PathBuf {
    let source = Path::new("tests/fixtures").join(fixture_name);
    let target = dir.join(target_name);
    let contents = fs::read_to_string(source).expect("failed to read fixture");
    fs::write(&target, contents).expect("failed to write fixture");
    target
}

#[test]
fn no_args_defaults_to_run_and_writes_readme() {
    let dir = prepare_workspace();
    write_runbook(&dir, "sw-runbook-run-success.json", "sw-runbook.json");

    let output = run_in_dir(&[], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rendered runbook to readme.md"));

    let readme = fs::read_to_string(dir.join("readme.md")).expect("missing readme output");
    assert!(readme.contains("# Runbook execution"));
    assert!(readme.contains("```shell"));
    assert!(readme.contains("```text\nHello there\n```"));
    assert!(!readme.contains("```text\nfirst\n```"));
    assert_eq!(
        fs::read_to_string(dir.join("sequence.txt")).expect("missing command side effect"),
        "first\nsecond\n"
    );
}

#[test]
fn run_command_writes_requested_output_file() {
    let dir = prepare_workspace();
    write_runbook(&dir, "sw-runbook-run-success.json", "example.json");

    let output = run_in_dir(
        &[
            "run",
            "--input-file",
            "example.json",
            "--output-format",
            "markdown",
            "--output-file",
            "custom.md",
        ],
        &dir,
    );

    assert!(output.status.success());
    let readme = fs::read_to_string(dir.join("custom.md")).expect("missing custom output");
    assert!(readme.contains("Captured output"));
    assert!(readme.contains("```text\nHello there\n```"));
    assert!(!readme.contains("```text\nfirst\n```"));
}

#[test]
fn multiline_command_lines_share_the_same_shell_context() {
    let dir = prepare_workspace();
    write_runbook(&dir, "sw-runbook-run-multiline.json", "sw-runbook.json");

    let output = run_in_dir(&["run"], &dir);

    assert!(output.status.success());
    let readme = fs::read_to_string(dir.join("readme.md")).expect("missing readme output");
    assert!(readme.contains("NAME='Albert Attard'\necho ${NAME}"));
    assert!(readme.contains("Name output"));
    assert!(readme.contains("```text\nAlbert Attard\n```"));
}

#[test]
fn invalid_runbook_returns_validation_failure_without_output_file() {
    let dir = prepare_workspace();
    write_runbook(&dir, "sw-runbook-missing-field.json", "sw-runbook.json");

    let output = run_in_dir(&["run"], &dir);

    assert_eq!(output.status.code(), Some(2));
    assert!(!dir.join("readme.md").exists());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Runbook is invalid"));
}

#[test]
fn missing_input_file_returns_operational_error() {
    let dir = prepare_workspace();

    let output = run_in_dir(&["run"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to read"));
}

#[test]
fn command_failure_returns_exit_code_two_without_output_file() {
    let dir = prepare_workspace();
    write_runbook(&dir, "sw-runbook-run-failure.json", "sw-runbook.json");

    let output = run_in_dir(&["run"], &dir);

    assert_eq!(output.status.code(), Some(2));
    assert!(!dir.join("readme.md").exists());
    assert_eq!(
        fs::read_to_string(dir.join("failure-sequence.txt"))
            .expect("missing side-effect from first command"),
        "before failure\n"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Command failed with status"));
}
