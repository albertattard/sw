use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sw"))
        .args(args)
        .output()
        .expect("failed to execute sw")
}

#[test]
fn explain_run_prints_concise_contract_summary() {
    let output = run(&["explain", "run"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("topic: run"));
    assert!(stdout.contains("availability: implemented"));
    assert!(stdout.contains("Default output file is `./README.md`."));
    assert!(stdout.contains(
        "Use `sw check` first if the question is whether prerequisites such as Java are available."
    ));
}

#[test]
fn explain_validate_prints_validation_contract_summary() {
    let output = run(&["explain", "validate"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("topic: validate"));
    assert!(stdout.contains("Default output format is `human`."));
    assert!(stdout.contains("The command performs no writes."));
}

#[test]
fn explain_all_prints_supported_topics_in_stable_order() {
    let output = run(&["explain", "--all"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    let help_index = stdout.find("topic: help").expect("missing help topic");
    let validate_index = stdout
        .find("topic: validate")
        .expect("missing validate topic");
    let run_index = stdout.find("topic: run").expect("missing run topic");
    let check_index = stdout.find("topic: check").expect("missing check topic");
    let init_index = stdout.find("topic: init").expect("missing init topic");
    let import_index = stdout.find("topic: import").expect("missing import topic");
    let example_index = stdout
        .find("topic: example")
        .expect("missing example topic");

    assert!(help_index < validate_index);
    assert!(validate_index < run_index);
    assert!(run_index < check_index);
    assert!(check_index < init_index);
    assert!(init_index < import_index);
    assert!(import_index < example_index);
}

#[test]
fn explain_check_guides_agent_to_prerequisites_and_examples() {
    let output = run(&["explain", "check"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("how do I check for Java 21?"));
    assert!(stdout.contains("Use `sw example Prerequisite`"));
    assert!(stdout.contains("Built-in prerequisite kinds include `java`"));
}

#[test]
fn explain_boundaries_between_help_example_and_explain_are_clear() {
    let output = run(&["explain", "help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Use `help` for exact flags and invocation syntax."));
    assert!(stdout.contains("Use `explain` when the question is about behavior, defaults, or which command to call next."));
    assert!(stdout.contains("If you need JSON shape examples, use `sw example <topic>` next."));
}

#[test]
fn explain_topic_matching_is_case_insensitive() {
    let lower = run(&["explain", "run"]);
    let upper = run(&["explain", "RUN"]);

    assert!(lower.status.success());
    assert!(upper.status.success());
    assert_eq!(lower.stdout, upper.stdout);
}

#[test]
fn explain_unknown_topic_returns_operational_error() {
    let output = run(&["explain", "unknown"]);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown explain topic: unknown"));
    assert!(stderr.contains("Supported topics: help, validate, run, check, init, import, example"));
}

#[test]
fn explain_without_topic_or_all_returns_usage_error() {
    let output = run(&["explain"]);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("The explain command requires a topic or --all"));
}
