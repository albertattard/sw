# Repository Guidelines

## Trigger phrases
- `commit changes`
  - Stage all staged and unstaged changes with `git add .`
  - Create a commit using the commit message format in this file
  - Push the commit to `origin` on the current branch

## Git workflow
- Only create a commit when the user explicitly asks.
- Only push when the user explicitly asks.
- Write commit messages in this format:
  - Subject line: imperative verb + outcome
  - Example: `Add validate subcommand`
- After the subject, include a short business-oriented description explaining why the change was made.
- Focus the description on product intent, maintainability, usability, or future growth, not on low-level code mechanics.
- The description may mention structural decisions when they support future capabilities.
- Avoid code-centric summaries as the primary explanation.

Example:

```text
Add validate subcommand

Allow users and agents to verify that a runbook is structurally valid before
trying to execute it. This also reorganizes the CLI so new commands can be
added cleanly as the application grows, instead of concentrating behavior in
main.rs. An anonymised fixture is included to support realistic testing without
tying the test suite to a specific project.
```

## Engineering rules
- Keep `src/main.rs` thin. Use it as the entrypoint and command dispatcher, not as the place for business logic.
- Add new CLI subcommands under `src/commands/`.
- Keep runbook parsing, modelling, and validation outside `main.rs`.
- Prefer self-describing long CLI option names such as `--input-file`, `--output-format`, and `--output-file`.
- Avoid short or ambiguous option names such as `--file` and `--output` unless there is a clear reason to add an alias.
- Add or update automated tests for every user-visible CLI feature.
- Prefer integration-style CLI tests for command behavior, exit codes, and output contracts.
- Treat machine-readable CLI output as a stable contract for users and agents. Avoid unnecessary breaking changes.
- Keep implementation aligned with the documented spec when behavior changes.
- Reject unknown runbook fields by default unless the format is intentionally expanded and documented.
- Refactor toward clearer module boundaries before adding complexity to an already crowded file.
