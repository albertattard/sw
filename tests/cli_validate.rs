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

#[test]
fn invalid_file_assert_checks_shape_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-assert-file.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].assert.checks[0]\""));
    assert!(stdout.contains("\"path\": \"entries[0].assert.checks[0].exists\""));
    assert!(stdout.contains("\"path\": \"entries[0].assert.checks[0].sha256\""));
}

#[test]
fn invalid_timeout_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-timeout.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].timeout\""));
}

#[test]
fn invalid_cleanup_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-cleanup.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].cleanup\""));
}

#[test]
fn invalid_output_content_type_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-content-type.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.content_type\""));
}

#[test]
fn invalid_output_trim_trailing_whitespace_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-trim-whitespace.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.trim_trailing_whitespace\""));
}

#[test]
fn invalid_output_rewrite_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.rewrite[0].replacement\""));
}

#[test]
fn invalid_output_rewrite_datetime_shift_with_format_and_pattern_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite-datetime-shift-format-pattern.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.rewrite[0]\""));
}

#[test]
fn invalid_output_rewrite_datetime_shift_with_id_and_use_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite-datetime-shift-id-use.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.rewrite[0]\""));
}

#[test]
fn invalid_output_rewrite_datetime_shift_with_format_and_custom_format_returns_validation_failure()
{
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite-datetime-shift-format-custom-format.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.rewrite[0]\""));
}

#[test]
fn invalid_output_rewrite_datetime_shift_with_use_and_base_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite-datetime-shift-use-base.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.rewrite[0]\""));
}

#[test]
fn invalid_output_rewrite_datetime_shift_pattern_requires_custom_format() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite-datetime-shift-pattern-missing-custom-format.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.rewrite[0]\""));
}

#[test]
fn invalid_output_rewrite_datetime_shift_duplicate_id_same_block_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite-datetime-shift-duplicate-id-same-block.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.rewrite[1].id\""));
}

#[test]
fn invalid_output_rewrite_datetime_shift_duplicate_id_different_commands_returns_validation_failure()
 {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite-datetime-shift-duplicate-id-different-commands.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[1].output.rewrite[0].id\""));
}

#[test]
fn invalid_output_rewrite_datetime_shift_use_before_anchor_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite-datetime-shift-use-before-anchor.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.rewrite[0].use\""));
}

#[test]
fn valid_output_rewrite_datetime_shift_use_cross_command_returns_success() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-valid-output-rewrite-datetime-shift-use-cross-command.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": true"));
}

#[test]
fn invalid_output_rewrite_keep_between_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite-keep-between.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.rewrite[0].start_offset\""));
}

#[test]
fn invalid_output_rewrite_keep_between_show_trim_markers_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite-keep-between-show-trim-markers.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.rewrite[0].show_trim_markers\""));
}

#[test]
fn invalid_display_file_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-display-file.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].path\""));
}

#[test]
fn invalid_display_file_line_count_without_start_line_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-display-file-line-count-without-start-line.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].line_count\""));
    assert!(stdout.contains("requires `start_line`"));
}

#[test]
fn invalid_display_file_line_range_values_return_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-display-file-line-range-values.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].start_line\""));
    assert!(stdout.contains("\"path\": \"entries[0].line_count\""));
}

#[test]
fn invalid_display_file_indent_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-display-file-indent.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].indent\""));
}

#[test]
fn invalid_prerequisites_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-prerequisites.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].checks[0].commands\""));
}

#[test]
fn invalid_prerequisites_plural_entry_type_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-prerequisites-plural.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].type\""));
    assert!(stdout.contains("unsupported entry type `Prerequisites`"));
}

#[test]
fn invalid_capture_duplicate_name_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-capture-duplicate-name.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[1].capture[0].name\""));
}

#[test]
fn invalid_capture_forward_reference_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-capture-forward-reference.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].commands[0]\""));
}

#[test]
fn invalid_output_rewrite_replacement_forward_reference_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite-replacement-forward-reference.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.rewrite[0].replacement\""));
}

#[test]
fn invalid_output_rewrite_pattern_forward_reference_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-output-rewrite-pattern-forward-reference.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].output.rewrite[0].pattern\""));
}

#[test]
fn invalid_rewrite_generated_capture_collision_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-rewrite-generated-capture-collision.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[1].capture[0].name\""));
}

#[test]
fn invalid_markdown_forward_reference_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-markdown-forward-reference.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].contents[0]\""));
}
