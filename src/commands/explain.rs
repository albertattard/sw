use crate::cli::{ExplainArgs, ExplainOutputFormat};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn run(args: ExplainArgs) -> ExitCode {
    match args.output_format {
        ExplainOutputFormat::Text => run_text(args),
        ExplainOutputFormat::Skill => run_skill(args),
    }
}

fn run_text(args: ExplainArgs) -> ExitCode {
    if args.output_file.is_some() {
        eprintln!("The explain command only accepts --output-file with --output-format=skill");
        return ExitCode::from(1);
    }

    if args.force {
        eprintln!("The explain command only accepts --force with --output-format=skill");
        return ExitCode::from(1);
    }

    if args.all {
        if let Some(topic) = args.topic {
            eprintln!("The explain command does not accept both a topic and --all: {topic}");
            return ExitCode::from(1);
        }

        for (index, topic) in topic_names().iter().enumerate() {
            if index > 0 {
                println!();
            }
            println!(
                "{}",
                explanation_for_topic(topic).expect("known explain topic")
            );
        }
        return ExitCode::SUCCESS;
    }

    let Some(topic) = args.topic else {
        eprintln!("The explain command requires a topic or --all");
        return ExitCode::from(1);
    };

    let Some(explanation) = explanation_for_topic(&topic) else {
        eprintln!(
            "Unknown explain topic: {}. Supported topics: {}",
            topic,
            topic_names().join(", ")
        );
        return ExitCode::from(1);
    };

    println!("{explanation}");
    ExitCode::SUCCESS
}

fn run_skill(args: ExplainArgs) -> ExitCode {
    if let Some(topic) = args.topic {
        eprintln!(
            "The explain command does not accept a topic with --output-format=skill: {topic}"
        );
        return ExitCode::from(1);
    }

    if args.all {
        eprintln!("The explain command does not accept --all with --output-format=skill");
        return ExitCode::from(1);
    }

    if args.force && args.output_file.is_none() {
        eprintln!("The explain command only accepts --force when --output-file is used");
        return ExitCode::from(1);
    }

    let skill_document = build_skill_document();

    let Some(output_file) = args.output_file else {
        println!("{skill_document}");
        return ExitCode::SUCCESS;
    };

    let path = match output_file_path(output_file) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };

    if path.exists() && !args.force {
        eprintln!(
            "Refusing to overwrite existing explain skill output: {}. Re-run with --force to overwrite it.",
            path.display()
        );
        return ExitCode::from(1);
    }

    if let Some(parent) = path.parent()
        && let Err(err) = fs::create_dir_all(parent)
    {
        eprintln!(
            "Failed to create parent directory for explain skill output {}: {err}",
            path.display()
        );
        return ExitCode::from(1);
    }

    if let Err(err) = fs::write(&path, skill_document) {
        eprintln!(
            "Failed to write explain skill output {}: {err}",
            path.display()
        );
        return ExitCode::from(1);
    }

    println!("Wrote explain skill to {}", path.display());
    ExitCode::SUCCESS
}

fn output_file_path(output_file: Option<PathBuf>) -> Result<PathBuf, String> {
    match output_file {
        Some(path) => Ok(path),
        None => default_skill_output_path(),
    }
}

fn default_skill_output_path() -> Result<PathBuf, String> {
    if let Some(codex_home) = env::var_os("CODEX_HOME")
        && !codex_home.is_empty()
    {
        return Ok(PathBuf::from(codex_home).join("skills/sw/SKILL.md"));
    }

    if let Some(home) = env::var_os("HOME")
        && !home.is_empty()
    {
        return Ok(PathBuf::from(home).join(".codex/skills/sw/SKILL.md"));
    }

    Err(
        "Could not determine the default Codex skill path because neither CODEX_HOME nor HOME is set."
            .to_string(),
    )
}

fn topic_names() -> Vec<&'static str> {
    explanations()
        .iter()
        .map(|explanation| explanation.topic)
        .collect()
}

fn explanation_for_topic(topic: &str) -> Option<String> {
    explanations()
        .into_iter()
        .find(|explanation| explanation.topic.eq_ignore_ascii_case(topic))
        .map(build_explanation)
}

fn build_skill_document() -> String {
    let lines = vec![
        "---".to_string(),
        "name: sw".to_string(),
        "description: Use this skill when the user needs help understanding or operating the `sw` CLI.".to_string(),
        "---".to_string(),
        "".to_string(),
        "# sw".to_string(),
        "".to_string(),
        "Use this skill when the user needs help understanding or operating the `sw` CLI.".to_string(),
        "".to_string(),
        "## Guidance".to_string(),
        "".to_string(),
        "- Start with `sw explain --all`.".to_string(),
        "- Treat the CLI output and the documented specs as authoritative over any cached assumptions."
            .to_string(),
    ];

    lines.join("\n")
}

fn explanations() -> Vec<Explanation<'static>> {
    vec![
        Explanation {
            topic: "help",
            availability: "implemented",
            purpose: "Show command usage, options, and subcommand discovery.",
            defaults: &[
                "Human-readable help only.",
                "`sw help --all` prints top-level help plus each implemented subcommand.",
                "`sw --version` and `sw version` print version/build identity for the current binary.",
            ],
            inputs: &[
                "`sw --help`",
                "`sw --version`",
                "`sw help`",
                "`sw help <subcommand>`",
                "`sw help --all`",
                "`sw [command] --help`",
                "`sw version`",
            ],
            outputs: &[
                "Usage text is written to stdout.",
                "Version/build identity text is written to stdout for version entry points.",
                "No files are changed.",
            ],
            exit_codes: &[
                "`0` when help prints successfully.",
                "`0` when version/build identity prints successfully.",
                "`1` for an unknown topic or help printing error.",
            ],
            constraints: &[
                "Use `help` for exact flags and invocation syntax.",
                "Use `--version` or `version` when you need to identify the current build before debugging behavior differences.",
                "Use `explain` when the question is about behavior, defaults, or which command to call next.",
            ],
            next: &[
                "Use this first when you need command-line syntax rather than feature guidance.",
                "Use `sw --version` or `sw version` first when you need to confirm which binary is running.",
                "If you need the product contract behind a command, use `sw explain <topic>` next.",
                "If you need runbook snippets, use `sw example <topic>` next. Examples default to YAML; use `--output-format json` when you need the JSON shape.",
            ],
        },
        Explanation {
            topic: "validate",
            availability: "implemented",
            purpose: "Check that a runbook is structurally valid without executing workflow steps.",
            defaults: &[
                "Default input candidates are `./sw-runbook.json`, `./sw-runbook.yaml`, and `./sw-runbook.yml`.",
                "Default output format is `human`.",
                "File-based authoring workflows default to YAML, while stdin input via `--input-file=-` defaults to JSON unless `--input-format=yaml` is provided.",
                "When `--input-file=-` is used, stdin is parsed as JSON unless `--input-format=yaml` is provided.",
            ],
            inputs: &[
                "`sw validate --input-file <runbook.{json|yaml|yml}>`",
                "`sw validate --input-file=-`",
                "`--input-format json|yaml` for stdin-backed input",
                "`--output-format human|json`",
            ],
            outputs: &[
                "Human output shows errors, warnings, and nearby offending blocks.",
                "JSON output returns `schema_version`, `valid`, `errors`, and `warnings`.",
                "The command performs no writes.",
            ],
            exit_codes: &[
                "`0` for a valid runbook.",
                "`2` for structural validation failures.",
                "`1` for missing files, unreadable files, invalid JSON syntax, invalid YAML syntax, or internal errors.",
            ],
            constraints: &[
                "Validation accepts JSON, YAML, and YML runbooks.",
                "`--input-file=-` reads the runbook from stdin.",
                "For stdin input, JSON is the default format and piped YAML requires `--input-format=yaml`.",
                "Without `--input-file=-`, `--input-format` does not replace the normal file lookup or file-extension-based parsing behavior.",
                "If more than one default runbook file exists, validation fails with an operational error and requires `--input-file`.",
                "`Markdown.contents`, `Command.commands`, `Patch.patch`, `Prerequisite.checks[*].contents`, and `Prerequisite.checks[*].commands` may be a single string or an array of strings.",
                "When those fields are provided as a string, validation treats that as shorthand for the existing line-array model.",
                "Validation checks schema and documented field rules only.",
                "Warnings do not make a runbook invalid.",
                "Use `validate` before `run` or `check` when the question is about input correctness.",
            ],
            next: &[
                "Use this when you need to know whether the runbook file itself is valid.",
                "Use `sw help validate` for flags and exact CLI syntax.",
                "Use `sw example <topic>` when you need a valid entry shape to copy.",
            ],
        },
        Explanation {
            topic: "run",
            availability: "implemented",
            purpose: "Execute a runbook in order and render the resulting documentation output.",
            defaults: &[
                "Running `sw` with no subcommand is equivalent to `sw run`.",
                "Default input candidates are `./sw-runbook.json`, `./sw-runbook.yaml`, and `./sw-runbook.yml`.",
                "Default output format is `markdown`.",
                "Default output file is `./README.md`.",
                "File-based authoring workflows default to YAML, while stdin input via `--input-file=-` defaults to JSON unless `--input-format=yaml` is provided.",
                "When `--input-file=-` is used, stdin is parsed as JSON unless `--input-format=yaml` is provided.",
                "Command output trims leading and trailing empty lines by default unless `output.trim_empty_lines` overrides it.",
                "`Command` entries default to a `30 seconds` timeout unless `timeout` is provided.",
            ],
            inputs: &[
                "`sw run --input-file <runbook.{json|yaml|yml}>`",
                "`sw --input-file=-` or `sw run --input-file=-` for stdin-backed JSON runbooks",
                "`--input-format json|yaml` for stdin-backed input",
                "`--output-format markdown`",
                "`--output-file <path>`",
                "`--verbose` for progress output on stderr",
                "`--verbose-mode auto|live|plain` to choose how verbose progress renders",
                "`--debug` for command rewrite and capture diagnostics on stderr",
            ],
            outputs: &[
                "Writes the generated document to the target output file.",
                "Prints human-readable status on stdout.",
                "May print progress or diagnostics on stderr when requested.",
            ],
            exit_codes: &[
                "`0` for successful execution and rendering.",
                "`2` for invalid runbook input or command execution failure.",
                "`1` for missing files, write failures, invalid JSON syntax, invalid YAML syntax, or internal errors.",
            ],
            constraints: &[
                "Run accepts JSON, YAML, and YML runbooks.",
                "`--input-file=-` reads the runbook from stdin.",
                "For stdin input, JSON is the default format and piped YAML requires `--input-format=yaml`.",
                "Without `--input-file=-`, `--input-format` keeps the existing file-backed defaults and does not override file-extension-based parsing.",
                "If more than one default runbook file exists, run fails with an operational error and requires `--input-file`.",
                "`Markdown.contents`, `Command.commands`, `Patch.patch`, `Prerequisite.checks[*].contents`, and `Prerequisite.checks[*].commands` may be a single string or an array of strings.",
                "When those fields are provided as a string, `sw run` splits them on newline boundaries before rendering or execution and does not turn a terminal line break into an extra blank line.",
                "`Command.cleanup` supports manual teardown as either a string or an array, and explicit `cleanup` replaces automatic process cleanup for that command entry.",
                "`Command.debug` enables diagnostics for one command entry, while global `--debug` enables diagnostics for all command entries.",
                "A command entry remains active until the command shell has exited and the captured stdout and stderr streams have both closed, so background processes that inherit those pipes can keep the entry open until they exit or the timeout is reached.",
                "Entries execute in runbook order.",
                "Command execution and rendering are part of the same workflow.",
                "Machine-readable contracts live in the runbook input, not in stdout.",
                "`Markdown`, `DisplayFile`, `Patch`, and `Command` entries may declare `indent` to prefix each non-empty rendered line and keep nested Markdown structures readable.",
                "`DisplayFile` fence detection recognizes `.java` as `java`, `.sql` as `sql`, and `.xml` as `xml`; other extensions fall back to `text`.",
                "Command output can render `stdout`, `stderr`, or `combined` with `output.stream`; if omitted, rendered output defaults to `combined`.",
                "Command output content types currently support `text`, `json`, `xml`, `html`, and `java`.",
                "`output.stream` changes rendered output only and does not widen capture or assertion sources.",
                "`datetime_shift.id` establishes a shared shift anchor, and `datetime_shift.use` reuses an earlier anchor so later rewrite rules follow the same synthetic timeline.",
                "Command output can be shortened with `output.rewrite` rule `type: limit_lines`, using `first`, `last`, and optional `show_trim_marker`.",
                "Rewrite `capture_as` creates `@{<capture_as>_original}` and `@{<capture_as>_rewritten}` variables so later entries can reuse the matched original and rewritten values.",
                "Use `datetime_shift.id`/`use` when you need timeline reuse; use rewrite `capture_as` when you need to reuse the rewritten timestamp text.",
                "Command output can trim outer empty lines with `output.trim_empty_lines` using `leading_trailing`, `leading`, `trailing`, or `none`.",
                "Markdown may interpolate captured variables with `@{name}` and preserve the literal syntax with `@@{name}`.",
                "Markdown interpolation may reference values captured earlier or later in the runbook.",
                "Command-based prerequisite checks use a shorter default timeout of `5 seconds` unless they declare `timeout`.",
                "`--verbose-mode=plain` is useful for SSH and wrapper-driven execution where in-place redraws are fragile.",
                "Prefer `output.rewrite` with `type: datetime_shift` over `replace` for semantic dates and times so relative timing stays intact.",
                "Use `replace` for dates and times only when the text is not semantically a date or time, or when `datetime_shift` cannot express the required format.",
            ],
            next: &[
                "Use this when you want the workflow executed and the README-style output generated.",
                "Use `sw validate` first if the question is whether the runbook shape is valid.",
                "Use `sw check` first if the question is whether prerequisites such as Java are available.",
                "Use `sw example Command` when you need a current YAML snippet for output fields such as `trim_empty_lines` and `stream`, or add `--output-format json` when the JSON shape is what you need.",
            ],
        },
        Explanation {
            topic: "check",
            availability: "implemented",
            purpose: "Validate the runbook and execute only prerequisite checks to confirm the environment is ready.",
            defaults: &[
                "Default input candidates are `./sw-runbook.json`, `./sw-runbook.yaml`, and `./sw-runbook.yml`.",
                "File-based authoring workflows default to YAML, while stdin input via `--input-file=-` defaults to JSON unless `--input-format=yaml` is provided.",
                "When `--input-file=-` is used, stdin is parsed as JSON unless `--input-format=yaml` is provided.",
                "Command-based prerequisite checks default to `5 seconds` unless `timeout` is provided.",
            ],
            inputs: &[
                "`sw check --input-file <runbook.{json|yaml|yml}>`",
                "`sw check --input-file=-`",
                "`--input-format json|yaml` for stdin-backed input",
            ],
            outputs: &[
                "Prints human-readable prerequisite status.",
                "Does not render or write `README.md`.",
                "Does not execute normal `Command` entries.",
            ],
            exit_codes: &[
                "`0` when the runbook is valid and all prerequisites pass.",
                "`2` when the runbook is valid but a prerequisite fails.",
                "`1` for missing files, invalid runbooks, or other operational errors.",
            ],
            constraints: &[
                "Check accepts JSON, YAML, and YML runbooks.",
                "`--input-file=-` reads the runbook from stdin.",
                "For stdin input, JSON is the default format and piped YAML requires `--input-format=yaml`.",
                "Without `--input-file=-`, `--input-format` keeps the existing file-backed defaults and does not override file-extension-based parsing.",
                "If more than one default runbook file exists, check fails with an operational error and requires `--input-file`.",
                "`Markdown.contents`, `Command.commands`, `Patch.patch`, `Prerequisite.checks[*].contents`, and `Prerequisite.checks[*].commands` may be a single string or an array of strings.",
                "The runbook is validated before prerequisite execution begins.",
                "Checks run in runbook order and stop on the first failing prerequisite.",
                "Built-in prerequisite kinds include `java`, including version rules such as `21` or `21+`.",
                "Normal `Command` entries keep their separate `30 seconds` default timeout for `sw run`.",
            ],
            next: &[
                "Use this when the question is about execution readiness, such as \"how do I check for Java 21?\"",
                "Use `sw help check` for exact flags and invocation syntax.",
                "Use `sw example Prerequisite` when you need a current YAML snippet for prerequisite checks, or add `--output-format json` when the JSON shape is what you need.",
            ],
        },
        Explanation {
            topic: "init",
            availability: "implemented",
            purpose: "Generate a starter runbook file from a realistic sample.",
            defaults: &[
                "Default output file is `./sw-runbook.yaml`.",
                "YAML is the default file-based starter format.",
                "Existing output files are not overwritten unless `--force` is provided.",
            ],
            inputs: &[
                "Command: `sw init`",
                "Options: `--output-file <path>` and `--force`",
            ],
            outputs: &[
                "Writes a starter runbook YAML or JSON file and prints a short stdout confirmation.",
            ],
            exit_codes: &[
                "`0` for successful creation.",
                "`1` for write failures or existing files without `--force`.",
            ],
            constraints: &[
                "The sample is intended as a realistic starting point, not the smallest possible runbook.",
                "When `--output-file` is omitted, init defaults to YAML.",
                "Init infers JSON or YAML from a recognized output-file extension.",
                "Init rejects unsupported output-file extensions instead of guessing a format.",
                "The sample includes `Heading`, `Markdown`, `DisplayFile`, `Prerequisite`, and `Command` entries.",
                "The generated sample is expected to remain valid according to the current runbook contract.",
            ],
            next: &[
                "Use this topic when you need the starter-runbook workflow and overwrite behavior.",
                "Run `sw validate --input-file <generated-runbook>` if you want an explicit post-generation validation check.",
                "Use `sw example <topic>` when you need smaller entry-level snippets instead of a full starter runbook.",
            ],
        },
        Explanation {
            topic: "import",
            availability: "implemented",
            purpose: "Import an existing Markdown README into a starter runbook file.",
            defaults: &[
                "Default input file is `./README.md`.",
                "Default output format is `yaml`.",
                "Default output file is `./sw-runbook.yaml`.",
                "YAML is the default file-based import format.",
                "Existing output files are not overwritten unless `--force` is provided.",
            ],
            inputs: &[
                "Command: `sw import`",
                "Options: `--input-file <README.md>`, `--output-file <runbook.{yaml|yml|json}>`, `--output-format json|yaml`, and `--force`",
            ],
            outputs: &[
                "Writes a best-effort runbook YAML or JSON file and prints a short stdout confirmation.",
            ],
            exit_codes: &[
                "`0` for a successful import.",
                "`1` for missing input, write failures, validation failures, or existing output without `--force`.",
            ],
            constraints: &[
                "Import is intentionally lossy and produces an editable starting point rather than a perfect round trip.",
                "When `--output-format` is omitted, `sw import` infers the format from a recognized output-file extension or defaults to YAML.",
                "If `--output-format` conflicts with a recognized output-file extension, `sw import` exits with `1` instead of writing a mismatched file.",
                "Headings map to `Heading` entries where possible, prose to `Markdown`, and fenced shell blocks to `Command` entries.",
                "Imported entries serialize `type` before entry-specific fields so the generated runbook is easier to scan.",
                "YAML import output separates adjacent `entries` items with a blank line and emits imported multi-line `Markdown.contents` with `|` block scalars.",
                "Fenced code blocks without a recognized shell language tag remain `Markdown` so the importer does not guess execution semantics.",
            ],
            next: &[
                "Use this topic when you need the current README-to-runbook conversion contract.",
                "Run `sw validate --input-file <generated-runbook>` if you want an explicit post-import validation check.",
                "Use `sw example <topic>` when you need current YAML snippets for manual authoring after the import, or add `--output-format json` when the JSON shape is what you need.",
            ],
        },
        Explanation {
            topic: "example",
            availability: "implemented",
            purpose: "Print a focused runbook snippet for a supported runbook topic.",
            defaults: &[
                "Default output format is `yaml`.",
                "Entity-type matching is case-insensitive.",
                "There is no aggregate mode in this increment.",
            ],
            inputs: &[
                "`sw example <entity-type>`",
                "`--output-format yaml|json`",
                "Supported topics currently include `Command`, `DisplayFile`, `Patch`, and `Prerequisite`.",
            ],
            outputs: &[
                "Writes a single YAML snippet to stdout by default.",
                "Writes JSON when `--output-format json` is selected.",
                "Writes a human-readable error to stderr for unsupported topics.",
            ],
            exit_codes: &[
                "`0` when the snippet is printed successfully.",
                "`1` for unknown topics or operational errors.",
            ],
            constraints: &[
                "Example output is documentation-oriented and may need editing before use.",
                "Use `example` for snippet shape, not for command behavior or defaults.",
                "The `Command` example includes current nested fields such as `trim_empty_lines`, `stream`, `cleanup`, and `debug`.",
                "The `DisplayFile` example includes the Java `collapse_method_body` transform for collapsing method bodies.",
                "The printed snippet is intended to remain a stable starting point for users and agents.",
            ],
            next: &[
                "Use this when the question is \"what runbook snippet should I write?\"",
                "Use `sw example DisplayFile` when you need the Java `collapse_method_body` transform shape.",
                "Use `sw explain <topic>` when the question is about behavior, defaults, or command boundaries.",
                "Use `sw help example` for exact invocation syntax.",
            ],
        },
    ]
}

#[derive(Clone, Copy)]
struct Explanation<'a> {
    topic: &'a str,
    availability: &'a str,
    purpose: &'a str,
    defaults: &'a [&'a str],
    inputs: &'a [&'a str],
    outputs: &'a [&'a str],
    exit_codes: &'a [&'a str],
    constraints: &'a [&'a str],
    next: &'a [&'a str],
}

fn build_explanation(explanation: Explanation<'_>) -> String {
    let mut lines = vec![
        format!("topic: {}", explanation.topic),
        format!("availability: {}", explanation.availability),
        format!("purpose: {}", explanation.purpose),
    ];
    push_section(&mut lines, "defaults", explanation.defaults);
    push_section(&mut lines, "inputs", explanation.inputs);
    push_section(&mut lines, "outputs", explanation.outputs);
    push_section(&mut lines, "exit_codes", explanation.exit_codes);
    push_section(&mut lines, "constraints", explanation.constraints);
    push_section(&mut lines, "next", explanation.next);
    lines.join("\n")
}

fn push_section(lines: &mut Vec<String>, name: &str, values: &[&str]) {
    lines.push(format!("{name}:"));
    for value in values {
        lines.push(format!("- {value}"));
    }
}
