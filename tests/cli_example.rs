use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sw"))
        .args(args)
        .output()
        .expect("failed to execute sw")
}

#[test]
fn command_example_defaults_to_valid_yaml_entry() {
    let output = run(&["example", "Command"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.trim_start().starts_with("type: Command\n"));
    let value: serde_json::Value =
        serde_norway::from_str(&stdout).expect("example output should be valid yaml");
    assert_eq!(value["type"], "Command");
    assert_eq!(value["debug"], true);
    assert!(value["commands"].is_array());
    assert!(value["assert"].is_object());
    assert_eq!(value["output"]["stream"], "combined");
    assert_eq!(value["output"]["trim_empty_lines"], "leading_trailing");
    assert!(value["output"]["rewrite"].is_array());
    assert!(
        value["output"]["rewrite"]
            .as_array()
            .expect("rewrite should be an array")
            .iter()
            .any(|rule| rule["custom_format"] == "%H:%M:%S%.3f")
    );
    assert!(
        value["output"]["rewrite"]
            .as_array()
            .expect("rewrite should be an array")
            .iter()
            .any(|rule| rule["capture_as"] == "started_at")
    );
    assert!(
        value["output"]["rewrite"]
            .as_array()
            .expect("rewrite should be an array")
            .iter()
            .any(|rule| rule["type"] == "limit_lines" && rule["first"] == 12)
    );
    assert_eq!(value["output"]["trim_trailing_whitespace"], true);
    assert!(value["capture"].is_array());
    assert!(value["cleanup"].is_array());
}

#[test]
fn command_example_explicit_yaml_matches_default_output() {
    let default_output = run(&["example", "Command"]);
    let explicit_output = run(&["example", "Command", "--output-format", "yaml"]);

    assert!(default_output.status.success());
    assert!(explicit_output.status.success());
    assert_eq!(default_output.stdout, explicit_output.stdout);
}

#[test]
fn command_example_prints_valid_json_entry_when_requested() {
    let output = run(&["example", "Command", "--output-format", "json"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout
            .trim_start()
            .starts_with("{\n  \"type\": \"Command\"")
    );
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "Command");
    assert_eq!(value["debug"], true);
    assert!(value["commands"].is_array());
    assert!(value["assert"].is_object());
    assert_eq!(value["output"]["stream"], "combined");
    assert_eq!(value["output"]["trim_empty_lines"], "leading_trailing");
    assert!(value["output"]["rewrite"].is_array());
    assert!(
        value["output"]["rewrite"]
            .as_array()
            .expect("rewrite should be an array")
            .iter()
            .any(|rule| rule["custom_format"] == "%H:%M:%S%.3f")
    );
    assert!(
        value["output"]["rewrite"]
            .as_array()
            .expect("rewrite should be an array")
            .iter()
            .any(|rule| rule["capture_as"] == "started_at")
    );
    assert!(
        value["output"]["rewrite"]
            .as_array()
            .expect("rewrite should be an array")
            .iter()
            .any(|rule| rule["type"] == "limit_lines" && rule["first"] == 12)
    );
    assert_eq!(value["output"]["trim_trailing_whitespace"], true);
    assert!(value["capture"].is_array());
    assert!(value["cleanup"].is_array());
}

#[test]
fn display_file_example_prints_valid_yaml_entry() {
    let output = run(&["example", "DisplayFile"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_norway::from_str(&stdout).expect("example output should be valid yaml");
    assert_eq!(value["type"], "DisplayFile");
    assert_eq!(value["start_line"], 1);
    assert_eq!(value["line_count"], 12);
    assert_eq!(value["indent"], 3);
    assert_eq!(value["transform"]["language"], "java");
    assert_eq!(
        value["transform"]["operations"][0]["type"],
        "collapse_method_body"
    );
    assert_eq!(value["transform"]["operations"][0]["name"], "initialize");
}

#[test]
fn patch_example_prints_valid_yaml_entry() {
    let output = run(&["example", "Patch"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_norway::from_str(&stdout).expect("example output should be valid yaml");
    assert_eq!(value["type"], "Patch");
    assert_eq!(value["path"], "./src/main/java/demo/Main.java");
    assert!(stdout.contains("patch: |\n"));
    assert_eq!(
        value["patch"],
        "@@ -10,3 +10,3 @@\n-        return oldValue;\n+        return newValue;\n"
    );
    assert!(value.get("restore").is_none());
}

#[test]
fn patch_example_prints_valid_json_entry_when_requested() {
    let output = run(&["example", "Patch", "--output-format", "json"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "Patch");
    assert_eq!(value["path"], "./src/main/java/demo/Main.java");
    assert!(value["patch"].is_array());
    assert_eq!(value["patch"][0], "@@ -10,3 +10,3 @@");
    assert!(value.get("restore").is_none());
}

#[test]
fn prerequisite_example_prints_valid_yaml_entry() {
    let output = run(&["example", "Prerequisite"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_norway::from_str(&stdout).expect("example output should be valid yaml");
    assert_eq!(value["type"], "Prerequisite");
    assert_eq!(value["checks"][0]["kind"], "java");
    assert_eq!(value["checks"][0]["name"], "Java 25+");
    assert_eq!(value["checks"][0]["version"], "25+");
    assert!(value["checks"][0].get("commands").is_none());
    assert!(value["checks"][0].get("assert").is_none());
    assert!(
        value["checks"][0]["help"]
            .as_str()
            .expect("help should be a string")
            .contains("Java 25 or newer")
    );
}

#[test]
fn prerequisite_example_prints_valid_json_entry_when_requested() {
    let output = run(&["example", "Prerequisite", "--output-format", "json"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "Prerequisite");
    assert_eq!(value["checks"][0]["kind"], "java");
    assert_eq!(value["checks"][0]["name"], "Java 25+");
    assert_eq!(value["checks"][0]["version"], "25+");
    assert!(value["checks"][0].get("commands").is_none());
    assert!(value["checks"][0].get("assert").is_none());
}

#[test]
fn unknown_example_topic_returns_operational_error() {
    let output = run(&["example", "unknown.topic"]);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown example topic"));
}

#[test]
fn command_example_topic_is_case_insensitive() {
    let output = run(&["example", "command"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_norway::from_str(&stdout).expect("example output should be valid yaml");
    assert_eq!(value["type"], "Command");
}

#[test]
fn patch_example_topic_is_case_insensitive() {
    let output = run(&["example", "patch"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_norway::from_str(&stdout).expect("example output should be valid yaml");
    assert_eq!(value["type"], "Patch");
}

#[test]
fn unknown_example_output_format_returns_operational_error() {
    let output = run(&["example", "Command", "--output-format", "toml"]);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown example output format"));
}

#[test]
fn nested_rewrite_topic_returns_operational_error() {
    let output = run(&["example", "rewrite.replace"]);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown example topic"));
}
