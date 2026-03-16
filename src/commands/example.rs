use crate::cli::ExampleArgs;
use std::process::ExitCode;

pub fn run(args: ExampleArgs) -> ExitCode {
    let Some(example) = example_for_topic(&args.topic) else {
        eprintln!("Unknown example topic: {}", args.topic);
        return ExitCode::from(1);
    };

    println!("{example}");
    ExitCode::SUCCESS
}

fn example_for_topic(topic: &str) -> Option<&'static str> {
    match topic.to_ascii_lowercase().as_str() {
        "command" => Some(
            r#"{
  "type": "Command",
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
}"#,
        ),
        "displayfile" => Some(
            r#"{
  "type": "DisplayFile",
  "path": "./src/main/java/demo/Example.java",
  "start_line": 1,
  "line_count": 12
}"#,
        ),
        "prerequisite" => Some(
            r#"{
  "type": "Prerequisite",
  "checks": [
    {
      "kind": "command",
      "name": "Oracle Java 25",
      "contents": [
        "- [Oracle Java 25](https://www.oracle.com/java/technologies/downloads/#java25)"
      ],
      "commands": [
        "java --version"
      ],
      "assert": {
        "exit_code": 0,
        "checks": [
          {
            "source": "stdout",
            "contains": "Java 25"
          }
        ]
      },
      "help": "Install Java 25 and make sure `java` is available on the PATH before running this example."
    }
  ]
}"#,
        ),
        _ => None,
    }
}
