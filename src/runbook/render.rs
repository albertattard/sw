use super::RenderError;
use super::execute::{
    cleanup_block, ensure_assertions, execute_command, run_cleanup_blocks, timeout_label,
};
use serde_json::Value;

pub(crate) fn render_markdown(runbook: &Value) -> Result<String, RenderError> {
    let entries = runbook
        .get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            RenderError::Operational("Runbook is missing an entries array".to_string())
        })?;

    let mut sections = Vec::new();
    let mut cleanups: Vec<Vec<String>> = Vec::new();
    let mut failure: Option<RenderError> = None;

    for entry in entries {
        let entry_type = entry.get("type").and_then(Value::as_str).ok_or_else(|| {
            RenderError::Operational("Runbook entry is missing a type".to_string())
        })?;

        let rendered = match entry_type {
            "Heading" => render_heading(entry)?,
            "Markdown" => render_markdown_entry(entry)?,
            "Command" => {
                if let Some(cleanup) = cleanup_block(entry)? {
                    cleanups.push(cleanup);
                }

                match render_command(entry) {
                    Ok(section) => section,
                    Err(RenderError::Timeout {
                        message,
                        partial_markdown,
                    }) => {
                        sections.push(partial_markdown);
                        failure = Some(RenderError::Timeout {
                            message,
                            partial_markdown: String::new(),
                        });
                        break;
                    }
                    Err(err) => {
                        failure = Some(err);
                        break;
                    }
                }
            }
            other => {
                return Err(RenderError::Operational(format!(
                    "Unsupported entry type `{other}`"
                )));
            }
        };

        sections.push(rendered);
    }

    let mut output = sections.join("\n\n");
    if !output.ends_with('\n') {
        output.push('\n');
    }

    let cleanup_failures = run_cleanup_blocks(&cleanups);
    let cleanup_message = if cleanup_failures.is_empty() {
        None
    } else {
        Some(format!(
            "Cleanup failed:\n- {}",
            cleanup_failures.join("\n- ")
        ))
    };

    match failure {
        Some(RenderError::Timeout { message, .. }) => Err(RenderError::Timeout {
            message: combine_messages(&message, cleanup_message.as_deref()),
            partial_markdown: output,
        }),
        Some(RenderError::CommandFailed(message)) => Err(RenderError::CommandFailed(
            combine_messages(&message, cleanup_message.as_deref()),
        )),
        Some(RenderError::Operational(message)) => Err(RenderError::Operational(combine_messages(
            &message,
            cleanup_message.as_deref(),
        ))),
        Some(RenderError::CleanupFailed { .. }) => unreachable!(),
        None => match cleanup_message {
            Some(message) => Err(RenderError::CleanupFailed {
                message,
                markdown: output,
            }),
            None => Ok(output),
        },
    }
}

fn render_heading(entry: &Value) -> Result<String, RenderError> {
    let level = entry
        .get("level")
        .and_then(Value::as_str)
        .ok_or_else(|| RenderError::Operational("Heading entry is missing level".to_string()))?;
    let title = entry
        .get("title")
        .and_then(Value::as_str)
        .ok_or_else(|| RenderError::Operational("Heading entry is missing title".to_string()))?;

    let marker = match level {
        "H1" => "#",
        "H2" => "##",
        "H3" => "###",
        "H4" => "####",
        "H5" => "#####",
        "H6" => "######",
        other => {
            return Err(RenderError::Operational(format!(
                "Unsupported heading level `{other}`"
            )));
        }
    };

    Ok(format!("{marker} {title}"))
}

fn render_markdown_entry(entry: &Value) -> Result<String, RenderError> {
    let contents = entry
        .get("contents")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            RenderError::Operational("Markdown entry is missing contents".to_string())
        })?;

    let mut lines = Vec::new();
    for item in contents {
        lines.push(
            item.as_str()
                .ok_or_else(|| {
                    RenderError::Operational(
                        "Markdown contents must contain only strings".to_string(),
                    )
                })?
                .to_string(),
        );
    }

    Ok(lines.join("\n"))
}

fn render_command(entry: &Value) -> Result<String, RenderError> {
    let commands = entry
        .get("commands")
        .and_then(Value::as_array)
        .ok_or_else(|| RenderError::Operational("Command entry is missing commands".to_string()))?;

    let mut command_lines = Vec::new();
    for item in commands {
        command_lines.push(
            item.as_str()
                .ok_or_else(|| {
                    RenderError::Operational("Command list must contain only strings".to_string())
                })?
                .to_string(),
        );
    }

    let command_text = command_lines.join("\n");
    let mut section = format!("```shell\n{command_text}\n```");
    let execution = execute_command(entry, &command_text)?;

    if execution.timed_out {
        append_output(entry, &execution.stdout, &mut section)?;
        return Err(RenderError::Timeout {
            message: format!("Command timed out after {}", timeout_label(entry)),
            partial_markdown: section,
        });
    }

    ensure_assertions(entry, &execution)?;
    append_output(entry, &execution.stdout, &mut section)?;
    Ok(section)
}

fn append_output(entry: &Value, stdout: &str, section: &mut String) -> Result<(), RenderError> {
    let Some(output) = entry.get("output") else {
        return Ok(());
    };

    if let Some(caption) = output.get("caption") {
        let caption_text = render_caption(caption)?;
        section.push_str("\n\n");
        section.push_str(&caption_text);
    }

    section.push_str("\n\n");
    section.push_str(&format!("```{}\n", output_content_type(output)?));
    section.push_str(stdout);
    if !stdout.ends_with('\n') && !stdout.is_empty() {
        section.push('\n');
    }
    section.push_str("```");
    Ok(())
}

fn render_caption(caption: &Value) -> Result<String, RenderError> {
    match caption {
        Value::String(text) => Ok(text.to_string()),
        Value::Array(items) => {
            let mut lines = Vec::new();
            for item in items {
                lines.push(
                    item.as_str()
                        .ok_or_else(|| {
                            RenderError::Operational(
                                "Command output caption must contain only strings".to_string(),
                            )
                        })?
                        .to_string(),
                );
            }
            Ok(lines.join("\n"))
        }
        _ => Err(RenderError::Operational(
            "Command output caption must be a string or array of strings".to_string(),
        )),
    }
}

fn output_content_type(output: &Value) -> Result<&'static str, RenderError> {
    match output.get("content_type").and_then(Value::as_str) {
        Some("text") | None => Ok("text"),
        Some("json") => Ok("json"),
        Some("xml") => Ok("xml"),
        Some(other) => Err(RenderError::Operational(format!(
            "Unsupported output content type `{other}`"
        ))),
    }
}

fn combine_messages(primary: &str, secondary: Option<&str>) -> String {
    match secondary {
        Some(secondary) => format!("{primary}\n{secondary}"),
        None => primary.to_string(),
    }
}
