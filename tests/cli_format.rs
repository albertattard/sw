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
fn format_orders_all_entry_types_in_json_and_yaml() {
    let original: serde_json::Value = serde_json::from_str(
        r#"{"entries":[
          {"z_extra":{"b":2,"a":1},"title":"Example","a_extra":"retained","level":"H1","type":"Heading"},
          {"indent":3,"contents":"Prose","type":"Markdown"},
          {"contents":"Work here","path":"demo","type":"ChangeDirectory"},
          {"indent":3,"offset":0,"show_trim_markers":true,"content_type":"java",
           "transform":{"operations":[{"replacement":"...","name":"main","type":"collapse_method_body"}],"language":"java"},
           "line_count":5,"start_line":2,"path":"Main.java","type":"DisplayFile"},
          {"indent":3,"offset":0,"show_trim_markers":true,"content_type":"text",
           "line_count":5,"start_line":2,"timeout":"10 seconds","url":"https://example.com/demo.txt","type":"DisplayUrl"},
          {"debug":true,"indent":3,"restore":"auto","patch":"@@ -1 +1 @@\n-old\n+new","path":"demo.txt","type":"Patch"},
          {"checks":[{"help":"Install Java","contents":"Java prerequisite","version":"25","name":"Java","kind":"java"}],"type":"Prerequisite"},
          {"message":"Stop","type":"Breakpoint"},
          {"path":"minimal.txt","type":"DisplayFile"}
        ]}"#,
    ).unwrap();
    let expected_orders: &[&[&str]] = &[
        &["type", "level", "title", "z_extra", "a_extra"],
        &["type", "contents", "indent"],
        &["type", "path", "contents"],
        &[
            "type",
            "path",
            "content_type",
            "start_line",
            "line_count",
            "transform",
            "show_trim_markers",
            "offset",
            "indent",
        ],
        &[
            "type",
            "url",
            "timeout",
            "start_line",
            "line_count",
            "content_type",
            "show_trim_markers",
            "offset",
            "indent",
        ],
        &["type", "path", "patch", "restore", "indent", "debug"],
        &["type", "checks"],
        &["type", "message"],
        &["type", "path"],
    ];
    for extension in ["json", "yaml", "yml"] {
        let dir = prepare_workspace();
        let input = if extension == "json" {
            serde_json::to_string(&original).unwrap()
        } else {
            serde_norway::to_string(&original).unwrap()
        };
        let path = write_file(&dir, &format!("sw-runbook.{extension}"), &input);
        let output = run_in_dir(&["format"], &dir);
        assert!(
            output.status.success(),
            "{extension}: {} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let formatted = fs::read_to_string(&path).unwrap();
        let parsed: serde_json::Value = if extension == "json" {
            serde_json::from_str(&formatted).unwrap()
        } else {
            serde_norway::from_str(&formatted).unwrap()
        };
        assert_eq!(parsed, original);
        for (index, expected) in expected_orders.iter().enumerate() {
            let entry = parsed["entries"][index].as_object().unwrap();
            let keys: Vec<_> = entry.keys().map(String::as_str).collect();
            assert_eq!(keys, *expected, "{extension}: entry {index}");
        }
        assert!(run_in_dir(&["format"], &dir).status.success());
        assert_eq!(fs::read_to_string(path).unwrap(), formatted);
        fs::remove_dir_all(dir).unwrap();
    }
}

#[test]
fn format_orders_command_fields_in_json_and_yaml() {
    let original: serde_json::Value = serde_json::from_str(
        r#"{
          "entries": [
            {"title": "Example", "level": "H1", "type": "Heading"},
            {
              "indent": 3,
              "timeout": "10 seconds",
              "cleanup": "echo cleanup",
              "capture": [
                {"pattern": "(hello)", "stage": "raw", "source": "stdout", "name": "greeting"},
                {"pattern": "(second)", "stage": "rewritten", "source": "stdout", "name": "result"}
              ],
              "output": {
                "rewrite": [
                  {"replacement": "first", "pattern": "hello", "type": "replace"},
                  {"replacement": "second", "pattern": "first", "type": "replace"}
                ],
                "stream": "stdout",
                "caption": ["One", "Two"]
              },
              "assert": {
                "checks": [{"contains": "hello", "source": "stdout"}],
                "exit_code": 0
              },
              "commands": "echo hello",
              "working_directory": ".",
              "execute_when": {"equals": "linux", "fact": "os"},
              "debug": true,
              "type": "Command"
            },
            {"commands": "echo legacy", "working_dir": ".", "type": "Command"},
            {"commands": "echo minimal", "type": "Command"}
          ]
        }"#,
    )
    .expect("valid fixture");

    for extension in ["json", "yaml", "yml"] {
        let dir = prepare_workspace();
        let input = if extension == "json" {
            serde_json::to_string(&original).unwrap()
        } else {
            serde_norway::to_string(&original).unwrap()
        };
        let path = write_file(&dir, &format!("sw-runbook.{extension}"), &input);
        let output = run_in_dir(&["format"], &dir);
        assert!(
            output.status.success(),
            "{extension}: {} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let formatted = fs::read_to_string(&path).unwrap();
        let parsed: serde_json::Value = if extension == "json" {
            serde_json::from_str(&formatted).unwrap()
        } else {
            serde_norway::from_str(&formatted).unwrap()
        };
        assert_eq!(parsed, original, "values must remain unchanged");
        let expected_orders: &[&[&str]] = &[
            &["type", "level", "title"],
            &[
                "type",
                "debug",
                "execute_when",
                "working_directory",
                "commands",
                "assert",
                "output",
                "capture",
                "cleanup",
                "timeout",
                "indent",
            ],
            &["type", "working_dir", "commands"],
            &["type", "commands"],
        ];
        for (entry, expected) in parsed["entries"]
            .as_array()
            .unwrap()
            .iter()
            .zip(expected_orders)
        {
            let keys: Vec<_> = entry
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(keys, *expected, "{extension}");
        }
        let second_run = run_in_dir(&["format"], &dir);
        assert!(second_run.status.success());
        assert_eq!(fs::read_to_string(&path).unwrap(), formatted);
        fs::remove_dir_all(dir).unwrap();
    }
}

#[test]
fn format_does_not_reorder_invalid_command_entries() {
    for extension in ["json", "yaml"] {
        let dir = prepare_workspace();
        let original = if extension == "json" {
            r#"{"entries":[{"indent":3,"commands":"echo hello","type":"Command","unknown":true}]}"#
        } else {
            "entries:\n  - indent: 3\n    commands: echo hello\n    type: Command\n    unknown: true\n"
        };
        let path = write_file(&dir, &format!("sw-runbook.{extension}"), original);
        let output = run_in_dir(&["format"], &dir);
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(fs::read_to_string(path).unwrap(), original);
        fs::remove_dir_all(dir).unwrap();
    }
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
        "{\n  \"entries\": [\n    {\n      \"type\": \"Heading\",\n      \"level\": \"H1\",\n      \"title\": \"Example\"\n    }\n  ]\n}\n"
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
fn format_normalizes_numeric_java_versions_to_strings_in_yaml_and_json() {
    let yaml_dir = prepare_workspace();
    let yaml_path = write_file(
        &yaml_dir,
        "sw-runbook.yaml",
        "entries:\n  - type: Prerequisite\n    checks:\n      - kind: java\n        name: Java 25\n        version: 25\n        contents: Java 25 is required.\n",
    );
    let yaml_output = run_in_dir(&["format"], &yaml_dir);
    assert!(yaml_output.status.success());
    let yaml_contents = fs::read_to_string(yaml_path).expect("missing formatted YAML file");
    assert!(yaml_contents.contains("version: '25'"));

    let json_dir = prepare_workspace();
    let json_path = write_file(
        &json_dir,
        "sw-runbook.json",
        r#"{"entries":[{"type":"Prerequisite","checks":[{"kind":"java","name":"Java 25","version":25,"contents":"Java 25 is required."}]}]}"#,
    );
    let json_output = run_in_dir(&["format"], &json_dir);
    assert!(json_output.status.success());
    let json_contents = fs::read_to_string(json_path).expect("missing formatted JSON file");
    assert!(json_contents.contains("\"version\": \"25\""));
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
        "entries:\n  - type: Heading\n    level: H1\n    title: Order Approval\n\n  - type: Markdown\n    contents: |-\n      First paragraph.\n\n      Second paragraph.\n"
    );
}

#[test]
fn format_keeps_change_directory_contents_as_a_literal_block_scalar() {
    let dir = prepare_workspace();
    let path = write_file(
        &dir,
        "sw-runbook.yaml",
        "entries:\n  - {type: ChangeDirectory, path: demo, contents: \"Work in the fork.\\nThen run the application.\"}\n",
    );

    let output = run_in_dir(&["format"], &dir);

    assert!(output.status.success());
    let contents = fs::read_to_string(path).expect("missing formatted file");
    assert!(contents.contains("type: ChangeDirectory"));
    assert!(contents.contains("contents: |-"));
    assert!(contents.contains("Work in the fork."));
}

#[test]
fn format_indents_nested_yaml_sequences_under_keys() {
    let dir = prepare_workspace();
    let path = write_file(
        &dir,
        "sw-runbook.yaml",
        "entries:\n- type: Command\n  commands:\n  - echo hello\n  output:\n    caption:\n    - Observed output\n",
    );

    let output = run_in_dir(&["format"], &dir);

    assert!(output.status.success());

    let contents = fs::read_to_string(path).expect("missing formatted file");
    assert!(contents.contains("entries:\n  - type: Command\n"));
    assert!(contents.contains("commands: echo hello\n"));
    assert!(contents.contains("caption:\n        - Observed output\n"));
}

#[test]
fn format_keeps_prerequisite_contents_as_literal_block_scalar() {
    let dir = prepare_workspace();
    let path = write_file(
        &dir,
        "sw-runbook.yaml",
        r#"entries:
  - type: Prerequisite
    checks:
      - kind: java
        name: Java 8
        version: "8"
        contents: |
          - [Oracle Java 8](https://www.oracle.com/java/technologies/downloads/#java8) is 
            required to run the baseline application before the migration starts.
        help: Download and install Oracle Java 8, then make it the default `java` version in the shell running this runbook.
"#,
    );

    let output = run_in_dir(&["format"], &dir);

    assert!(output.status.success());
    let contents = fs::read_to_string(path).expect("missing formatted file");
    let value: serde_json::Value =
        serde_norway::from_str(&contents).expect("formatted yaml should be valid");
    assert_eq!(
        value["entries"][0]["checks"][0]["contents"],
        "- [Oracle Java 8](https://www.oracle.com/java/technologies/downloads/#java8) is \n  required to run the baseline application before the migration starts.\n"
    );
    assert!(contents.contains("contents: |\n"));
    assert!(contents.contains(
        "          - [Oracle Java 8](https://www.oracle.com/java/technologies/downloads/#java8) is \n"
    ));
    assert!(contents.contains(
        "            required to run the baseline application before the migration starts.\n"
    ));
    assert!(!contents.contains("contents: \"- [Oracle Java 8]"));
}

#[test]
fn format_normalizes_prerequisite_contents_array_to_literal_block_scalar() {
    let dir = prepare_workspace();
    let path = write_file(
        &dir,
        "sw-runbook.yaml",
        "entries:\n  - type: Prerequisite\n    checks:\n      - kind: java\n        name: Java 8\n        version: '8'\n        contents:\n          - Install Java 8.\n          - Export JAVA_HOME.\n",
    );

    let output = run_in_dir(&["format"], &dir);

    assert!(output.status.success());
    let contents = fs::read_to_string(path).expect("missing formatted file");
    let value: serde_json::Value =
        serde_norway::from_str(&contents).expect("formatted yaml should be valid");
    assert_eq!(
        value["entries"][0]["checks"][0]["contents"],
        "Install Java 8.\nExport JAVA_HOME."
    );
    assert!(
        contents.contains("contents: |-\n          Install Java 8.\n          Export JAVA_HOME.\n")
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

#[test]
fn format_orders_nested_schema_objects_and_preserves_user_maps() {
    let original: serde_json::Value = serde_json::from_str(r#"{
      "entries": [
        {"type":"Prerequisite","checks":[
          {"help":"Install Java","contents":"Java needed","java_home":"jdk","distribution":"epp","version":"25","name":"Java","kind":"java"},
          {"contents":"Other Java","java_home_env":"JAVA_HOME","version":"21+","name":"Other","kind":"java"},
          {"help":"Install tool","contents":"Tool needed","timeout":"5 seconds",
           "assert":{"checks":[{"contains":"ok","source":"stdout"}],"exit_code":0},
           "commands":"echo ok","name":"Tool","kind":"command"}
        ]},
        {"type":"Command","commands":"echo 123",
         "execute_when":{"equals":"linux","fact":"os"},
         "environment":{"Z_VAR":{"secret":"z_secret"},"A_VAR":{"secret":"a_secret"}},
         "assert":{"checks":[
           {"contains":"123","source":"stdout"},
           {"exists":true,"path":"demo.txt","source":"file"},
           {"sha256":"0000000000000000000000000000000000000000000000000000000000000000","path":"other.txt","source":"file"}
         ],"exit_code":0},
         "capture":[
           {"parse_as":{"grouping_separator":",","decimal_separator":".","type":"number"},"pattern":"(123)","stage":"raw","source":"stdout","name":"z_number"},
           {"parse_as":{"locale":"en-US","type":"number"},"pattern":"(123)","stage":"rewritten","source":"stdout","name":"a_number"}
         ],
         "output":{"rewrite":[
           {"capture_as":"replaced","replacement":"456","pattern":"123","type":"replace"},
           {"show_trim_markers":false,"end_offset":-1,"start_offset":1,"end":"END","start":"START","type":"keep_between"},
           {"end_pattern":"END.*","start_pattern":"START.*","type":"keep_between"},
           {"show_trim_marker":false,"last":2,"first":3,"type":"limit_lines"},
           {"capture_as":"shifted","custom_format":"%Y-%m-%d","id":"timeline","base":"2020-01-01T00:00:00Z","pattern":"[0-9]{4}-[0-9]{2}-[0-9]{2}","type":"datetime_shift"},
           {"use":"timeline","format":"rfc3339","type":"datetime_shift"}
         ],"trim_trailing_whitespace":false,"trim_empty_lines":"none","content_type":"text","stream":"stdout","caption":"Numbers"}},
        {"type":"DisplayFile","path":"Main.java","transform":{"operations":[
          {"replacement":"...","name":"zMethod","type":"collapse_method_body"},
          {"name":"aMethod","type":"collapse_method_body"}
        ],"language":"java"}},
        {"type":"Heading","title":"Extra fields","level":"H1",
         "z_extra":{"output":{"stream":"stdout","caption":"Leave order"},"entries":[{"commands":"echo extra","type":"Command"}]},
         "a_extra":{"equals":"linux","fact":"os"}},
        {"type":"Command","commands":"echo minimal"}
      ],
      "secrets":{"z_secret":{"generate":{"length":16}},"a_secret":{"generate":{}}}
    }"#).unwrap();
    let expected_orders: &[(&str, &[&str])] = &[
        ("", &["secrets", "entries"]),
        ("/secrets", &["z_secret", "a_secret"]),
        ("/secrets/z_secret", &["generate"]),
        ("/secrets/z_secret/generate", &["length"]),
        ("/secrets/a_secret/generate", &[]),
        (
            "/entries/0/checks/0",
            &[
                "kind",
                "name",
                "version",
                "distribution",
                "java_home",
                "contents",
                "help",
            ],
        ),
        (
            "/entries/0/checks/1",
            &["kind", "name", "version", "java_home_env", "contents"],
        ),
        (
            "/entries/0/checks/2",
            &[
                "kind", "name", "commands", "assert", "timeout", "contents", "help",
            ],
        ),
        ("/entries/0/checks/2/assert", &["exit_code", "checks"]),
        (
            "/entries/0/checks/2/assert/checks/0",
            &["source", "contains"],
        ),
        ("/entries/1/execute_when", &["fact", "equals"]),
        ("/entries/1/environment", &["Z_VAR", "A_VAR"]),
        ("/entries/1/environment/Z_VAR", &["secret"]),
        ("/entries/1/assert", &["exit_code", "checks"]),
        ("/entries/1/assert/checks/0", &["source", "contains"]),
        ("/entries/1/assert/checks/1", &["source", "path", "exists"]),
        ("/entries/1/assert/checks/2", &["source", "path", "sha256"]),
        (
            "/entries/1/capture/0",
            &["name", "source", "stage", "pattern", "parse_as"],
        ),
        (
            "/entries/1/capture/1",
            &["name", "source", "stage", "pattern", "parse_as"],
        ),
        (
            "/entries/1/capture/0/parse_as",
            &["type", "decimal_separator", "grouping_separator"],
        ),
        ("/entries/1/capture/1/parse_as", &["type", "locale"]),
        (
            "/entries/1/output",
            &[
                "caption",
                "stream",
                "content_type",
                "trim_empty_lines",
                "trim_trailing_whitespace",
                "rewrite",
            ],
        ),
        (
            "/entries/1/output/rewrite/0",
            &["type", "pattern", "replacement", "capture_as"],
        ),
        (
            "/entries/1/output/rewrite/1",
            &[
                "type",
                "start",
                "end",
                "start_offset",
                "end_offset",
                "show_trim_markers",
            ],
        ),
        (
            "/entries/1/output/rewrite/2",
            &["type", "start_pattern", "end_pattern"],
        ),
        (
            "/entries/1/output/rewrite/3",
            &["type", "first", "last", "show_trim_marker"],
        ),
        (
            "/entries/1/output/rewrite/4",
            &[
                "type",
                "pattern",
                "base",
                "id",
                "custom_format",
                "capture_as",
            ],
        ),
        ("/entries/1/output/rewrite/5", &["type", "format", "use"]),
        ("/entries/2/transform", &["language", "operations"]),
        (
            "/entries/2/transform/operations/0",
            &["type", "name", "replacement"],
        ),
        ("/entries/2/transform/operations/1", &["type", "name"]),
        (
            "/entries/3",
            &["type", "level", "title", "z_extra", "a_extra"],
        ),
        ("/entries/4", &["type", "commands"]),
    ];
    for extension in ["json", "yaml", "yml"] {
        let dir = prepare_workspace();
        let input = if extension == "json" {
            serde_json::to_string(&original).unwrap()
        } else {
            serde_norway::to_string(&original).unwrap()
        };
        let path = write_file(&dir, &format!("sw-runbook.{extension}"), &input);
        let output = run_in_dir(&["format"], &dir);
        assert!(
            output.status.success(),
            "{extension}: {} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let formatted = fs::read_to_string(&path).unwrap();
        let parsed: serde_json::Value = if extension == "json" {
            serde_json::from_str(&formatted).unwrap()
        } else {
            serde_norway::from_str(&formatted).unwrap()
        };
        // Value equality includes array order and absent fields, but ignores map order.
        assert_eq!(parsed, original, "{extension}: semantic values");
        for (pointer, expected) in expected_orders {
            let keys: Vec<_> = parsed
                .pointer(pointer)
                .unwrap()
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(keys, *expected, "{extension}: {pointer}");
        }
        for field in ["z_extra", "a_extra"] {
            assert_eq!(
                serde_json::to_string(&parsed["entries"][3][field]).unwrap(),
                serde_json::to_string(&original["entries"][3][field]).unwrap(),
                "{extension}: accepted extra {field} retains nested order"
            );
        }
        assert!(run_in_dir(&["validate"], &dir).status.success());
        assert!(run_in_dir(&["format"], &dir).status.success());
        assert_eq!(fs::read_to_string(path).unwrap(), formatted);
        fs::remove_dir_all(dir).unwrap();
    }
}

#[test]
fn format_rejects_invalid_nested_fields_without_writing() {
    for extension in ["json", "yaml", "yml"] {
        for invalid in [
            r#"{"entries":[{"type":"Command","commands":"echo hi","output":{"rewrite":[{"replacement":"x","pattern":"hi","type":"replace","unknown":true}]}}]}"#,
            r#"{"entries":[{"type":"Prerequisite","checks":[{"version":"25","name":"Java","kind":"java","contents":"Needed","java_home":"jdk","java_home_env":"JAVA_HOME"}]}]}"#,
        ] {
            let dir = prepare_workspace();
            let value: serde_json::Value = serde_json::from_str(invalid).unwrap();
            let input = if extension == "json" {
                invalid.to_string()
            } else {
                serde_norway::to_string(&value).unwrap()
            };
            let path = write_file(&dir, &format!("sw-runbook.{extension}"), &input);
            assert_eq!(run_in_dir(&["validate"], &dir).status.code(), Some(2));
            assert_eq!(run_in_dir(&["format"], &dir).status.code(), Some(2));
            assert_eq!(fs::read_to_string(path).unwrap(), input);
            fs::remove_dir_all(dir).unwrap();
        }
    }
}
