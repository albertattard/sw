use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn unique_temp_dir() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "sw-import-test-{}-{nanos}-{id}",
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

fn write_fixture(dir: &Path, fixture_name: &str, target_name: &str) -> PathBuf {
    let source = Path::new("tests/fixtures").join(fixture_name);
    let target = dir.join(target_name);
    let contents = fs::read_to_string(source).expect("failed to read fixture");
    fs::write(&target, contents).expect("failed to write fixture");
    target
}

fn markdown_contents_include(entry: &Value, expected: &str) -> bool {
    match entry.get("contents") {
        Some(Value::Array(lines)) => lines.iter().any(|line| line == expected),
        Some(Value::String(text)) => text.lines().any(|line| line == expected),
        _ => false,
    }
}

#[test]
fn import_defaults_to_readme_input_and_runbook_output() {
    let dir = prepare_workspace();
    write_fixture(&dir, "readme-import-sample.md", "README.md");

    let output = run_in_dir(&["import"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Imported README.md to sw-runbook.yaml"));
    assert!(dir.join("sw-runbook.yaml").exists());

    let validate_output = run_in_dir(&["validate", "--output-format=json"], &dir);
    assert!(validate_output.status.success());
    let validate_stdout = String::from_utf8_lossy(&validate_output.stdout);
    assert!(validate_stdout.contains("\"valid\": true"));
}

#[test]
fn import_infers_yaml_from_explicit_output_path() {
    let dir = prepare_workspace();
    write_fixture(&dir, "readme-import-sample.md", "guide.md");

    let output = run_in_dir(
        &[
            "import",
            "--input-file",
            "guide.md",
            "--output-file",
            "generated-runbook.yaml",
        ],
        &dir,
    );

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Imported guide.md to generated-runbook.yaml"));
    assert!(dir.join("generated-runbook.yaml").exists());
    let runbook =
        fs::read_to_string(dir.join("generated-runbook.yaml")).expect("missing imported file");
    let value: serde_json::Value =
        serde_norway::from_str(&runbook).expect("import output should be valid yaml");
    assert!(value["entries"].is_array());
}

#[test]
fn import_infers_json_from_explicit_output_path() {
    let dir = prepare_workspace();
    write_fixture(&dir, "readme-import-sample.md", "guide.md");

    let output = run_in_dir(
        &[
            "import",
            "--input-file",
            "guide.md",
            "--output-file",
            "generated-runbook.json",
        ],
        &dir,
    );

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Imported guide.md to generated-runbook.json"));
    let runbook =
        fs::read_to_string(dir.join("generated-runbook.json")).expect("missing imported file");
    let value: serde_json::Value =
        serde_json::from_str(&runbook).expect("import output should be valid json");
    assert!(value["entries"].is_array());
}

#[test]
fn import_output_format_json_changes_default_output_path() {
    let dir = prepare_workspace();
    write_fixture(&dir, "readme-import-sample.md", "README.md");

    let output = run_in_dir(&["import", "--output-format", "json"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Imported README.md to sw-runbook.json"));
    assert!(dir.join("sw-runbook.json").exists());
}

#[test]
fn import_json_places_type_first_in_serialized_output() {
    let dir = prepare_workspace();
    write_fixture(&dir, "readme-import-sample.md", "README.md");

    let output = run_in_dir(&["import", "--output-format", "json"], &dir);

    assert!(output.status.success());
    let runbook = fs::read_to_string(dir.join("sw-runbook.json")).expect("missing imported file");
    let type_pos = runbook
        .find("\"type\": \"Heading\"")
        .expect("json output should contain heading type");
    let level_pos = runbook
        .find("\"level\": \"H1\"")
        .expect("json output should contain heading level");
    assert!(type_pos < level_pos);
}

#[test]
fn import_rejects_mismatched_output_format_and_extension() {
    let dir = prepare_workspace();
    write_fixture(&dir, "readme-import-sample.md", "README.md");

    let output = run_in_dir(
        &[
            "import",
            "--output-format",
            "json",
            "--output-file",
            "generated-runbook.yaml",
        ],
        &dir,
    );

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Output file extension does not match --output-format"));
    assert!(!dir.join("generated-runbook.yaml").exists());
}

#[test]
fn import_refuses_to_overwrite_existing_output_without_force() {
    let dir = prepare_workspace();
    write_fixture(&dir, "readme-import-sample.md", "README.md");
    let output_path = dir.join("sw-runbook.yaml");
    fs::write(&output_path, "sentinel\n").expect("failed to seed output file");

    let output = run_in_dir(&["import"], &dir);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Refusing to overwrite existing output file"));
    assert_eq!(
        fs::read_to_string(&output_path).expect("missing seeded output"),
        "sentinel\n"
    );
}

#[test]
fn import_force_overwrites_existing_output() {
    let dir = prepare_workspace();
    write_fixture(&dir, "readme-import-sample.md", "README.md");
    let output_path = dir.join("sw-runbook.yaml");
    fs::write(&output_path, "sentinel\n").expect("failed to seed output file");

    let output = run_in_dir(&["import", "--force"], &dir);

    assert!(output.status.success());
    let runbook = fs::read_to_string(&output_path).expect("missing imported runbook");
    assert!(runbook.contains("entries:"));
    assert!(!runbook.contains("sentinel"));
}

#[test]
fn import_maps_headings_prose_and_shell_blocks_into_runbook_entries() {
    let dir = prepare_workspace();
    write_fixture(&dir, "readme-import-sample.md", "README.md");

    let output = run_in_dir(&["import"], &dir);
    assert!(output.status.success());

    let runbook = fs::read_to_string(dir.join("sw-runbook.yaml")).expect("missing imported file");
    let value: serde_json::Value =
        serde_norway::from_str(&runbook).expect("import output should be valid yaml");
    let entries = value["entries"]
        .as_array()
        .expect("entries should be an array");

    assert_eq!(entries[0]["type"], "Heading");
    assert_eq!(entries[0]["level"], "H1");
    assert_eq!(entries[0]["title"], "Example Workflow");
    assert!(entries.iter().any(|entry| {
        entry["type"] == "Markdown"
            && markdown_contents_include(entry, "This README shows the main steps.")
    }));
    assert!(entries.iter().any(|entry| {
        entry["type"] == "Command"
            && entry["commands"]
                .as_array()
                .expect("commands should be an array")
                .iter()
                .any(|line| line == "./mvnw clean verify")
    }));
    assert!(entries.iter().any(|entry| {
        entry["type"] == "Markdown" && markdown_contents_include(entry, "```yaml")
    }));
}

#[test]
fn import_yaml_places_type_first_and_separates_entries() {
    let dir = prepare_workspace();
    write_fixture(&dir, "readme-import-sample.md", "README.md");

    let output = run_in_dir(&["import"], &dir);

    assert!(output.status.success());
    let runbook = fs::read_to_string(dir.join("sw-runbook.yaml")).expect("missing imported file");
    assert!(
        runbook.starts_with(
            "entries:\n  - type: Heading\n    level: H1\n    title: Example Workflow\n"
        )
    );
    assert!(runbook.contains("title: Example Workflow\n\n  - type: Markdown\n"));
}

#[test]
fn import_yaml_uses_block_scalars_for_multiline_markdown_contents() {
    let dir = prepare_workspace();
    write_fixture(&dir, "readme-import-sample.md", "README.md");

    let output = run_in_dir(&["import"], &dir);

    assert!(output.status.success());
    let runbook = fs::read_to_string(dir.join("sw-runbook.yaml")).expect("missing imported file");
    assert!(runbook.contains(
        "  - type: Markdown\n    contents: |\n      ```yaml\n      kind: example\n      ```\n"
    ));
}
