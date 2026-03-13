use crate::cli::ExampleArgs;
use serde_json::json;
use std::process::ExitCode;

pub fn run(args: ExampleArgs) -> ExitCode {
    let Some(example) = example_for_topic(&args.topic) else {
        eprintln!("Unknown example topic: {}", args.topic);
        return ExitCode::from(1);
    };

    match serde_json::to_string_pretty(&example) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("Failed to render example: {err}");
            ExitCode::from(1)
        }
    }
}

fn example_for_topic(topic: &str) -> Option<serde_json::Value> {
    match topic {
        "Command" => Some(json!({
            "type": "Command",
            "commands": [
                "echo 'Hello there'"
            ],
            "output": {
                "caption": "Command output"
            }
        })),
        "DisplayFile" => Some(json!({
            "type": "DisplayFile",
            "path": "./src/main/java/demo/Example.java",
            "start_line": 1,
            "line_count": 12
        })),
        "Prerequisite" => Some(json!({
            "type": "Prerequisite",
            "checks": [
                {
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
        })),
        "rewrite.keep_between" => Some(json!({
            "type": "keep_between",
            "start": "[INFO] --- exec:3.6.3:exec (default-cli) @ demo ---",
            "end": "[INFO] ------------------------------------------------------------------------",
            "start_offset": 1,
            "end_offset": -1,
            "show_trim_markers": true
        })),
        "rewrite.replace" => Some(json!({
            "type": "replace",
            "pattern": "/Users/demo/project",
            "replacement": "."
        })),
        "rewrite.datetime_shift" => Some(json!({
            "type": "datetime_shift",
            "format": "rfc3339",
            "id": "main_timeline"
        })),
        "rewrite.capture_replace" => Some(json!({
            "type": "replace",
            "pattern": "@{audio_path_1_original}",
            "replacement": "@{audio_path_1_rewritten}"
        })),
        _ => None,
    }
}
