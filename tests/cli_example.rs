use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sw"))
        .args(args)
        .output()
        .expect("failed to execute sw")
}

#[test]
fn command_example_prints_valid_json_entry() {
    let output = run(&["example", "Command"]);

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
    assert!(value["commands"].is_array());
    assert!(value["assert"].is_object());
    assert!(value["output"]["rewrite"].is_array());
    assert!(
        value["output"]["rewrite"]
            .as_array()
            .expect("rewrite should be an array")
            .iter()
            .any(|rule| rule["custom_format"] == "%H:%M:%S%.3f")
    );
    assert!(value["capture"].is_array());
    assert!(value["cleanup"].is_array());
}

#[test]
fn display_file_example_prints_valid_json_entry() {
    let output = run(&["example", "DisplayFile"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "DisplayFile");
    assert_eq!(value["start_line"], 1);
    assert_eq!(value["line_count"], 12);
    assert_eq!(value["transform"]["language"], "java");
    assert_eq!(
        value["transform"]["operations"][0]["type"],
        "collapse_method_body"
    );
    assert_eq!(value["transform"]["operations"][0]["name"], "initialize");
}

#[test]
fn patch_example_prints_valid_json_entry() {
    let output = run(&["example", "Patch"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "Patch");
    assert_eq!(value["path"], "./src/main/java/demo/Main.java");
    assert!(value["patch"].is_array());
    assert!(value.get("restore").is_none());
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
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "Command");
}

#[test]
fn patch_example_topic_is_case_insensitive() {
    let output = run(&["example", "patch"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "Patch");
}

#[test]
fn nested_rewrite_topic_returns_operational_error() {
    let output = run(&["example", "rewrite.replace"]);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown example topic"));
}
