use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    std::env::temp_dir().join(format!("sw-test-{}-{nanos}", std::process::id()))
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
    write_runbook(&dir, "sw-runbook-anonymized.json", "sw-runbook.json");

    let output = run_in_dir(&[], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rendered runbook to readme.md"));

    let readme = fs::read_to_string(dir.join("readme.md")).expect("missing readme output");
    assert!(readme.contains("# Audio - Example AI Provider API"));
    assert!(readme.contains("```shell"));
}

#[test]
fn run_command_writes_requested_output_file() {
    let dir = prepare_workspace();
    write_runbook(&dir, "sw-runbook-anonymized.json", "example.json");

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
    assert!(readme.contains("## Prerequisites"));
    assert!(readme.contains("The program generates an audio file from text,"));
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
