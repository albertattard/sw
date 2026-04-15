use crate::cli::InitArgs;
use crate::runbook;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const DEFAULT_INIT_OUTPUT: &str = "sw-runbook.yaml";

pub fn run(args: InitArgs) -> ExitCode {
    let output_path = args
        .output_file
        .unwrap_or_else(|| PathBuf::from(DEFAULT_INIT_OUTPUT));
    let output_format = match infer_output_format(&output_path) {
        Ok(output_format) => output_format,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    if output_path.exists() && !args.force {
        eprintln!(
            "Refusing to overwrite existing file {} without --force",
            output_path.display()
        );
        return ExitCode::from(1);
    }

    let runbook = sample_runbook();
    let validation_result = runbook::validate(&runbook, &output_path);
    if !validation_result.valid {
        eprintln!("Internal error: generated starter runbook is invalid for the current contract");
        return ExitCode::from(1);
    }

    let rendered: Result<String, String> = match output_format {
        InitOutputFormat::Json => serde_json::to_string_pretty(&runbook)
            .map(|s| format!("{s}\n"))
            .map_err(|err| err.to_string()),
        InitOutputFormat::Yaml => serde_norway::to_string(&runbook).map_err(|err| err.to_string()),
    };
    let rendered = match rendered {
        Ok(rendered) => rendered,
        Err(err) => {
            eprintln!("Failed to serialize starter runbook: {err}");
            return ExitCode::from(1);
        }
    };

    if let Err(err) = std::fs::write(&output_path, rendered) {
        eprintln!("Failed to write {}: {err}", output_path.display());
        return ExitCode::from(1);
    }

    println!("Created starter runbook at {}", output_path.display());
    ExitCode::SUCCESS
}

enum InitOutputFormat {
    Json,
    Yaml,
}

fn infer_output_format(path: &Path) -> Result<InitOutputFormat, String> {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("json") => Ok(InitOutputFormat::Json),
        Some("yaml" | "yml") => Ok(InitOutputFormat::Yaml),
        _ => Err(format!(
            "Unsupported init output format for {}. Use a .json, .yaml, or .yml file name.",
            path.display()
        )),
    }
}

fn sample_runbook() -> Value {
    json!({
        "entries": [
            {
                "type": "Heading",
                "level": "H1",
                "title": "Example Workflow"
            },
            {
                "type": "Markdown",
                "contents": [
                    "Use this starter runbook as a realistic template for your project.",
                    "",
                    "Replace the placeholder file paths, prerequisite checks, commands, and rewrites with behavior that matches your workflow."
                ]
            },
            {
                "type": "DisplayFile",
                "path": "./src/main/java/demo/Example.java",
                "start_line": 1,
                "line_count": 12
            },
            {
                "type": "Prerequisite",
                "checks": [
                    {
                        "kind": "command",
                        "name": "Example prerequisite",
                        "contents": [
                            "- Replace this with an environment check that matters for your project."
                        ],
                        "commands": [
                            "printf 'Replace this prerequisite with a real check\\n'"
                        ],
                        "assert": {
                            "exit_code": 0,
                            "checks": [
                                {
                                    "source": "stdout",
                                    "contains": "Replace this prerequisite"
                                }
                            ]
                        },
                        "help": "Update this prerequisite so it verifies the toolchain or services your workflow needs."
                    }
                ]
            },
            {
                "type": "Command",
                "commands": [
                    "printf 'Workspace: /Users/demo/project\\n'",
                    "printf 'token=demo-token\\n'",
                    "printf 'status=ready\\n'"
                ],
                "timeout": "30 seconds",
                "assert": {
                    "exit_code": 0,
                    "checks": [
                        {
                            "source": "stdout",
                            "contains": "status=ready"
                        }
                    ]
                },
                "output": {
                    "caption": [
                        "Observed output"
                    ],
                    "content_type": "text",
                    "stream": "combined",
                    "trim_empty_lines": "leading_trailing",
                    "trim_trailing_whitespace": true,
                    "rewrite": [
                        {
                            "type": "replace",
                            "pattern": "/Users/demo/project",
                            "replacement": "."
                        }
                    ]
                },
                "capture": [
                    {
                        "name": "sample_token",
                        "source": "stdout",
                        "stage": "rewritten",
                        "pattern": "token=(.+)"
                    }
                ],
                "cleanup": [
                    "printf 'Replace this cleanup with real teardown steps\\n' >/dev/null"
                ]
            },
            {
                "type": "Markdown",
                "contents": [
                    "Captured token: @{sample_token}",
                    "",
                    "Use later Markdown entries like this to document values captured from earlier commands."
                ]
            }
        ]
    })
}
