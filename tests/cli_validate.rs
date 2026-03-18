use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn unique_temp_dir() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "sw-validate-test-{}-{nanos}-{id}",
        std::process::id()
    ))
}

fn prepare_workspace() -> PathBuf {
    let dir = unique_temp_dir();
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

fn run_in_dir(args: &[&str], dir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sw"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to execute sw")
}

fn write_runbook(dir: &Path, fixture_name: &str, target_name: &str) -> PathBuf {
    let source = Path::new("tests/fixtures").join(fixture_name);
    let target = dir.join(target_name);
    let contents = fs::read_to_string(source).expect("failed to read fixture");
    fs::write(&target, contents).expect("failed to write fixture");
    target
}

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
fn invalid_runbook_human_output_includes_offending_entry() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-prerequisite-help.json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Runbook is invalid"));
    assert!(stdout.contains(
        "Prerequisite check help must be a single string, not an array. Remove the surrounding [ ]."
    ));
    assert!(stdout.contains("Offending block:"));
    assert!(stdout.contains("\"name\": \"jq\""));
    assert!(stdout.contains("\"help\": ["));
}

#[test]
fn invalid_runbook_human_output_prints_a_block_for_each_error() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-prerequisite-help-multiple.json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.matches("Offending block:").count(), 2);
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
fn invalid_command_debug_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-command-debug.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].debug\""));
    assert!(stdout.contains("must be a boolean"));
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
fn valid_java_prerequisite_returns_success() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-valid-prerequisite-java.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": true"));
}

#[test]
fn valid_patch_returns_success() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-valid-patch.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": true"));
}

#[test]
fn invalid_patch_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-patch.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"path\": \"entries[0].restore\""));
}

#[test]
fn invalid_java_prerequisite_with_both_java_home_and_java_home_env_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-prerequisite-java-both-homes.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].checks[0]\""));
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
fn valid_output_rewrite_keep_between_start_only_returns_success() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-valid-output-rewrite-keep-between-start-only.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": true"));
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
fn negative_display_file_indent_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-display-file-indent-negative.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].indent\""));
    assert!(stdout.contains("non-negative integer"));
}

#[test]
fn display_file_negative_offset_warning_keeps_runbook_valid() {
    let dir = prepare_workspace();
    write_runbook(
        &dir,
        "sw-runbook-valid-display-file-offset-warning.json",
        "sw-runbook.json",
    );
    fs::write(dir.join("Example.java"), "    four spaces\n  two spaces\n")
        .expect("failed to write Example.java");

    let output = run_in_dir(&["validate", "--output-format", "json"], &dir);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": true"));
    assert!(stdout.contains("\"warnings\": ["));
    assert!(stdout.contains("\"path\": \"entries[0].offset\""));
    assert!(stdout.contains("cannot be fully applied"));
}

#[test]
fn invalid_display_file_transform_language_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-display-file-transform-language.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].transform.language\""));
    assert!(stdout.contains("must be `java`"));
}

#[test]
fn invalid_display_file_transform_operation_returns_validation_failure() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-invalid-display-file-transform-operation.json",
        "--output-format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": false"));
    assert!(stdout.contains("\"path\": \"entries[0].transform.operations[0].type\""));
    assert!(stdout.contains("collapse_method_body"));
}

#[test]
fn background_command_warning_keeps_runbook_valid_in_json_output() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-valid-background-command-warning.json",
        "--output-format",
        "json",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": true"));
    assert!(stdout.contains("\"warnings\": ["));
    assert!(stdout.contains("\"path\": \"entries[0].commands\""));
    assert!(stdout.contains("background process"));
    assert!(stdout.contains("timeout or progress behavior misleading"));
}

#[test]
fn background_command_warning_appears_in_human_output() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-valid-background-command-warning.json",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Runbook is valid"));
    assert!(stdout.contains("warning entries[0].commands"));
    assert!(stdout.contains("background process"));
    assert!(stdout.contains("saving `$!` to a PID file"));
}

#[test]
fn redirected_background_command_does_not_warn() {
    let output = run(&[
        "validate",
        "--input-file",
        "tests/fixtures/sw-runbook-valid-background-command-redirected.json",
        "--output-format",
        "json",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\": true"));
    assert!(stdout.contains("\"warnings\": []"));
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
