use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sw"))
        .args(args)
        .output()
        .expect("failed to execute sw")
}

#[test]
fn help_flag_prints_help() {
    let output = run(&["--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Sociable Weaver (SW)"));
    assert!(stdout.contains("--version"));
    assert!(stdout.contains("--verbose-mode"));
    assert!(stdout.contains("Still weaving the nest. Features are hatching soon."));
}

#[test]
fn help_subcommand_prints_help() {
    let output = run(&["help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("--verbose"));
    assert!(stdout.contains("--verbose-mode"));
    assert!(stdout.contains("--debug"));
    assert!(stdout.contains("check"));
    assert!(stdout.contains("convert"));
    assert!(stdout.contains("example"));
    assert!(stdout.contains("explain"));
    assert!(stdout.contains("format"));
    assert!(stdout.contains("init"));
    assert!(stdout.contains("import"));
    assert!(stdout.contains("help"));
    assert!(stdout.contains("version"));
}

#[test]
fn help_subcommand_for_known_topic_prints_targeted_help() {
    let output = run(&["help", "run"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Render a runbook to output"));
    assert!(stdout.contains("--input-file"));
    assert!(stdout.contains("Use `-` to read from stdin"));
    assert!(stdout.contains("--input-format"));
    assert!(stdout.contains("Ignored unless `--input-file=-` is used"));
    assert!(stdout.contains(
        "`Markdown`, `DisplayFile`, `Patch`, and `Command` entries may declare `indent`"
    ));
    assert!(stdout.contains("File-based runbooks default to YAML"));
    assert!(stdout.contains("--output-file"));
    assert!(stdout.contains("trim_empty_lines"));
    assert!(stdout.contains("stream"));
    assert!(stdout.contains("cleanup"));
    assert!(stdout.contains("sw example Command"));
    assert!(stdout.contains("sw explain run"));
    assert!(!stdout.contains("Check runbook prerequisites"));
}

#[test]
fn help_subcommand_for_unknown_topic_fails() {
    let output = run(&["help", "unknown"]);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown help topic: unknown"));
}

#[test]
fn help_all_prints_top_level_and_known_subcommand_help() {
    let output = run(&["help", "--all"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Sociable Weaver (SW)"));
    assert!(stdout.contains("Check runbook prerequisites"));
    assert!(stdout.contains("Convert a runbook file to the opposite supported format"));
    assert!(stdout.contains("Print a runbook example for a topic"));
    assert!(stdout.contains("Explain a feature contract or discovery path"));
    assert!(stdout.contains("Format a runbook file in place"));
    assert!(stdout.contains("Generate a starter runbook file"));
    assert!(stdout.contains("Import a Markdown README into a starter runbook"));
    assert!(stdout.contains("Render a runbook to output"));
    assert!(stdout.contains("Show help for the CLI or a specific subcommand"));
    assert!(stdout.contains("Print version/build identity"));
    assert!(stdout.contains("Validate a runbook file"));
}

#[test]
fn check_help_prints_help() {
    let output = run(&["check", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--input-file"));
    assert!(stdout.contains("Use `-` to read from stdin"));
    assert!(stdout.contains("--input-format"));
    assert!(stdout.contains("Ignored unless `--input-file=-` is used"));
    assert!(stdout.contains("File-based runbooks default to YAML"));
    assert!(stdout.contains("Command-based prerequisite checks default to a `5 seconds` timeout"));
    assert!(stdout.contains("sw explain check"));
}

#[test]
fn example_help_prints_help() {
    let output = run(&["example", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Example topic"));
    assert!(stdout.contains("--output-format"));
    assert!(stdout.contains("Defaults to YAML output for file-based authoring."));
    assert!(stdout.contains("trim_empty_lines"));
    assert!(stdout.contains("stream"));
    assert!(stdout.contains("The `Command` example includes current nested output fields"));
    assert!(stdout.contains("sw example DisplayFile"));
    assert!(stdout.contains("collapse_method_body"));
}

#[test]
fn convert_help_prints_help() {
    let output = run(&["convert", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--input-file"));
    assert!(stdout.contains("--output-file"));
    assert!(stdout.contains("--output-format"));
    assert!(stdout.contains("--force"));
    assert!(stdout.contains("Converts a runbook file from JSON to YAML"));
    assert!(stdout.contains("If exactly one default runbook exists"));
    assert!(stdout.contains("does not accept `--input-file=-`"));
}

#[test]
fn explain_help_prints_help() {
    let output = run(&["explain", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Explain topic"));
    assert!(stdout.contains("--all"));
    assert!(stdout.contains("--output-format"));
    assert!(stdout.contains("--output-file"));
    assert!(stdout.contains("--force"));
}

#[test]
fn format_help_prints_help() {
    let output = run(&["format", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--input-file"));
    assert!(stdout.contains("Formats JSON and YAML runbooks in place"));
    assert!(stdout.contains("Default input candidates are `./sw-runbook.json`, `./sw-runbook.yaml`, and `./sw-runbook.yml`."));
    assert!(stdout.contains("does not accept `--input-file=-`"));
}

#[test]
fn import_help_prints_help() {
    let output = run(&["import", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--input-file"));
    assert!(stdout.contains("--output-file"));
    assert!(stdout.contains("--output-format"));
    assert!(stdout.contains("--force"));
    assert!(stdout.contains("Defaults to `./README.md` input and `./sw-runbook.yaml` output."));
    assert!(stdout.contains("YAML is the default file-based import format."));
    assert!(stdout.contains("Headings map to `Heading` entries"));
    assert!(stdout.contains("sw explain import"));
}

#[test]
fn init_help_prints_help() {
    let output = run(&["init", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--output-file"));
    assert!(stdout.contains("--force"));
    assert!(stdout.contains("Defaults to `./sw-runbook.yaml`."));
    assert!(stdout.contains("YAML is the default file-based starter format."));
    assert!(stdout.contains("`.yaml`, `.yml`, and `.json`"));
    assert!(stdout.contains("Heading"));
    assert!(stdout.contains("Prerequisite"));
    assert!(stdout.contains("sw explain init"));
}

#[test]
fn run_help_prints_help() {
    let output = run(&["run", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--verbose"));
    assert!(stdout.contains("--verbose-mode"));
    assert!(stdout.contains("--debug"));
    assert!(stdout.contains("--input-file"));
    assert!(stdout.contains("Use `-` to read from stdin"));
    assert!(stdout.contains("--input-format"));
    assert!(stdout.contains("Ignored unless `--input-file=-` is used"));
    assert!(stdout.contains("File-based runbooks default to YAML"));
    assert!(stdout.contains("--output-format"));
    assert!(stdout.contains("--output-file"));
    assert!(stdout.contains("trim_empty_lines"));
    assert!(stdout.contains("stream"));
    assert!(stdout.contains("cleanup"));
    assert!(stdout.contains("`Command` entries default to a `2 minutes` timeout"));
    assert!(stdout.contains("prerequisite checks default to `5 seconds`"));
    assert!(stdout.contains("`.java` as `java`, `.sql` as `sql`, and `.xml` as `xml`"));
    assert!(stdout.contains("SSH-safe line-based progress output"));
    assert!(stdout.contains("sw example Command"));
    assert!(stdout.contains("--output-format json"));
    assert!(stdout.contains("sw explain run"));
}

#[test]
fn validate_help_prints_help() {
    let output = run(&["validate", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--input-file"));
    assert!(stdout.contains("Use `-` to read from stdin"));
    assert!(stdout.contains("--input-format"));
    assert!(stdout.contains("Ignored unless `--input-file=-` is used"));
    assert!(stdout.contains("File-based runbooks default to YAML elsewhere in the CLI"));
    assert!(stdout.contains("--output-format"));
}

#[test]
fn unknown_command_fails() {
    let output = run(&["unknown"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unrecognized subcommand"));
}

#[test]
fn version_flag_prints_version_build_identity() {
    let output = run(&["--version"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with(&format!("sw {}", env!("CARGO_PKG_VERSION"))));
    if let Some(commit) = option_env!("SW_GIT_COMMIT")
        && !commit.is_empty()
    {
        assert!(stdout.contains(commit));
    }
}

#[test]
fn version_subcommand_matches_version_flag() {
    let flag_output = run(&["--version"]);
    let command_output = run(&["version"]);

    assert!(flag_output.status.success());
    assert!(command_output.status.success());
    assert_eq!(flag_output.stdout, command_output.stdout);
}
