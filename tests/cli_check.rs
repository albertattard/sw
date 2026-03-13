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
fn check_succeeds_when_prerequisites_pass_and_skips_main_commands() {
    let dir = prepare_workspace();
    write_runbook(
        &dir,
        "sw-runbook-run-prerequisites-success.json",
        "sw-runbook.json",
    );

    let output = run_in_dir(&["check"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("All prerequisite checks passed"));
    assert!(!dir.join("README.md").exists());
    assert_eq!(
        fs::read_to_string(dir.join("prereq-order.txt")).expect("missing prereq-order.txt"),
        "prereq\n"
    );
}

#[test]
fn check_fails_when_prerequisite_fails_and_skips_main_commands() {
    let dir = prepare_workspace();
    write_runbook(
        &dir,
        "sw-runbook-run-prerequisites-failure.json",
        "sw-runbook.json",
    );

    let output = run_in_dir(&["check"], &dir);

    assert_eq!(output.status.code(), Some(2));
    assert!(!dir.join("README.md").exists());
    assert_eq!(
        fs::read_to_string(dir.join("prereq-failure-order.txt"))
            .expect("missing prereq-failure-order.txt"),
        "prereq\n"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Prerequisite failed: Docker daemon"));
    assert!(stderr.contains("Start Docker Desktop before running this example."));
}

#[test]
fn check_returns_operational_error_for_invalid_runbook() {
    let dir = prepare_workspace();
    write_runbook(
        &dir,
        "sw-runbook-invalid-prerequisites.json",
        "sw-runbook.json",
    );

    let output = run_in_dir(&["check"], &dir);

    assert_eq!(output.status.code(), Some(1));
    assert!(!dir.join("README.md").exists());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Runbook is invalid"));
}

#[test]
fn check_succeeds_without_prerequisites_and_skips_commands() {
    let dir = prepare_workspace();
    write_runbook(&dir, "sw-runbook-run-success.json", "sw-runbook.json");

    let output = run_in_dir(&["check"], &dir);

    assert!(output.status.success());
    assert!(!dir.join("README.md").exists());
    assert!(!dir.join("sequence.txt").exists());
}

#[test]
fn check_missing_file_returns_operational_error() {
    let dir = prepare_workspace();

    let output = run_in_dir(&["check"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to read"));
}
