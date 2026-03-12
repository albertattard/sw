use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sw"))
        .args(args)
        .output()
        .expect("failed to execute sw")
}

#[test]
fn valid_runbook_returns_success_json() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-anonymized.json",
        "--output-format",
        "json",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": true"));
    assert!(stdout.contains("\"errors\": []"));
}

#[test]
fn invalid_runbook_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-missing-field.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].title\""));
}

#[test]
fn invalid_json_returns_operational_error() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-json.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\":false"));
    assert!(stdout.contains("Invalid JSON"));
}

#[test]
fn missing_file_returns_operational_error() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/does-not-exist.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Failed to read"));
}

#[test]
fn invalid_assert_shape_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-assert.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].assert.exit_code\""));
}

#[test]
fn invalid_assert_checks_shape_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-assert-checks.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].assert.checks[0].source\""));
    assert!(stdout.contains("\"path\": \"entries[0].assert.checks[0].contains\""));
}
