# Repository Guidelines

## Change Delivery

Follow the canonical maintainer procedure in
[`docs/release/README.md`](docs/release/README.md) for preparing a branch,
verification, commit, pull request, merge, cleanup, and optional release.

### Trigger phrase

- `commit changes`
  - Authorizes the agent to perform the commit, push, and pull-request steps
    in the delivery guide.
  - Stage all staged and unstaged changes with `git add .` before committing.
  - Open a pull request to `main` with `gh pr create` when the GitHub CLI is
    available; otherwise report the exact command the user can run.
  - Do not delete local branches as part of this trigger.
  - Report the result in this format:
    - A short summary sentence, for example `Committed and pushed the current changes.`
    - `Verification run before commit:` followed by the non-git verification and build commands that were executed, listed as bullets in the order they were executed
    - `Commit:` with the short commit hash
    - `Branch:` with the branch name
    - `Remote:` with the pushed remote and branch
    - `Pull request:` with the PR URL, or the exact `gh pr create` command if a PR could not be opened
    - `Commit message used:` followed by the full commit message with a blank line between subject and body
    - If unrelated local changes were intentionally left uncommitted, list them under `I left unrelated local changes uncommitted:`

- `release changes as v<version>`
  - Authorizes the complete guarded release flow in the delivery guide,
    including merging the pull request, creating and pushing the specified tag,
    and verifying the GitHub Release and Homebrew tap update.
  - Requires an explicit version tag such as `v0.1.3`; never infer a release
    version from the change.
  - Stop and report the failure if local verification, required PR checks,
    merging, release publication, tap synchronization, or Homebrew upgrade
    verification fails.
  - Report the pull request, merge commit, tag, release URL, tap verification,
    and installed `sw version` when the flow succeeds.

### Authorization Boundaries

- Only create a commit when the user explicitly asks.
- Only push when the user explicitly asks.
- Treat `commit changes` as explicit permission to push the resulting branch
  and, when the branch is not `main`, open a pull request to `main`.
- Treat `release changes as v<version>` as explicit permission to merge the
  release pull request and push that exact release tag after the required
  checks pass.
- Do not push directly to `main`; branch protection requires the `Quality`
  check to run through a pull request.

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
