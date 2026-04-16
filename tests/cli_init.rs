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

#[test]
fn init_writes_default_runbook_file() {
    let dir = prepare_workspace();

    let output = run_in_dir(&["init"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created starter runbook at sw-runbook.yaml"));
    assert!(dir.join("sw-runbook.yaml").exists());
}

#[test]
fn init_writes_requested_yaml_output_file() {
    let dir = prepare_workspace();

    let output = run_in_dir(&["init", "--output-file", "starter.yaml"], &dir);

    assert!(output.status.success());
    let contents =
        fs::read_to_string(dir.join("starter.yaml")).expect("missing generated yaml file");
    assert!(contents.starts_with("entries:\n  - type: Heading\n"));
    assert!(contents.contains("\n\n  - type: Markdown\n"));
    assert!(contents.contains("checks:\n      - kind: command\n"));
    assert!(contents.contains("commands:\n      - "));
    assert!(contents.contains("caption:\n        - Observed output\n"));
}

#[test]
fn init_refuses_to_overwrite_without_force() {
    let dir = prepare_workspace();
    fs::write(dir.join("sw-runbook.yaml"), "existing").expect("failed to write existing file");

    let output = run_in_dir(&["init"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Refusing to overwrite existing file"));
    assert_eq!(
        fs::read_to_string(dir.join("sw-runbook.yaml")).expect("missing original file"),
        "existing"
    );
}

#[test]
fn init_overwrites_with_force() {
    let dir = prepare_workspace();
    fs::write(dir.join("sw-runbook.yaml"), "existing").expect("failed to write existing file");

    let output = run_in_dir(&["init", "--force"], &dir);

    assert!(output.status.success());
    let contents = fs::read_to_string(dir.join("sw-runbook.yaml")).expect("missing generated file");
    assert!(contents.contains("type: Heading"));
    assert!(contents.contains("type: DisplayFile"));
    assert_ne!(contents, "existing");
}

#[test]
fn init_generated_yaml_sample_passes_validate() {
    let dir = prepare_workspace();

    let init_output = run_in_dir(&["init"], &dir);
    assert!(init_output.status.success());

    let validate_output = run_in_dir(
        &[
            "validate",
            "--input-file",
            "sw-runbook.yaml",
            "--output-format",
            "json",
        ],
        &dir,
    );

    assert!(validate_output.status.success());
    let stdout = String::from_utf8_lossy(&validate_output.stdout);
    assert!(stdout.contains("\"valid\": true"));
}

#[test]
fn init_generated_sample_includes_all_supported_entry_types() {
    let dir = prepare_workspace();

    let output = run_in_dir(&["init"], &dir);

    assert!(output.status.success());
    let contents = fs::read_to_string(dir.join("sw-runbook.yaml")).expect("missing generated file");
    assert!(contents.contains("type: Heading"));
    assert!(contents.contains("type: Markdown"));
    assert!(contents.contains("type: DisplayFile"));
    assert!(contents.contains("type: Prerequisite"));
    assert!(contents.contains("type: Command"));
}

#[test]
fn init_writes_json_when_output_file_uses_json_extension() {
    let dir = prepare_workspace();

    let output = run_in_dir(&["init", "--output-file", "starter.json"], &dir);

    assert!(output.status.success());
    let contents =
        fs::read_to_string(dir.join("starter.json")).expect("missing generated json file");
    assert!(contents.contains("\"type\": \"Heading\""));
    assert!(contents.contains("\"type\": \"Command\""));
}

#[test]
fn init_rejects_unsupported_output_extension() {
    let dir = prepare_workspace();

    let output = run_in_dir(&["init", "--output-file", "starter.txt"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unsupported init output format"));
    assert!(!dir.join("starter.txt").exists());
}
