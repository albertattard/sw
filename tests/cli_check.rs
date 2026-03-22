use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

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

fn run_in_dir_with_stdin(args: &[&str], dir: &Path, stdin: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_sw"))
        .args(args)
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute sw");

    child
        .stdin
        .as_mut()
        .expect("missing stdin pipe")
        .write_all(stdin.as_bytes())
        .expect("failed to write stdin");

    child.wait_with_output().expect("failed to read sw output")
}

fn run_in_dir_with_env(args: &[&str], dir: &Path, envs: &[(&str, &Path)]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_sw"));
    command.args(args).current_dir(dir);
    for (key, value) in envs {
        command.env(key, value);
    }
    command.output().expect("failed to execute sw")
}

fn run_in_dir_clearing_env(args: &[&str], dir: &Path, keys: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_sw"));
    command.args(args).current_dir(dir);
    for key in keys {
        command.env_remove(key);
    }
    command.output().expect("failed to execute sw")
}

fn write_runbook(dir: &Path, fixture_name: &str, target_name: &str) -> PathBuf {
    let source = Path::new("tests/fixtures").join(fixture_name);
    let target = dir.join(target_name);
    let contents = fs::read_to_string(source).expect("failed to read fixture");
    fs::write(&target, contents).expect("failed to write fixture");
    target
}

fn create_fake_java_home(dir: &Path, folder_name: &str, version: &str) -> PathBuf {
    let java_home = dir.join(folder_name);
    let bin_dir = java_home.join("bin");
    fs::create_dir_all(&bin_dir).expect("failed to create fake java bin dir");
    let java_path = bin_dir.join("java");
    fs::write(
        &java_path,
        format!("#!/bin/sh\necho 'openjdk version \"{version}.0.1\"' >&2\necho 'Fake Java' >&2\n"),
    )
    .expect("failed to write fake java");
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(&java_path)
            .expect("missing fake java metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&java_path, permissions).expect("failed to make fake java executable");
    }
    java_home
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
fn check_accepts_yaml_input_file() {
    let dir = prepare_workspace();
    write_runbook(
        &dir,
        "sw-runbook-run-prerequisites-success.yaml",
        "example.yaml",
    );

    let output = run_in_dir(&["check", "--input-file", "example.yaml"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("All prerequisite checks passed"));
    assert_eq!(
        fs::read_to_string(dir.join("prereq-order.txt")).expect("missing prereq-order.txt"),
        "prereq\n"
    );
}

#[test]
fn check_accepts_json_runbook_from_stdin() {
    let dir = prepare_workspace();
    let stdin = fs::read_to_string("tests/fixtures/sw-runbook-run-prerequisites-success.json")
        .expect("failed to read fixture");

    let output = run_in_dir_with_stdin(&["check", "--input-file=-"], &dir, &stdin);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("All prerequisite checks passed"));
    assert_eq!(
        fs::read_to_string(dir.join("prereq-order.txt")).expect("missing prereq-order.txt"),
        "prereq\n"
    );
}

#[test]
fn check_accepts_yaml_runbook_from_stdin_with_explicit_format() {
    let dir = prepare_workspace();
    let stdin = fs::read_to_string("tests/fixtures/sw-runbook-run-prerequisites-success.yaml")
        .expect("failed to read fixture");

    let output = run_in_dir_with_stdin(
        &["check", "--input-file=-", "--input-format=yaml"],
        &dir,
        &stdin,
    );

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("All prerequisite checks passed"));
    assert_eq!(
        fs::read_to_string(dir.join("prereq-order.txt")).expect("missing prereq-order.txt"),
        "prereq\n"
    );
}

#[test]
fn check_rejects_yaml_runbook_from_stdin_without_explicit_format() {
    let dir = prepare_workspace();
    let stdin = fs::read_to_string("tests/fixtures/sw-runbook-run-prerequisites-success.yaml")
        .expect("failed to read fixture");

    let output = run_in_dir_with_stdin(&["check", "--input-file=-"], &dir, &stdin);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid JSON in stdin"));
    assert!(!dir.join("prereq-order.txt").exists());
}

#[test]
fn check_input_format_without_stdin_keeps_default_file_lookup() {
    let dir = prepare_workspace();
    write_runbook(
        &dir,
        "sw-runbook-run-prerequisites-success.json",
        "sw-runbook.json",
    );

    let output = run_in_dir(&["check", "--input-format=yaml"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("All prerequisite checks passed"));
}

#[test]
fn check_uses_yaml_default_when_json_is_missing() {
    let dir = prepare_workspace();
    write_runbook(
        &dir,
        "sw-runbook-run-prerequisites-success.yaml",
        "sw-runbook.yaml",
    );

    let output = run_in_dir(&["check"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("All prerequisite checks passed"));
}

#[test]
fn check_succeeds_when_java_prerequisites_pass() {
    let dir = prepare_workspace();
    let java_17_home = create_fake_java_home(&dir, "jdk-17", "17");
    let java_24_home = create_fake_java_home(&dir, "jdk-24", "24");
    write_runbook(
        &dir,
        "sw-runbook-run-prerequisites-java-success.json",
        "sw-runbook.json",
    );

    let output = run_in_dir_with_env(
        &["check"],
        &dir,
        &[
            ("JAVA_17_HOME", &java_17_home),
            ("JAVA_24_HOME", &java_24_home),
        ],
    );

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("All prerequisite checks passed"));
    assert!(!dir.join("prereq-java-order.txt").exists());
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
fn check_fails_when_java_home_env_is_unset() {
    let dir = prepare_workspace();
    write_runbook(
        &dir,
        "sw-runbook-run-prerequisites-java-env-missing.json",
        "sw-runbook.json",
    );

    let output = run_in_dir_clearing_env(&["check"], &dir, &["JAVA_17_HOME"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(!dir.join("prereq-java-missing-order.txt").exists());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("environment variable `JAVA_17_HOME` is not set"));
    assert!(stderr.contains("Set `JAVA_17_HOME` to a Java 17 home directory."));
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
