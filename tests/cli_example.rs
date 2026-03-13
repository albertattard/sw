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
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "Command");
    assert!(value["commands"].is_array());
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
}

#[test]
fn keep_between_example_prints_valid_json_fragment() {
    let output = run(&["example", "rewrite.keep_between"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "keep_between");
    assert_eq!(value["show_trim_markers"], true);
}

#[test]
fn replace_example_prints_valid_json_fragment() {
    let output = run(&["example", "rewrite.replace"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "replace");
    assert_eq!(value["replacement"], ".");
}

#[test]
fn datetime_shift_example_prints_valid_json_fragment() {
    let output = run(&["example", "rewrite.datetime_shift"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "datetime_shift");
    assert_eq!(value["format"], "rfc3339");
}

#[test]
fn capture_oriented_rewrite_example_prints_valid_json_fragment() {
    let output = run(&["example", "rewrite.capture_replace"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "replace");
    assert_eq!(value["pattern"], "@{audio_path_1_original}");
    assert_eq!(value["replacement"], "@{audio_path_1_rewritten}");
}

#[test]
fn unknown_example_topic_returns_operational_error() {
    let output = run(&["example", "unknown.topic"]);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown example topic"));
}
