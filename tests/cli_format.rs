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
    std::env::temp_dir().join(format!(
        "sw-format-test-{}-{nanos}-{id}",
        std::process::id()
    ))
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

fn write_file(dir: &Path, name: &str, contents: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, contents).expect("failed to write test file");
    path
}

#[test]
fn format_rewrites_valid_json_in_place() {
    let dir = prepare_workspace();
    let path = write_file(
        &dir,
        "sw-runbook.json",
        "{ \"entries\" : [ { \"title\" : \"Example\", \"type\" : \"Heading\", \"level\" : \"H1\" } ] }",
    );

    let output = run_in_dir(&["format"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Formatted runbook: sw-runbook.json"));

    let contents = fs::read_to_string(path).expect("missing formatted file");
    assert_eq!(
        contents,
        "{\n  \"entries\": [\n    {\n      \"title\": \"Example\",\n      \"type\": \"Heading\",\n      \"level\": \"H1\"\n    }\n  ]\n}\n"
    );
}

#[test]
fn format_rewrites_valid_yaml_in_place() {
    let dir = prepare_workspace();
    let path = write_file(
        &dir,
        "sw-runbook.yaml",
        "entries: [{title: Example, type: Heading, level: H1}]",
    );

    let output = run_in_dir(&["format"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Formatted runbook: sw-runbook.yaml"));

    let contents = fs::read_to_string(path).expect("missing formatted file");
    let value: serde_json::Value =
        serde_norway::from_str(&contents).expect("formatted yaml should be valid");
    assert_eq!(value["entries"][0]["title"], "Example");
    assert_eq!(value["entries"][0]["type"], "Heading");
    assert_eq!(value["entries"][0]["level"], "H1");
    assert!(contents.ends_with('\n'));
    assert!(!contents.contains("{title: Example"));
}

#[test]
fn format_inserts_blank_lines_between_yaml_entries() {
    let dir = prepare_workspace();
    let path = write_file(
        &dir,
        "sw-runbook.yaml",
        "entries:\n  - {type: Heading, level: H1, title: Order Approval}\n  - {type: Markdown, contents: \"First paragraph.\\n\\nSecond paragraph.\"}\n",
    );

    let output = run_in_dir(&["format"], &dir);

    assert!(output.status.success());

    let contents = fs::read_to_string(path).expect("missing formatted file");
    assert_eq!(
        contents,
        "entries:\n- type: Heading\n  level: H1\n  title: Order Approval\n\n- type: Markdown\n  contents: |-\n    First paragraph.\n\n    Second paragraph.\n"
    );
}

#[test]
fn format_uses_yaml_default_when_json_is_missing() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "sw-runbook.yaml",
        "entries: [{title: Example, type: Heading, level: H1}]",
    );

    let output = run_in_dir(&["format"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Formatted runbook: sw-runbook.yaml"));
}

#[test]
fn format_fails_when_multiple_default_runbooks_exist() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "sw-runbook.json",
        "{ \"entries\": [ { \"type\": \"Heading\", \"level\": \"H1\", \"title\": \"Json\" } ] }",
    );
    write_file(
        &dir,
        "sw-runbook.yaml",
        "entries: [{type: Heading, level: H1, title: Yaml}]",
    );

    let output = run_in_dir(&["format"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Multiple default runbooks found"));
    assert!(stderr.contains("Specify --input-file explicitly"));
}

#[test]
fn format_rejects_stdin_selection() {
    let dir = prepare_workspace();

    let output = run_in_dir(&["format", "--input-file=-"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("does not accept --input-file=-"));
}

#[test]
fn format_invalid_json_does_not_modify_file() {
    let dir = prepare_workspace();
    let original = "{ invalid json }\n";
    let path = write_file(&dir, "sw-runbook.json", original);

    let output = run_in_dir(&["format"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid JSON"));
    assert_eq!(
        fs::read_to_string(path).expect("missing original file"),
        original
    );
}

#[test]
fn format_invalid_yaml_does_not_modify_file() {
    let dir = prepare_workspace();
    let original = "entries:\n  - type: Heading\n    level: H1\n   title: Broken\n";
    let path = write_file(&dir, "sw-runbook.yaml", original);

    let output = run_in_dir(&["format"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid YAML"));
    assert_eq!(
        fs::read_to_string(path).expect("missing original file"),
        original
    );
}

#[test]
fn format_invalid_runbook_does_not_modify_file() {
    let dir = prepare_workspace();
    let original = "{ \"entries\": [ { \"level\": \"H1\", \"title\": \"Missing type\" } ] }\n";
    let path = write_file(&dir, "sw-runbook.json", original);

    let output = run_in_dir(&["format"], &dir);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Runbook is invalid"));
    assert_eq!(
        fs::read_to_string(path).expect("missing original file"),
        original
    );
}
