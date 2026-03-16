# Repository Guidelines

## Trigger phrases
- `commit changes`
  - Run the formatting check
  - Run the lint checks
  - Run the automated tests
  - Build the release binary
  - Stage all staged and unstaged changes with `git add .`
  - Create a commit using the commit message format in this file
  - Push the commit to `origin` on the current branch
  - Report the result in this format:
    - A short summary sentence, for example `Committed and pushed the current changes.`
    - `Verification run before commit:` followed by the non-git verification and build commands that were executed, listed as bullets in the order they were executed
    - `Commit:` with the short commit hash
    - `Branch:` with the branch name
    - `Remote:` with the pushed remote and branch
    - `Commit message used:` followed by the full commit message with a blank line between subject and body
    - If unrelated local changes were intentionally left uncommitted, list them under `I left unrelated local changes uncommitted:`

## Git workflow
- Only create a commit when the user explicitly asks.
- Only push when the user explicitly asks.
- Before `commit changes`, run `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`, and `cargo build --release`.
- If formatting, linting, tests, or the release build fail, stop and report the failure instead of committing.
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
- Follow spec-driven delivery for user-visible features.
- For a new feature or behavior change: define or update the spec first, then create or update the task, then implement the change.
- Default workflow unless the user says otherwise:
  - Define or update the spec
  - Create or update the task
  - Commit the spec/task slice when the user asks to commit
  - Implement the code afterward so it aligns with the committed spec
- Experimental workflow when the user is explicitly trying things out first:
  - Build the code change first
  - Let the user try it
  - Then define or update the spec
  - Commit the spec/task slice when the user asks to commit
  - Then finalize or adjust the code so it aligns with the spec
- Treat the spec as the source of truth for user-visible behavior.
- If implementation and spec differ, treat the implementation as wrong until the spec is intentionally updated.
- Do not change user-visible behavior without updating the relevant spec and task.
- Treat specs as living documents and tasks as bounded delivery slices.
- New task files must include a `category` field in front matter.
- New task files must live under `docs/tasks/<category>/`.
- Keep task filenames unchanged when moving tasks between directories.
- Keep `docs/tasks/README.md` as the root entry point and index for all task files.
- Use one of these controlled task categories when creating new tasks:
  - `discovery`
  - `validate`
  - `format`
  - `run`
  - `rewrite`
  - `prerequisite`
  - `display-file`
  - `example`
  - `explain`
  - `init`
  - `import`
  - `release`
  - `repo-process`
- When a spec grows, prefer creating a new task for the new increment instead of reopening an already completed task.
- Keep completed tasks as historical records unless they were tracked incorrectly.
- Do not mark a task as done or check its acceptance criteria until the implementation and verification for that task have actually been completed.
- Keep task status aligned with the actual implementation state.
- Keep `src/main.rs` thin. Use it as the entrypoint and command dispatcher, not as the place for business logic.
- Add new CLI subcommands under `src/commands/`.
- Keep runbook parsing, modelling, and validation outside `main.rs`.
- Prefer self-describing long CLI option names such as `--input-file`, `--output-format`, and `--output-file`.
- Avoid short or ambiguous option names such as `--file` and `--output` unless there is a clear reason to add an alias.
- Add or update automated tests for every user-visible CLI feature.
- Prefer integration-style CLI tests for command behavior, exit codes, and output contracts.
- Treat machine-readable CLI output as a stable contract for users and agents. Avoid unnecessary breaking changes.
- Prefer discovery and introspection commands that are easy for both humans and agents to consume.
- When exposing repository knowledge through the CLI, keep default output concise, stable, and suitable for programmatic use, while allowing fuller output when needed.
- Keep CLI help output aligned with the implemented commands, options, defaults, and current feature set.
- Any user-visible CLI change must update help text and help-focused tests in the same change.
- Keep implementation aligned with the documented spec when behavior changes.
- Reject unknown runbook fields by default unless the format is intentionally expanded and documented.
- Refactor toward clearer module boundaries before adding complexity to an already crowded file.
