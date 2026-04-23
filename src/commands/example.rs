use crate::cli::ExampleArgs;
use std::process::ExitCode;

pub fn run(args: ExampleArgs) -> ExitCode {
    let output_format = match parse_output_format(args.output_format.as_deref()) {
        Ok(output_format) => output_format,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    let Some(example) = example_for_topic(&args.topic) else {
        eprintln!("Unknown example topic: {}", args.topic);
        return ExitCode::from(1);
    };

    let rendered = match output_format {
        ExampleOutputFormat::Json => example.json,
        ExampleOutputFormat::Yaml => example.yaml,
    };

    println!("{rendered}");
    ExitCode::SUCCESS
}

#[derive(Clone, Copy)]
enum ExampleOutputFormat {
    Json,
    Yaml,
}

#[derive(Clone, Copy)]
struct ExampleSnippet {
    json: &'static str,
    yaml: &'static str,
}

fn parse_output_format(value: Option<&str>) -> Result<ExampleOutputFormat, String> {
    match value.map(|value| value.to_ascii_lowercase()) {
        None => Ok(ExampleOutputFormat::Yaml),
        Some(value) if value == "json" => Ok(ExampleOutputFormat::Json),
        Some(value) if value == "yaml" => Ok(ExampleOutputFormat::Yaml),
        Some(value) => Err(format!("Unknown example output format: {value}")),
    }
}

fn example_for_topic(topic: &str) -> Option<ExampleSnippet> {
    match topic.to_ascii_lowercase().as_str() {
        "command" => Some(ExampleSnippet {
            json: COMMAND_EXAMPLE_JSON,
            yaml: COMMAND_EXAMPLE_YAML,
        }),
        "displayfile" => Some(ExampleSnippet {
            json: DISPLAY_FILE_EXAMPLE_JSON,
            yaml: DISPLAY_FILE_EXAMPLE_YAML,
        }),
        "patch" => Some(ExampleSnippet {
            json: PATCH_EXAMPLE_JSON,
            yaml: PATCH_EXAMPLE_YAML,
        }),
        "prerequisite" => Some(ExampleSnippet {
            json: PREREQUISITE_EXAMPLE_JSON,
            yaml: PREREQUISITE_EXAMPLE_YAML,
        }),
        _ => None,
    }
}

const COMMAND_EXAMPLE_JSON: &str = r#"{
  "type": "Command",
  "debug": true,
  "commands": [
    "echo 'Hello there'"
  ],
  "timeout": "30 seconds",
  "indent": 3,
  "assert": {
    "exit_code": 0,
    "checks": [
      {
        "source": "stdout",
        "contains": "Hello"
      }
    ]
  },
  "output": {
    "caption": "Command output",
    "content_type": "text",
    "stream": "combined",
    "trim_empty_lines": "leading_trailing",
    "trim_trailing_whitespace": true,
    "rewrite": [
      {
        "type": "replace",
        "pattern": "/Users/demo/project",
        "replacement": "."
      },
      {
        "type": "datetime_shift",
        "format": "rfc3339",
        "id": "main_timeline"
      },
      {
        "type": "datetime_shift",
        "use": "main_timeline",
        "pattern": "\\b\\d{2}:\\d{2}:\\d{2}\\.\\d{3}\\b",
        "custom_format": "%H:%M:%S%.3f"
      },
      {
        "type": "keep_between",
        "start": "[INFO] --- begin ---",
        "end": "[INFO] --- end ---",
        "start_offset": 1,
        "end_offset": -1,
        "show_trim_markers": true
      }
    ]
  },
  "capture": [
    {
      "name": "generated_value",
      "source": "stdout",
      "stage": "rewritten",
      "pattern": "(demo)"
    }
  ],
  "cleanup": [
    "echo 'cleanup'"
  ]
}"#;

const COMMAND_EXAMPLE_YAML: &str = r#"type: Command
debug: true
commands:
  - "echo 'Hello there'"
timeout: 30 seconds
indent: 3
assert:
  exit_code: 0
  checks:
    - source: stdout
      contains: Hello
output:
  caption: Command output
  content_type: text
  stream: combined
  trim_empty_lines: leading_trailing
  trim_trailing_whitespace: true
  rewrite:
    - type: replace
      pattern: /Users/demo/project
      replacement: .
    - type: datetime_shift
      format: rfc3339
      id: main_timeline
    - type: datetime_shift
      use: main_timeline
      pattern: '\b\d{2}:\d{2}:\d{2}\.\d{3}\b'
      custom_format: '%H:%M:%S%.3f'
    - type: keep_between
      start: '[INFO] --- begin ---'
      end: '[INFO] --- end ---'
      start_offset: 1
      end_offset: -1
      show_trim_markers: true
capture:
  - name: generated_value
    source: stdout
    stage: rewritten
    pattern: '(demo)'
cleanup:
  - "echo 'cleanup'""#;

const DISPLAY_FILE_EXAMPLE_JSON: &str = r#"{
  "type": "DisplayFile",
  "path": "./src/main/java/demo/Example.java",
  "start_line": 1,
  "line_count": 12,
  "indent": 3,
  "transform": {
    "language": "java",
    "operations": [
      {
        "type": "collapse_method_body",
        "name": "initialize"
      }
    ]
  }
}"#;

const DISPLAY_FILE_EXAMPLE_YAML: &str = r#"type: DisplayFile
path: ./src/main/java/demo/Example.java
start_line: 1
line_count: 12
indent: 3
transform:
  language: java
  operations:
    - type: collapse_method_body
      name: initialize"#;

const PATCH_EXAMPLE_JSON: &str = r#"{
  "type": "Patch",
  "path": "./src/main/java/demo/Main.java",
  "patch": [
    "@@ -10,3 +10,3 @@",
    "-        return oldValue;",
    "+        return newValue;"
  ],
  "indent": 3
}"#;

const PATCH_EXAMPLE_YAML: &str = r#"type: Patch
path: ./src/main/java/demo/Main.java
patch:
  - '@@ -10,3 +10,3 @@'
  - '-        return oldValue;'
  - '+        return newValue;'
indent: 3"#;

const PREREQUISITE_EXAMPLE_JSON: &str = r#"{
  "type": "Prerequisite",
  "checks": [
    {
      "kind": "java",
      "name": "Java 25+",
      "version": "25+",
      "contents": [
        "- [Java 25 downloads](https://www.oracle.com/java/technologies/downloads/#java25)"
      ],
      "help": "Install Java 25 or newer and make sure `java` is available on the PATH before running this example."
    }
  ]
}"#;

const PREREQUISITE_EXAMPLE_YAML: &str = r#"type: Prerequisite
checks:
  - kind: java
    name: Java 25+
    version: '25+'
    contents:
      - '- [Java 25 downloads](https://www.oracle.com/java/technologies/downloads/#java25)'
    help: Install Java 25 or newer and make sure `java` is available on the PATH before running this example."#;
