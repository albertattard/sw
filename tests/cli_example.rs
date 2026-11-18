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
    assert_eq!(value["working_directory"], "demo");
    assert!(value.get("working_dir").is_none());
    assert_eq!(value["execute_when"]["fact"], "os");
    assert_eq!(value["execute_when"]["equals"], "linux");
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
    assert_eq!(value["working_directory"], "demo");
    assert!(value.get("working_dir").is_none());
    assert_eq!(value["execute_when"]["fact"], "os");
    assert_eq!(value["execute_when"]["equals"], "linux");
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
fn breakpoint_example_prints_valid_yaml_entry() {
    let output = run(&["example", "Breakpoint"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.trim_start().starts_with("type: Breakpoint\n"));
    let value: serde_json::Value =
        serde_norway::from_str(&stdout).expect("example output should be valid yaml");
    assert_eq!(value["type"], "Breakpoint");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be a string")
            .contains("debugging")
    );
}

#[test]
fn breakpoint_example_prints_valid_json_entry_when_requested() {
    let output = run(&["example", "Breakpoint", "--output-format", "json"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout
            .trim_start()
            .starts_with("{\n  \"type\": \"Breakpoint\"")
    );
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "Breakpoint");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be a string")
            .contains("debugging")
    );
}

#[test]
fn change_directory_example_prints_valid_yaml_entry() {
    let output = run(&["example", "ChangeDirectory"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_norway::from_str(&stdout).expect("example output should be valid yaml");
    assert_eq!(value["type"], "ChangeDirectory");
    assert_eq!(value["path"], "demo-supermarket");
    assert!(value["contents"].as_str().is_some());
}

#[test]
fn display_file_example_prints_valid_yaml_entry() {
    let output = run(&["example", "DisplayFile"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_norway::from_str(&stdout).expect("example output should be valid yaml");
    assert_eq!(value["type"], "DisplayFile");
    assert_eq!(value["content_type"], "java");
    assert_eq!(value["start_line"], 24);
    assert_eq!(value["line_count"], 12);
    assert_eq!(value["show_trim_markers"], false);
    assert_eq!(value["indent"], 3);
    assert_eq!(value["offset"], -12);
    assert_eq!(value["transform"]["language"], "java");
    assert_eq!(
        value["transform"]["operations"][0]["type"],
        "collapse_method_body"
    );
    assert_eq!(value["transform"]["operations"][0]["name"], "initialize");
}

#[test]
fn display_file_example_prints_valid_json_entry_when_requested() {
    let output = run(&["example", "DisplayFile", "--output-format", "json"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("example output should be valid json");
    assert_eq!(value["type"], "DisplayFile");
    assert_eq!(value["content_type"], "java");
    assert_eq!(value["start_line"], 24);
    assert_eq!(value["line_count"], 12);
    assert_eq!(value["show_trim_markers"], false);
    assert_eq!(value["indent"], 3);
    assert_eq!(value["offset"], -12);
    assert_eq!(value["transform"]["language"], "java");
    assert_eq!(
        value["transform"]["operations"][0]["type"],
        "collapse_method_body"
    );
    assert_eq!(value["transform"]["operations"][0]["name"], "initialize");
}

#[test]
fn display_url_example_prints_valid_yaml_entry() {
    let output = run(&["example", "DisplayUrl"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_norway::from_str(&stdout).expect("example output should be valid yaml");
    assert_eq!(value["type"], "DisplayUrl");
    assert_eq!(
        value["url"],
        "https://raw.githubusercontent.com/albertattard/sw/refs/heads/main/docs/guides/entities.md"
    );
    assert_eq!(value["content_type"], "markdown");
    assert_eq!(value["timeout"], "10 seconds");
    assert!(value.get("start_line").is_none());
    assert_eq!(value["line_count"], 40);
    assert_eq!(value["indent"], 3);
}

#[test]
fn patch_example_prints_valid_yaml_entry() {
    let output = run(&["example", "Patch"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_norway::from_str(&stdout).expect("example output should be valid yaml");
    assert_eq!(value["type"], "Patch");
    assert_eq!(value["debug"], true);
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
    assert_eq!(value["debug"], true);
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

#[test]
fn secrets_example_formats_describe_equivalent_complete_runbooks() {
    let yaml = run(&["example", "Secrets"]);
    let explicit_yaml = run(&["example", "Secrets", "--output-format", "yaml"]);
    let mixed_case = run(&["example", "sEcReTs"]);
    let json = run(&["example", "secrets", "--output-format", "json"]);
    for output in [&yaml, &explicit_yaml, &mixed_case, &json] {
        assert!(output.status.success());
    }
    assert_eq!(yaml.stdout, explicit_yaml.stdout);
    assert_eq!(yaml.stdout, mixed_case.stdout);
    let yaml_value: serde_json::Value = serde_norway::from_slice(&yaml.stdout).unwrap();
    let json_value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert_eq!(yaml_value, json_value);
    assert_eq!(yaml_value["secrets"].as_object().unwrap().len(), 2);
    assert_eq!(
        yaml_value["secrets"]["session_token"]["generate"],
        serde_json::json!({})
    );
    assert_eq!(
        yaml_value["secrets"]["signing_key"]["generate"]["length"],
        48
    );
    let entries = yaml_value["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 2);
    for entry in entries {
        assert_eq!(entry["type"], "Command");
        assert_eq!(
            entry["environment"]["SESSION_TOKEN"]["secret"],
            "session_token"
        );
        assert!(
            entry["commands"]
                .as_str()
                .unwrap()
                .contains("${#SESSION_TOKEN}")
        );
        assert_eq!(entry["assert"]["exit_code"], 0);
    }
    assert_eq!(
        entries[0]["environment"]["SIGNING_KEY"]["secret"],
        "signing_key"
    );
    assert!(
        entries[0]["commands"]
            .as_str()
            .unwrap()
            .contains("${#SIGNING_KEY}")
    );
}

#[test]
fn secrets_examples_validate_and_run_without_exposing_values_or_leaving_command_files() {
    struct Workspace(std::path::PathBuf);
    impl Drop for Workspace {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    for format in ["yaml", "json"] {
        let workspace = Workspace(std::env::temp_dir().join(format!(
            "sw-example-secrets-{}-{format}",
            std::process::id()
        )));
        std::fs::create_dir(&workspace.0).unwrap();
        let example = run(&["example", "Secrets", "--output-format", format]);
        assert!(example.status.success());
        let input_name = format!("secrets.{format}");
        std::fs::write(workspace.0.join(&input_name), &example.stdout).unwrap();
        let invoke = |args: &[&str]| {
            Command::new(env!("CARGO_BIN_EXE_sw"))
                .args(args)
                .current_dir(&workspace.0)
                .env("SESSION_TOKEN", "inherited")
                .env("SIGNING_KEY", "inherited")
                .output()
                .unwrap()
        };
        let validation = invoke(&["validate", "--input-file", &input_name]);
        assert!(
            validation.status.success(),
            "{}",
            String::from_utf8_lossy(&validation.stderr)
        );
        let normal = invoke(&["run", "--input-file", &input_name]);
        assert!(
            normal.status.success(),
            "{}",
            String::from_utf8_lossy(&normal.stderr)
        );
        assert!(!String::from_utf8_lossy(&normal.stderr).contains("Generated secrets:"));
        let markdown = std::fs::read_to_string(workspace.0.join("README.md")).unwrap();
        for message in [
            "Secret bindings are ready.",
            "Shared secret binding is ready.",
        ] {
            assert!(markdown.contains(message));
        }

        // Obtain this invocation's values only through the intentional reveal list.
        let revealed = invoke(&[
            "run",
            "--input-file",
            &input_name,
            "--reveal-secrets",
            "--verbose",
            "--debug",
        ]);
        assert!(revealed.status.success());
        let stderr = String::from_utf8_lossy(&revealed.stderr);
        let mut lines = stderr.lines();
        assert_eq!(lines.next(), Some("Generated secrets:"));
        let mut values = Vec::new();
        for (name, length) in [("session_token", 32), ("signing_key", 48)] {
            let line = lines.next().unwrap();
            let value = line.strip_prefix(&format!("  {name}: ")).unwrap();
            assert_eq!(value.len(), length);
            assert!(value.bytes().all(|c| c.is_ascii_alphanumeric()));
            values.push(value);
        }
        let remaining_stderr = lines.collect::<Vec<_>>().join("\n");
        let stdout = String::from_utf8_lossy(&revealed.stdout);
        let markdown = std::fs::read_to_string(workspace.0.join("README.md")).unwrap();
        for value in values {
            assert!(!stdout.contains(value));
            assert!(!remaining_stderr.contains(value));
            assert!(!markdown.contains(value));
        }
        let mut files: Vec<_> = std::fs::read_dir(&workspace.0)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();
        files.sort();
        assert_eq!(files, ["README.md".to_string(), input_name]);
    }
}
