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
        "sw-convert-test-{}-{nanos}-{id}",
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
fn convert_auto_detects_json_default_and_writes_yaml() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "sw-runbook.json",
        "{ \"entries\": [ { \"type\": \"Heading\", \"level\": \"H1\", \"title\": \"Example\" } ] }",
    );

    let output = run_in_dir(&["convert"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Converted sw-runbook.json to sw-runbook.yaml"));

    let converted = fs::read_to_string(dir.join("sw-runbook.yaml")).expect("missing yaml output");
    let value: serde_json::Value =
        serde_norway::from_str(&converted).expect("converted output should be valid yaml");
    assert_eq!(value["entries"][0]["title"], "Example");
    assert!(converted.starts_with("entries:\n  - type: Heading\n"));
}

#[test]
fn convert_auto_detects_yaml_default_and_writes_json() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "sw-runbook.yaml",
        "entries:\n  - type: Heading\n    level: H1\n    title: Example\n",
    );

    let output = run_in_dir(&["convert"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Converted sw-runbook.yaml to sw-runbook.json"));

    let converted = fs::read_to_string(dir.join("sw-runbook.json")).expect("missing json output");
    let value: serde_json::Value =
        serde_json::from_str(&converted).expect("converted output should be valid json");
    assert_eq!(value["entries"][0]["title"], "Example");
    assert!(converted.starts_with("{\n  \"entries\": [\n"));
}

#[test]
fn convert_uses_yml_input_and_writes_json() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "sw-runbook.yml",
        "entries:\n  - type: Heading\n    level: H1\n    title: Example\n",
    );

    let output = run_in_dir(&["convert"], &dir);

    assert!(output.status.success());
    assert!(dir.join("sw-runbook.json").exists());
}

#[test]
fn convert_derives_output_path_from_explicit_json_input() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "example.json",
        "{ \"entries\": [ { \"type\": \"Heading\", \"level\": \"H1\", \"title\": \"Example\" } ] }",
    );

    let output = run_in_dir(&["convert", "--input-file", "example.json"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Converted example.json to example.yaml"));
    assert!(dir.join("example.yaml").exists());
}

#[test]
fn convert_derives_output_path_from_explicit_yaml_input() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "example.yaml",
        "entries:\n  - type: Heading\n    level: H1\n    title: Example\n",
    );

    let output = run_in_dir(&["convert", "--input-file", "example.yaml"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Converted example.yaml to example.json"));
    assert!(dir.join("example.json").exists());
}

#[test]
fn convert_rejects_ambiguous_default_inputs() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "sw-runbook.json",
        "{ \"entries\": [ { \"type\": \"Heading\", \"level\": \"H1\", \"title\": \"Json\" } ] }",
    );
    write_file(
        &dir,
        "sw-runbook.yaml",
        "entries:\n  - type: Heading\n    level: H1\n    title: Yaml\n",
    );

    let output = run_in_dir(&["convert"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Multiple default runbooks found"));
    assert!(stderr.contains("Specify --input-file explicitly"));
}

#[test]
fn convert_fails_when_no_default_input_exists() {
    let dir = prepare_workspace();

    let output = run_in_dir(&["convert"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No default runbook found"));
}

#[test]
fn convert_rejects_stdin_selection() {
    let dir = prepare_workspace();

    let output = run_in_dir(&["convert", "--input-file=-"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("does not accept --input-file=-"));
}

#[test]
fn convert_rejects_same_format_conversion() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "example.json",
        "{ \"entries\": [ { \"type\": \"Heading\", \"level\": \"H1\", \"title\": \"Example\" } ] }",
    );

    let output = run_in_dir(
        &[
            "convert",
            "--input-file",
            "example.json",
            "--output-format",
            "json",
        ],
        &dir,
    );

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("requires the opposite format"));
}

#[test]
fn convert_rejects_same_input_and_output_path() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "example.json",
        "{ \"entries\": [ { \"type\": \"Heading\", \"level\": \"H1\", \"title\": \"Example\" } ] }",
    );

    let output = run_in_dir(
        &[
            "convert",
            "--input-file",
            "example.json",
            "--output-file",
            "./example.json",
        ],
        &dir,
    );

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("does not support in-place conversion"));
}

#[test]
fn convert_rejects_mismatched_output_format_and_extension() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "example.json",
        "{ \"entries\": [ { \"type\": \"Heading\", \"level\": \"H1\", \"title\": \"Example\" } ] }",
    );

    let output = run_in_dir(
        &[
            "convert",
            "--input-file",
            "example.json",
            "--output-file",
            "converted.yaml",
            "--output-format",
            "json",
        ],
        &dir,
    );

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Output file extension does not match --output-format"));
}

#[test]
fn convert_refuses_to_overwrite_existing_output_without_force() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "example.json",
        "{ \"entries\": [ { \"type\": \"Heading\", \"level\": \"H1\", \"title\": \"Example\" } ] }",
    );
    write_file(&dir, "converted.yaml", "existing");

    let output = run_in_dir(
        &[
            "convert",
            "--input-file",
            "example.json",
            "--output-file",
            "converted.yaml",
        ],
        &dir,
    );

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Refusing to overwrite existing output file"));
    assert_eq!(
        fs::read_to_string(dir.join("converted.yaml")).expect("missing original output"),
        "existing"
    );
}

#[test]
fn convert_overwrites_existing_output_with_force() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "example.json",
        "{ \"entries\": [ { \"type\": \"Heading\", \"level\": \"H1\", \"title\": \"Example\" } ] }",
    );
    write_file(&dir, "converted.yaml", "existing");

    let output = run_in_dir(
        &[
            "convert",
            "--input-file",
            "example.json",
            "--output-file",
            "converted.yaml",
            "--force",
        ],
        &dir,
    );

    assert!(output.status.success());
    let converted = fs::read_to_string(dir.join("converted.yaml")).expect("missing output file");
    assert_ne!(converted, "existing");
    let value: serde_json::Value =
        serde_norway::from_str(&converted).expect("output should be valid yaml");
    assert_eq!(value["entries"][0]["title"], "Example");
}

#[test]
fn convert_invalid_json_does_not_write_output() {
    let dir = prepare_workspace();
    write_file(&dir, "example.json", "{ invalid json }\n");

    let output = run_in_dir(&["convert", "--input-file", "example.json"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid JSON"));
    assert!(!dir.join("example.yaml").exists());
}

#[test]
fn convert_invalid_runbook_does_not_write_output() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "example.json",
        "{ \"entries\": [ { \"level\": \"H1\", \"title\": \"Missing type\" } ] }\n",
    );

    let output = run_in_dir(&["convert", "--input-file", "example.json"], &dir);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Runbook is invalid"));
    assert!(!dir.join("example.yaml").exists());
}

#[test]
fn convert_json_markdown_contents_array_to_yaml_block_scalar() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "example.json",
        r#"{
  "entries": [
    {
      "type": "Markdown",
      "contents": [
        "First paragraph.",
        "",
        "Second paragraph."
      ]
    }
  ]
}
"#,
    );

    let output = run_in_dir(&["convert", "--input-file", "example.json"], &dir);

    assert!(output.status.success());
    let converted = fs::read_to_string(dir.join("example.yaml")).expect("missing yaml output");
    assert!(converted.contains("contents: |-"));
    assert!(converted.contains("First paragraph.\n\n      Second paragraph."));

    let value: serde_json::Value =
        serde_norway::from_str(&converted).expect("converted output should be valid yaml");
    assert_eq!(
        value["entries"][0]["contents"],
        "First paragraph.\n\nSecond paragraph."
    );
}

#[test]
fn convert_json_command_fields_to_yaml_block_scalars() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "example.json",
        r#"{
  "entries": [
    {
      "type": "Command",
      "commands": [
        "echo first",
        "echo second"
      ],
      "cleanup": [
        "echo cleanup",
        "rm -f temp.txt"
      ]
    }
  ]
}
"#,
    );

    let output = run_in_dir(&["convert", "--input-file", "example.json"], &dir);

    assert!(output.status.success());
    let converted = fs::read_to_string(dir.join("example.yaml")).expect("missing yaml output");
    assert!(converted.contains("commands: |-"));
    assert!(converted.contains("echo first\n      echo second"));
    assert!(converted.contains("cleanup: |-"));
    assert!(converted.contains("echo cleanup\n      rm -f temp.txt"));

    let value: serde_json::Value =
        serde_norway::from_str(&converted).expect("converted output should be valid yaml");
    assert_eq!(value["entries"][0]["commands"], "echo first\necho second");
    assert_eq!(
        value["entries"][0]["cleanup"],
        "echo cleanup\nrm -f temp.txt"
    );
}

#[test]
fn convert_json_prerequisite_fields_to_yaml_block_scalars_and_validate() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "example.json",
        r#"{
  "entries": [
    {
      "type": "Prerequisite",
      "checks": [
        {
          "kind": "command",
          "name": "Tooling",
          "contents": [
            "- Install tooling",
            "- Confirm setup"
          ],
          "commands": [
            "echo ready",
            "exit 0"
          ],
          "assert": {
            "exit_code": 0
          },
          "help": "Install the required tooling."
        }
      ]
    }
  ]
}
"#,
    );

    let output = run_in_dir(&["convert", "--input-file", "example.json"], &dir);

    assert!(output.status.success());
    let converted = fs::read_to_string(dir.join("example.yaml")).expect("missing yaml output");
    assert!(converted.contains("contents: |-"));
    assert!(converted.contains("- Install tooling\n          - Confirm setup"));
    assert!(converted.contains("commands: |-"));
    assert!(converted.contains("echo ready\n          exit 0"));

    let validate_output = run_in_dir(&["validate", "--input-file", "example.yaml"], &dir);
    assert!(validate_output.status.success());
}

#[test]
fn convert_keeps_non_scalar_capable_string_arrays_as_yaml_sequences() {
    let dir = prepare_workspace();
    write_file(
        &dir,
        "example.json",
        r#"{
  "entries": [
    {
      "type": "Patch",
      "path": "README.md",
      "patch": [
        "@@ -1 +1 @@",
        "-before",
        "+after"
      ]
    }
  ]
}
"#,
    );

    let output = run_in_dir(&["convert", "--input-file", "example.json"], &dir);

    assert!(output.status.success());
    let converted = fs::read_to_string(dir.join("example.yaml")).expect("missing yaml output");
    assert!(converted.contains("patch:\n      - '@@ -1 +1 @@'\n      - -before\n      - +after\n"));

    let value: serde_json::Value =
        serde_norway::from_str(&converted).expect("converted output should be valid yaml");
    assert!(value["entries"][0]["patch"].is_array());
}
