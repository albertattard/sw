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
    std::env::temp_dir().join(format!("sw-test-{}-{nanos}-{id}", std::process::id()))
}

fn prepare_workspace() -> PathBuf {
    let dir = unique_temp_dir();
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sw"))
        .args(args)
        .output()
        .expect("failed to execute sw")
}

fn run_in_dir(args: &[&str], dir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sw"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to execute sw")
}

fn run_in_dir_with_env(args: &[&str], dir: &Path, envs: &[(&str, &Path)]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_sw"));
    command.args(args).current_dir(dir);
    for (key, value) in envs {
        command.env(key, value);
    }
    command.output().expect("failed to execute sw")
}

#[test]
fn explain_run_prints_concise_contract_summary() {
    let output = run(&["explain", "run"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("topic: run"));
    assert!(stdout.contains("availability: implemented"));
    assert!(stdout.contains("Default input file lookup order is `./sw-runbook.json`, `./sw-runbook.yaml`, then `./sw-runbook.yml`."));
    assert!(stdout.contains(
        "When `--input-file=-` is used, stdin is parsed as JSON unless `--input-format=yaml` is provided."
    ));
    assert!(stdout.contains("Default output file is `./README.md`."));
    assert!(stdout.contains(
        "Command output trims leading and trailing empty lines by default unless `output.trim_empty_lines` overrides it."
    ));
    assert!(stdout.contains(
        "`Command` entries default to a `2 minutes` timeout unless `timeout` is provided."
    ));
    assert!(stdout.contains(
        "Command output can render `stdout`, `stderr`, or `combined` with `output.stream`; if omitted, rendered output defaults to `combined`."
    ));
    assert!(stdout.contains(
        "`output.stream` changes rendered output only and does not widen capture or assertion sources."
    ));
    assert!(
        stdout.contains(
            "`sw --input-file=-` or `sw run --input-file=-` for stdin-backed JSON runbooks"
        )
    );
    assert!(stdout.contains("`--input-format json|yaml` for stdin-backed input"));
    assert!(
        stdout.contains("`--verbose-mode auto|live|plain` to choose how verbose progress renders")
    );
    assert!(stdout.contains("`--input-file=-` reads the runbook from stdin."));
    assert!(stdout.contains(
        "For stdin input, JSON is the default format and piped YAML requires `--input-format=yaml`."
    ));
    assert!(stdout.contains(
        "Without `--input-file=-`, `--input-format` keeps the existing file-backed defaults and does not override file-extension-based parsing."
    ));
    assert!(stdout.contains(
        "`DisplayFile` fence detection recognizes `.java` as `java` and `.sql` as `sql`; other extensions fall back to `text`."
    ));
    assert!(stdout.contains(
        "Use `sw check` first if the question is whether prerequisites such as Java are available."
    ));
    assert!(stdout.contains(
        "Prefer `output.rewrite` with `type: datetime_shift` over `replace` for semantic dates and times so relative timing stays intact."
    ));
    assert!(stdout.contains(
        "Use `replace` for dates and times only when the text is not semantically a date or time, or when `datetime_shift` cannot express the required format."
    ));
    assert!(stdout.contains(
        "Command output can trim outer empty lines with `output.trim_empty_lines` using `leading_trailing`, `leading`, `trailing`, or `none`."
    ));
    assert!(stdout.contains(
        "Command-based prerequisite checks use a shorter default timeout of `5 seconds` unless they declare `timeout`."
    ));
    assert!(stdout.contains(
        "`--verbose-mode=plain` is useful for SSH and wrapper-driven execution where in-place redraws are fragile."
    ));
    assert!(stdout.contains(
        "Use `sw example Command` when you need the current JSON shape for output fields such as `trim_empty_lines` and `stream`."
    ));
}

#[test]
fn explain_validate_prints_validation_contract_summary() {
    let output = run(&["explain", "validate"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("topic: validate"));
    assert!(stdout.contains("Default output format is `human`."));
    assert!(stdout.contains(
        "When `--input-file=-` is used, stdin is parsed as JSON unless `--input-format=yaml` is provided."
    ));
    assert!(stdout.contains("`sw validate --input-file=-`"));
    assert!(stdout.contains("`--input-format json|yaml` for stdin-backed input"));
    assert!(stdout.contains("Validation accepts JSON, YAML, and YML runbooks."));
    assert!(stdout.contains("`--input-file=-` reads the runbook from stdin."));
    assert!(stdout.contains(
        "Without `--input-file=-`, `--input-format` does not replace the normal file lookup or file-extension-based parsing behavior."
    ));
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
    assert!(stdout.contains("Check accepts JSON, YAML, and YML runbooks."));
    assert!(stdout.contains(
        "When `--input-file=-` is used, stdin is parsed as JSON unless `--input-format=yaml` is provided."
    ));
    assert!(stdout.contains(
        "Command-based prerequisite checks default to `5 seconds` unless `timeout` is provided."
    ));
    assert!(stdout.contains("`sw check --input-file=-`"));
    assert!(stdout.contains("`--input-format json|yaml` for stdin-backed input"));
    assert!(stdout.contains("`--input-file=-` reads the runbook from stdin."));
    assert!(stdout.contains(
        "Without `--input-file=-`, `--input-format` keeps the existing file-backed defaults and does not override file-extension-based parsing."
    ));
    assert!(stdout.contains("Built-in prerequisite kinds include `java`"));
    assert!(stdout.contains(
        "Normal `Command` entries keep their separate `2 minutes` default timeout for `sw run`."
    ));
}

#[test]
fn explain_import_prints_current_import_contract() {
    let output = run(&["explain", "import"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("topic: import"));
    assert!(stdout.contains("availability: implemented"));
    assert!(stdout.contains("Default input file is `./README.md`."));
    assert!(stdout.contains("Default output file is `./sw-runbook.json`."));
    assert!(stdout.contains("Command: `sw import`"));
    assert!(stdout.contains("`0` for a successful import."));
    assert!(stdout.contains("Headings map to `Heading` entries"));
    assert!(
        stdout.contains(
            "Fenced code blocks without a recognized shell language tag remain `Markdown`"
        )
    );
    assert!(stdout.contains("Run `sw validate --input-file <generated-runbook>`"));
}

#[test]
fn explain_boundaries_between_help_example_and_explain_are_clear() {
    let output = run(&["explain", "help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Use `help` for exact flags and invocation syntax."));
    assert!(
        stdout.contains("Use `--version` or `version` when you need to identify the current build")
    );
    assert!(stdout.contains("Use `explain` when the question is about behavior, defaults, or which command to call next."));
    assert!(stdout.contains("If you need JSON shape examples, use `sw example <topic>` next."));
}

#[test]
fn explain_example_mentions_trim_empty_lines_in_command_example() {
    let output = run(&["explain", "example"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(
        "The `Command` example includes current nested output fields such as `trim_empty_lines` and `stream`."
    ));
    assert!(stdout.contains(
        "The `DisplayFile` example includes the Java `collapse_method_body` transform for collapsing method bodies."
    ));
    assert!(stdout.contains(
        "Use `sw example DisplayFile` when you need the Java `collapse_method_body` transform shape."
    ));
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

#[test]
fn explain_skill_prints_skill_document_to_stdout() {
    let output = run(&["explain", "--output-format=skill"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with(
        "---\nname: sw\ndescription: Use this skill when the user needs help understanding or operating the `sw` CLI.\n---\n\n# sw\n"
    ));
    assert!(stdout.contains("## Guidance"));
    assert!(stdout.contains("Start with `sw explain --all`."));
    assert!(stdout.contains(
        "Treat the CLI output and the documented specs as authoritative over any cached assumptions."
    ));
    assert!(!stdout.contains("## Command Map"));
    assert!(!stdout.contains("### run"));
}

#[test]
fn explain_skill_output_file_without_value_writes_to_default_codex_path() {
    let dir = prepare_workspace();
    let codex_home = dir.join(".codex-home");

    let output = run_in_dir_with_env(
        &["explain", "--output-format=skill", "--output-file"],
        &dir,
        &[("CODEX_HOME", &codex_home)],
    );

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let output_path = codex_home.join("skills/sw/SKILL.md");
    assert!(stdout.contains(&format!("Wrote explain skill to {}", output_path.display())));
    let skill = fs::read_to_string(&output_path).expect("missing skill output");
    assert!(skill.starts_with(
        "---\nname: sw\ndescription: Use this skill when the user needs help understanding or operating the `sw` CLI.\n---\n\n# sw\n"
    ));
    assert!(skill.contains("Start with `sw explain --all`."));
    assert!(!skill.contains("## Command Map"));
}

#[test]
fn explain_skill_output_file_with_explicit_path_writes_to_requested_location() {
    let dir = prepare_workspace();
    let output_path = dir.join("nested/custom-skill.md");
    let output_path_arg = format!("--output-file={}", output_path.display());

    let output = run_in_dir(
        &["explain", "--output-format=skill", &output_path_arg],
        &dir,
    );

    assert!(output.status.success());
    let skill = fs::read_to_string(&output_path).expect("missing skill output");
    assert!(skill.starts_with(
        "---\nname: sw\ndescription: Use this skill when the user needs help understanding or operating the `sw` CLI.\n---\n\n# sw\n"
    ));
    assert!(skill.contains("Start with `sw explain --all`."));
    assert!(!skill.contains("## Command Map"));
}

#[test]
fn explain_skill_refuses_to_overwrite_existing_file_without_force() {
    let dir = prepare_workspace();
    let output_path = dir.join("SKILL.md");
    fs::write(&output_path, "existing skill\n").expect("failed to seed skill file");
    let output_path_arg = format!("--output-file={}", output_path.display());

    let output = run_in_dir(
        &["explain", "--output-format=skill", &output_path_arg],
        &dir,
    );

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Refusing to overwrite existing explain skill output"));
    assert_eq!(
        fs::read_to_string(&output_path).expect("missing original skill"),
        "existing skill\n"
    );
}

#[test]
fn explain_skill_force_overwrites_existing_file() {
    let dir = prepare_workspace();
    let output_path = dir.join("SKILL.md");
    fs::write(&output_path, "existing skill\n").expect("failed to seed skill file");
    let output_path_arg = format!("--output-file={}", output_path.display());

    let output = run_in_dir(
        &[
            "explain",
            "--output-format=skill",
            &output_path_arg,
            "--force",
        ],
        &dir,
    );

    assert!(output.status.success());
    let skill = fs::read_to_string(&output_path).expect("missing overwritten skill");
    assert!(skill.starts_with(
        "---\nname: sw\ndescription: Use this skill when the user needs help understanding or operating the `sw` CLI.\n---\n\n# sw\n"
    ));
    assert!(!skill.contains("existing skill"));
}

#[test]
fn explain_skill_force_without_output_file_returns_usage_error() {
    let output = run(&["explain", "--output-format=skill", "--force"]);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("only accepts --force when --output-file is used"));
}
