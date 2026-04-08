use crate::cli::{ImportArgs, ImportOutputFormat};
use crate::runbook;
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

const DEFAULT_IMPORT_INPUT: &str = "README.md";
const DEFAULT_IMPORT_OUTPUT_JSON: &str = "sw-runbook.json";
const DEFAULT_IMPORT_OUTPUT_YAML: &str = "sw-runbook.yaml";

pub fn run(args: ImportArgs) -> ExitCode {
    let input_path = args
        .input_file
        .unwrap_or_else(|| PathBuf::from(DEFAULT_IMPORT_INPUT));
    let (output_path, output_format) =
        match resolve_output_target(args.output_file, args.output_format) {
            Ok(target) => target,
            Err(message) => {
                eprintln!("{message}");
                return ExitCode::from(1);
            }
        };

    if output_path.exists() && !args.force {
        eprintln!(
            "Refusing to overwrite existing output file: {}",
            output_path.display()
        );
        return ExitCode::from(1);
    }

    let contents = match fs::read_to_string(&input_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Failed to read {}: {err}", input_path.display());
            return ExitCode::from(1);
        }
    };

    let runbook = import_readme(&contents);
    let Some(entries) = runbook.get("entries").and_then(Value::as_array) else {
        eprintln!("Internal error: imported runbook is missing `entries`");
        return ExitCode::from(1);
    };

    if entries.is_empty() {
        eprintln!(
            "Failed to import {}: no supported content produced any runbook entries",
            input_path.display()
        );
        return ExitCode::from(1);
    }

    let validation = runbook::validate(&runbook, &output_path);
    if !validation.valid {
        eprintln!(
            "Generated runbook failed validation: {}",
            validation_error_summary(&validation)
        );
        return ExitCode::from(1);
    }

    let output = match serialize_runbook(&runbook, output_format) {
        Ok(output) => output,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    if let Err(err) = fs::write(&output_path, output) {
        eprintln!("Failed to write {}: {err}", output_path.display());
        return ExitCode::from(1);
    }

    println!(
        "Imported {} to {}",
        input_path.display(),
        output_path.display()
    );
    ExitCode::SUCCESS
}

fn resolve_output_target(
    output_file: Option<PathBuf>,
    output_format: Option<ImportOutputFormat>,
) -> Result<(PathBuf, ImportOutputFormat), String> {
    let inferred_format = output_file
        .as_ref()
        .and_then(|path| infer_output_format_from_path(path));

    if let (Some(explicit), Some(inferred), Some(path)) =
        (output_format, inferred_format, output_file.as_ref())
        && explicit != inferred
    {
        return Err(format!(
            "Output file extension does not match --output-format: {}",
            path.display()
        ));
    }

    let resolved_format = output_format
        .or(inferred_format)
        .unwrap_or(ImportOutputFormat::Yaml);
    let resolved_path = output_file.unwrap_or_else(|| match resolved_format {
        ImportOutputFormat::Json => PathBuf::from(DEFAULT_IMPORT_OUTPUT_JSON),
        ImportOutputFormat::Yaml => PathBuf::from(DEFAULT_IMPORT_OUTPUT_YAML),
    });

    Ok((resolved_path, resolved_format))
}

fn infer_output_format_from_path(path: &std::path::Path) -> Option<ImportOutputFormat> {
    match path.extension().and_then(|value| value.to_str()) {
        Some("json") => Some(ImportOutputFormat::Json),
        Some("yaml" | "yml") => Some(ImportOutputFormat::Yaml),
        _ => None,
    }
}

fn serialize_runbook(runbook: &Value, format: ImportOutputFormat) -> Result<String, String> {
    let serialized = match format {
        ImportOutputFormat::Json => serde_json::to_string_pretty(runbook)
            .map_err(|err| format!("Failed to serialize imported runbook as JSON: {err}"))?,
        ImportOutputFormat::Yaml => serde_norway::to_string(runbook)
            .map_err(|err| format!("Failed to serialize imported runbook as YAML: {err}"))?,
    };

    if serialized.ends_with('\n') {
        Ok(serialized)
    } else {
        Ok(format!("{serialized}\n"))
    }
}

fn validation_error_summary(result: &runbook::ValidationResult) -> String {
    result
        .errors
        .iter()
        .map(|error| format!("{} {}", error.path, error.message))
        .collect::<Vec<_>>()
        .join("; ")
}

fn import_readme(contents: &str) -> Value {
    let entries = import_entries(contents);
    json!({ "entries": entries })
}

fn import_entries(contents: &str) -> Vec<Value> {
    let mut entries = Vec::new();
    let mut markdown_lines = Vec::new();
    let mut fence_state: Option<FenceState> = None;

    for line in contents.lines() {
        if let Some(state) = &mut fence_state {
            if is_closing_fence(line, state.marker_char, state.marker_len) {
                let state = fence_state.take().expect("fence state should exist");
                match state.kind {
                    FenceKind::Shell => {
                        if !push_command_entry(&mut entries, state.lines.clone()) {
                            markdown_lines.push(state.opening_line);
                            markdown_lines.extend(state.lines);
                            markdown_lines.push(line.to_string());
                        }
                    }
                    FenceKind::Markdown => {
                        markdown_lines.push(line.to_string());
                    }
                }
                continue;
            }

            state.lines.push(line.to_string());
            if matches!(state.kind, FenceKind::Markdown) {
                markdown_lines.push(line.to_string());
            }
            continue;
        }

        if let Some((marker_char, marker_len, info)) = parse_fence_open(line) {
            push_markdown_entry(&mut entries, std::mem::take(&mut markdown_lines));
            let kind = if is_shell_fence(&info) {
                FenceKind::Shell
            } else {
                FenceKind::Markdown
            };
            if matches!(kind, FenceKind::Markdown) {
                markdown_lines.push(line.to_string());
            }
            fence_state = Some(FenceState {
                kind,
                marker_char,
                marker_len,
                opening_line: line.to_string(),
                lines: Vec::new(),
            });
            continue;
        }

        if let Some((level, title)) = parse_heading(line) {
            push_markdown_entry(&mut entries, std::mem::take(&mut markdown_lines));
            entries.push(json!({
                "type": "Heading",
                "level": level,
                "title": title,
            }));
            continue;
        }

        markdown_lines.push(line.to_string());
    }

    if let Some(state) = fence_state.take() {
        match state.kind {
            FenceKind::Shell => {
                markdown_lines.push(state.opening_line);
                markdown_lines.extend(state.lines);
            }
            FenceKind::Markdown => {}
        }
    }

    push_markdown_entry(&mut entries, markdown_lines);
    entries
}

#[derive(Clone, Copy)]
enum FenceKind {
    Shell,
    Markdown,
}

struct FenceState {
    kind: FenceKind,
    marker_char: char,
    marker_len: usize,
    opening_line: String,
    lines: Vec<String>,
}

fn push_markdown_entry(entries: &mut Vec<Value>, lines: Vec<String>) -> bool {
    let trimmed = trim_outer_blank_lines(lines);
    if trimmed.is_empty() {
        return false;
    }

    entries.push(json!({
        "type": "Markdown",
        "contents": trimmed,
    }));
    true
}

fn push_command_entry(entries: &mut Vec<Value>, lines: Vec<String>) -> bool {
    let trimmed = trim_outer_blank_lines(lines);
    if trimmed.is_empty() {
        return false;
    }

    entries.push(json!({
        "type": "Command",
        "commands": trimmed,
    }));
    true
}

fn trim_outer_blank_lines(lines: Vec<String>) -> Vec<String> {
    let start = lines
        .iter()
        .position(|line| !line.trim().is_empty())
        .unwrap_or(lines.len());
    let end = lines
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .map_or(start, |index| index + 1);

    lines[start..end].to_vec()
}

fn parse_heading(line: &str) -> Option<(String, String)> {
    let leading_spaces = line.chars().take_while(|ch| *ch == ' ').count();
    if leading_spaces > 3 {
        return None;
    }

    let trimmed = &line[leading_spaces..];
    let hash_count = trimmed.chars().take_while(|ch| *ch == '#').count();
    if !(1..=6).contains(&hash_count) {
        return None;
    }

    let remainder = &trimmed[hash_count..];
    if !remainder.starts_with(' ') && !remainder.starts_with('\t') {
        return None;
    }

    let title = remainder.trim();
    if title.is_empty() {
        return None;
    }

    let title = title.trim_end_matches('#').trim_end().to_string();
    if title.is_empty() {
        return None;
    }

    Some((format!("H{hash_count}"), title))
}

fn parse_fence_open(line: &str) -> Option<(char, usize, String)> {
    let leading_spaces = line.chars().take_while(|ch| *ch == ' ').count();
    if leading_spaces > 3 {
        return None;
    }

    let trimmed = &line[leading_spaces..];
    let marker_char = trimmed.chars().next()?;
    if marker_char != '`' && marker_char != '~' {
        return None;
    }

    let marker_len = trimmed.chars().take_while(|ch| *ch == marker_char).count();
    if marker_len < 3 {
        return None;
    }

    let info = trimmed[marker_len..].trim().to_string();
    Some((marker_char, marker_len, info))
}

fn is_closing_fence(line: &str, marker_char: char, marker_len: usize) -> bool {
    let leading_spaces = line.chars().take_while(|ch| *ch == ' ').count();
    if leading_spaces > 3 {
        return false;
    }

    let trimmed = &line[leading_spaces..];
    if !trimmed.starts_with(marker_char) {
        return false;
    }

    let count = trimmed.chars().take_while(|ch| *ch == marker_char).count();

    count >= marker_len && trimmed[count..].trim().is_empty()
}

fn is_shell_fence(info: &str) -> bool {
    let Some(language) = info.split_whitespace().next() else {
        return false;
    };

    matches!(
        language.to_ascii_lowercase().as_str(),
        "bash" | "sh" | "shell" | "shellscript" | "zsh"
    )
}

#[cfg(test)]
mod tests {
    use super::import_readme;

    #[test]
    fn import_maps_headings_markdown_and_shell_fences() {
        let runbook = import_readme(
            "# Title\n\nIntro text.\n\n```bash\necho hello\n```\n\n## Next\n\nMore text.\n",
        );
        let entries = runbook["entries"]
            .as_array()
            .expect("entries should be an array");

        assert_eq!(entries.len(), 5);
        assert_eq!(entries[0]["type"], "Heading");
        assert_eq!(entries[0]["level"], "H1");
        assert_eq!(entries[0]["title"], "Title");
        assert_eq!(entries[1]["type"], "Markdown");
        assert_eq!(entries[2]["type"], "Command");
        assert_eq!(entries[2]["commands"][0], "echo hello");
        assert_eq!(entries[3]["type"], "Heading");
        assert_eq!(entries[4]["type"], "Markdown");
    }

    #[test]
    fn import_keeps_non_shell_fences_as_markdown() {
        let runbook = import_readme("```yaml\nname: demo\n```\n");
        let entries = runbook["entries"]
            .as_array()
            .expect("entries should be an array");

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0]["type"], "Markdown");
        assert_eq!(entries[0]["contents"][0], "```yaml");
        assert_eq!(entries[0]["contents"][1], "name: demo");
        assert_eq!(entries[0]["contents"][2], "```");
    }

    #[test]
    fn import_treats_unclosed_shell_fence_as_markdown() {
        let runbook = import_readme("```bash\necho hello\n");
        let entries = runbook["entries"]
            .as_array()
            .expect("entries should be an array");

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0]["type"], "Markdown");
        assert_eq!(entries[0]["contents"][0], "```bash");
        assert_eq!(entries[0]["contents"][1], "echo hello");
    }
}
