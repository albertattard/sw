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
    assert!(stdout.contains("Still weaving the nest. Features are hatching soon."));
}

#[test]
fn help_subcommand_prints_help() {
    let output = run(&["help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("--verbose"));
    assert!(stdout.contains("--debug"));
    assert!(stdout.contains("check"));
    assert!(stdout.contains("example"));
    assert!(stdout.contains("help"));
}

#[test]
fn help_subcommand_for_known_topic_prints_targeted_help() {
    let output = run(&["help", "run"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Render a runbook to output"));
    assert!(stdout.contains("--input-file"));
    assert!(stdout.contains("--output-file"));
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
    assert!(stdout.contains("Print a JSON example for a runbook topic"));
    assert!(stdout.contains("Render a runbook to output"));
    assert!(stdout.contains("Show help for the CLI or a specific subcommand"));
    assert!(stdout.contains("Validate a runbook file"));
}

#[test]
fn check_help_prints_help() {
    let output = run(&["check", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--input-file"));
}

#[test]
fn example_help_prints_help() {
    let output = run(&["example", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Example topic"));
    assert!(stdout.contains("rewrite.keep_between"));
}

#[test]
fn run_help_prints_help() {
    let output = run(&["run", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--verbose"));
    assert!(stdout.contains("--debug"));
    assert!(stdout.contains("--input-file"));
    assert!(stdout.contains("--output-format"));
    assert!(stdout.contains("--output-file"));
}

#[test]
fn validate_help_prints_help() {
    let output = run(&["validate", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--input-file"));
    assert!(stdout.contains("--output-format"));
}

#[test]
fn unknown_command_fails() {
    let output = run(&["unknown"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unrecognized subcommand"));
}
