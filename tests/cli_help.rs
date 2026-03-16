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
    assert!(stdout.contains("check"));
    assert!(stdout.contains("example"));
    assert!(stdout.contains("help"));
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
