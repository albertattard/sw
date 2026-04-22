---
id: SPEC-003
title: Run Runbook to Markdown
status: in_progress
priority: high
owner: @aattard
last_updated: 2026-04-22
---

## Problem

Users and AI agents need a simple way to turn a validated runbook into a
readable Markdown document while also verifying that the documented commands
complete successfully.

## User-facing Behavior

The CLI defaults to the `run` command. Invoking `sw` with no subcommand is
equivalent to:

```bash
sw run
```

The CLI also provides an explicit `run` command:

```bash
sw
sw --verbose
sw --debug
sw --input-file=-
sw run
sw run --input-file <sw-runbook.yaml>
sw run --input-file=- --input-format=yaml
```

Initial version behavior:
- Use the implicit default runbook only when exactly one of
  `sw-runbook.json`, `sw-runbook.yaml`, or `sw-runbook.yml` exists in the
  current directory.
- If more than one default runbook file exists in the current directory, fail
  with an operational error and require `--input-file`.
- Render the runbook entries in order.
- Execute command entries in order.
- Produce Markdown output.
- Write the generated document to `./README.md` by default.

This command generates documentation output and executes the commands declared
in the runbook.

## Inputs

- Optional named input file parameter: `--input-file <runbook.{json|yaml|yml}>`
  or `--input-file=-` to read the runbook from stdin.
- Optional input format parameter: `--input-format json|yaml`.
- Optional output format parameter: `--output-format markdown`.
- Optional output file parameter: `--output-file <path>`.
- Optional progress parameter: `--verbose`.
- Optional verbose progress mode parameter: `--verbose-mode auto|live|plain`.
- Optional diagnostic parameter: `--debug`.
- `--verbose` may be provided either before the subcommand as a global flag or
  after the `run` subcommand.
- `--verbose-mode` may be provided either before the subcommand as a global
  flag or after the `run` subcommand.
- `--debug` may be provided either before the subcommand as a global flag or
  after the `run` subcommand.

### CLI Defaults

- File-based runbook creation flows default to YAML, but stdin-backed runbook
  input keeps JSON as the default machine-oriented format.
- If `--input-file=-` is provided, read the runbook from stdin.
- If `--input-file=-` is provided and `--input-format` is omitted, parse stdin
  as JSON.
- If `--input-file=-` is provided and `--input-format=yaml`, parse stdin as
  YAML.
- If `--input-file` is provided with a path, use that path.
- If `--input-file` is omitted and exactly one of `./sw-runbook.json`,
  `./sw-runbook.yaml`, or `./sw-runbook.yml` exists, use that file.
- If `--input-file` is omitted and more than one of `./sw-runbook.json`,
  `./sw-runbook.yaml`, or `./sw-runbook.yml` exists, return exit code `1` with
  a clear error that the default input is ambiguous and `--input-file` must be
  specified.
- If `--input-file` is omitted, `--input-format` does not bypass this default
  file ambiguity check.
- When reading from a file path or from the default file lookup, infer the
  input format from the file extension or default file name.
- If `--output-format` is not provided, default to `markdown`.
- If `--output-file` is not provided, default to `./README.md`.
- If `--verbose` is not provided, progress output is suppressed.
- If `--verbose-mode` is not provided, it defaults to `auto`.
- If `--verbose` is not provided, `--verbose-mode` has no effect.
- If `--debug` is not provided, diagnostic output is suppressed.
- If `sw` is invoked without a subcommand, `sw --verbose` behaves the same as
  `sw run --verbose`.
- If `sw` is invoked without a subcommand, `sw --verbose-mode=<mode>` behaves
  the same as `sw run --verbose-mode=<mode>`.
- If `sw` is invoked without a subcommand, `sw --debug` behaves the same as
  `sw run --debug`.
- If a `Command` entry omits `timeout`, it defaults to `2 minutes`.

## Outputs

- Generated Markdown file written to the target path.
- Human-readable status on stdout.
- Optional human-readable progress output on stderr when `--verbose` is used.
- Optional diagnostic trace output on stderr when `--debug` is used.

### Exit Codes

- `0`: runbook executed and rendered successfully.
- `1`: operational error (missing file, unreadable file, invalid JSON, invalid
  YAML, write
  failure, internal error).
- `2`: invalid runbook input or command execution failure.

### Supported Output Formats

- `markdown`

### Verbose Progress Output

- `sw run --verbose` emits one progress line before each runbook entry begins.
- `sw --verbose` behaves the same as `sw run --verbose`.
- `sw run --verbose-mode=auto` is the default verbose progress mode.
- `sw run --verbose-mode=live` forces in-place timer updates.
- `sw run --verbose-mode=plain` forces SSH-safe line-based progress output.
- Verbose progress output is written to stderr.
- Verbose progress output is intended to help humans and agents follow
  long-running execution without changing the stable stdout contract.
- Each verbose line includes the entry position, the total number of entries,
  the entry type, and a short summary.
- The current entry number is left-padded with spaces so all entry numbers
  align to the width of the total entry count.
- `Markdown` entry summaries use the first non-empty content line.
- `Command` entry summaries use the first non-empty command line.
- `Heading` entry summaries use the title.
- `DisplayFile` entry summaries use the file path and any declared line range.
- `Prerequisite` entry summaries identify that prerequisite checks are being
  processed.
- Summaries are single-line and may be truncated for readability.
- Verbose progress output does not print full long-form Markdown content or
  full multi-line command blocks.
- When stderr is a TTY, the current entry line includes a live elapsed timer
  that updates in place while that entry is running.
- In `plain` mode, verbose output always uses line-based progress even when
  stderr is a TTY.
- In `plain` mode, verbose output prints a start line when an entry begins.
- In `plain` mode, verbose output prints a completion line when an entry
  finishes, including the elapsed time and timeout window when applicable.
- Elapsed time is formatted for readability:
  - under one minute: seconds with one decimal place such as `12.4s`
  - one minute or more: minutes and seconds such as `1m 8s`
- `Command` entry progress includes an expected timeout window in addition to
  elapsed time, using the declared timeout or the default timeout when no
  timeout is set.
- For example, a running command entry may render as
  `[16/75] Command: java -jar './target/app.jar' (1m 8s / 2m)`.
- A running timer does not use a trailing `...`; the fact that the value keeps
  changing is sufficient to show that the entry is still active.
- When an entry finishes, the final elapsed time remains on the completed line.
- When stderr is not a TTY, verbose progress falls back to non-live line-based
  output and does not attempt in-place timer updates.
- `auto` mode chooses live progress only when stderr is a TTY and otherwise
  falls back to plain line-based progress.

### Debug Diagnostic Output

- `sw run --debug` emits diagnostic execution details to stderr.
- `sw --debug` behaves the same as `sw run --debug`.
- Debug output is additive and must not change the stable stdout contract.
- Debug output may be used together with `--verbose`.
- When both flags are enabled, both progress and debug diagnostics are written
  to stderr.
- `Command` entries may declare `debug`.
- `Command.debug` is a boolean.
- If `Command.debug` is omitted, it defaults to `false`.
- When global `--debug` is enabled, all `Command` entries emit debug
  diagnostics regardless of `Command.debug`.
- When global `--debug` is not enabled, only `Command` entries with
  `debug: true` emit debug diagnostics.
- `Command.debug` applies only to the command entry where it is declared and
  does not enable diagnostics for other entries.
- Debug output is intended to help humans and agents troubleshoot runbook
  authoring issues such as rewrite matching, generated captures, and command
  output interpolation.
- For `Command` entries, debug output may include:
  - raw stdout before rewrites
  - rewritten stdout after rewrites
  - rewrite rules after variable interpolation
  - rewrite match counts
  - explicit capture values
  - generated `capture_as` values
- Debug output should identify which command entry and which rewrite or capture
  rule produced the diagnostic information.
- Debug output is for troubleshooting and does not need to be a stable
  machine-readable contract.

## Rendering Rules

### Common Rules

- Generated Markdown begins with the comment
  `<!-- Generated by Sociable Weaver (https://github.com/albertattard/sw). Manual edits will be overwritten when regenerated. -->`.
- Entries are rendered in the same order as they appear in the runbook.

### Heading Entries

- `Heading` entries render as Markdown headings based on their `level`.

### Markdown Entries

- `Markdown` entries copy their `contents` into the output in order.
- `Markdown.contents` may be either a single string or an array of strings.
- When `Markdown.contents` is a string, it is normalized into the existing
  line-array model by splitting on newline boundaries before rendering.
- Scalar `Markdown.contents` ignores a terminal line break that exists only to
  terminate the scalar, so YAML literal scalars do not introduce an extra
  blank line before the next runbook entry.
- `Markdown` may declare `indent`.
- If `indent` is present, it is a non-negative integer.
- `indent` applies to the whole rendered Markdown section after interpolation.
- Each non-empty rendered Markdown line is prefixed with that many spaces.
- Empty lines remain empty.

### DisplayFile Entries

- `DisplayFile` entries copy the contents of the referenced file into the
  generated Markdown as a fenced code block.
- `DisplayFile.path` is resolved relative to the runbook location.
- `DisplayFile` may declare `start_line`.
- `DisplayFile` may declare `line_count`.
- `DisplayFile` may declare `indent`.
- `DisplayFile` may declare `offset`.
- `start_line` is 1-based.
- If `start_line` is omitted, rendering starts from line 1.
- If `start_line` is present and `line_count` is omitted, rendering continues
  from `start_line` to the end of the file.
- If `line_count` is present, only that many lines are rendered starting from
  `start_line`.
- If `indent` is present, it is a non-negative integer.
- `indent` applies to the whole rendered fenced block, including the opening
  fence, inner content lines, and closing fence.
- If `offset` is present, it is an integer.
- `offset` applies only to the copied file content lines inside the fenced
  block.
- If `offset` is positive, each non-empty copied file content line is prefixed
  with that many spaces.
- If `offset` is negative, up to that many leading spaces are removed from each
  non-empty copied file content line.
- Empty copied file content lines are preserved when `offset` is applied.
- If `offset` is negative and one or more non-empty copied file content lines
  have fewer leading spaces than requested, validation may emit a non-blocking
  warning that the negative offset cannot be fully applied to all lines.
- `line_count` without `start_line` is invalid.
- `DisplayFile` rendering does not execute the referenced file.
- `DisplayFile` fenced blocks use a detected content type when the file
  extension is recognized.
- In this increment, recognized `DisplayFile` extensions include `.java`,
  which renders as `java`, `.sql`, which renders as `sql`, and `.xml`, which
  renders as `xml`.
- If the `DisplayFile` extension is not recognized, the generated Markdown
  uses a `text` fenced block.

### Prerequisite Entries

- `Prerequisite` entries render prerequisite documentation and verify the
  runtime environment before the rest of the runbook proceeds.
- A `Prerequisite` entry is a single runbook entry that can group one or more
  prerequisite checks in its `checks` array.
- Each prerequisite check declares a short `name`.
- Each prerequisite check declares a `kind`.
- In this increment, supported prerequisite check kinds are `command` and
  `java`.
- Each prerequisite check declares `contents` as either a string or an array
  of Markdown lines.
- When prerequisite `contents` is a string, it is normalized into the existing
  line-array model by splitting on newline boundaries before rendering.
- Scalar prerequisite `contents` ignores a terminal line break that exists
  only to terminate the scalar, so YAML literal scalars do not introduce an
  extra blank line before the next runbook entry.
- `contents` is rendered into the generated Markdown in the declared order.
- A `command` prerequisite check declares `commands` as an array of command
  lines.
- `Prerequisite.checks[*].commands` may also be a single string.
- When prerequisite `commands` is a string, it is normalized into the existing
  line-array model by splitting on newline boundaries before execution.
- Scalar prerequisite `commands` ignores a terminal line break that exists
  only to terminate the scalar, so YAML literal scalars do not add an extra
  blank command line.
- All lines within a single `command` prerequisite check execute together in
  the same shell context.
- A `command` prerequisite check may declare `timeout`.
- If a `command` prerequisite check omits `timeout`, it defaults to
  `5 seconds`.
- A `command` prerequisite check may declare `assert` using the same structure
  as `Command` assertions.
- A `java` prerequisite check performs a built-in Java runtime validation
  without requiring runbook-authored shell parsing.
- A `java` prerequisite check declares `version`.
- `version` supports an exact major version such as `17`.
- `version` supports a minimum major version such as `24+`.
- If a `java` prerequisite check omits both `java_home` and `java_home_env`,
  the Java executable is resolved from `PATH`.
- A `java` prerequisite check may declare `java_home`.
- `java_home` is a literal path to a Java home directory.
- A `java` prerequisite check may declare `java_home_env`.
- `java_home_env` names an environment variable whose value must resolve to a
  Java home directory.
- `java_home` and `java_home_env` are mutually exclusive.
- A `java` prerequisite check validates Java by executing the resolved
  `<java-home>/bin/java` or `java` from `PATH`.
- A `java` prerequisite check fails if the Java executable cannot be resolved.
- A `java` prerequisite check fails if the resolved Java major version does not
  satisfy `version`.
- Each prerequisite check may declare `assert` using the same structure as
  `Command` assertions.
- Each prerequisite check may declare `help` as a human-readable remediation
  message.
- Prerequisite checks execute before normal runbook command entries.
- If any prerequisite check fails, the run stops before executing the main
  workflow.
- A failed prerequisite check is reported as a run failure with exit code `2`.

### Patch Entries

- `Patch` entries apply a textual patch to a target file during runbook
  execution.
- `Patch.path` is resolved relative to the runbook location.
- `Patch` entries declare `patch` as an array of patch lines.
- Patch lines are executed as one patch application step in the declared
  order.
- `Patch` entries render their patch contents as fenced `diff` blocks in the
  generated Markdown.
- A `Patch` entry may declare `indent`.
- If `indent` is present, the rendered patch section is prefixed with that
  number of leading spaces on each rendered line.
- `Patch` entries are executed in order with other runbook entries.
- `Patch` entries default to `restore: auto`.
- `restore: auto` snapshots the original target file contents before the first
  patch touching that file is applied in a run.
- Patch application is non-interactive.
- If a patch cannot be applied cleanly, the run fails instead of waiting for
  interactive input from the patch tool.
- Patch application failures must not leave `.orig` or `.rej` sidecar files
  behind in the working tree.
- `restore: auto` restores the original pre-run contents after the run
  completes successfully and also when the run stops early because of failure
  or timeout.
- If multiple `Patch` entries modify the same file, they may build on each
  other during the run.
- Automatic patch restoration unwinds in reverse patch-application order.
- When multiple `Patch` entries modify the same file, restoration returns that
  file to its original pre-run contents rather than to an intermediate state.
- Automatic patch restoration is best-effort and continues unwinding later
  registered patch restores even if an earlier restore step fails.
- A run with one or more patch restore failures is considered failed.

### Command Entries

- `Command` entries render their `commands` as fenced shell code blocks.
- `Command.commands` may be either a single string or an array of strings.
- When `Command.commands` is a string, it is normalized into the existing
  line-array model by splitting on newline boundaries before rendering and
  execution.
- Scalar `Command.commands` ignores a terminal line break that exists only to
  terminate the scalar, so YAML literal scalars do not add an extra blank
  command line.
- A `Command` entry may declare `working_dir`.
- If `working_dir` is present, it must be a string.
- `working_dir` is resolved relative to the runbook file's directory.
- `working_dir` applies to command execution, that command entry's explicit
  cleanup block, and file assertions for that command entry.
- If `working_dir` is omitted, command execution uses the runbook file's
  directory.
- `working_dir` must remain within the runbook file's directory tree after
  normalization.
- Absolute `working_dir` paths are not supported in this increment.
- If `working_dir` is present, the rendered command block wraps the displayed
  command in a subshell that changes into that directory before executing the
  command text.
- The rendered wrapper must be copy-pasteable as shell input.
- The rendered wrapper should use `cd '<working_dir>' &&` so the displayed
  command fails safely if the directory change fails.
- A `Command` entry may declare `indent`.
- If `indent` is present, the rendered command section is prefixed with that
  number of leading spaces on each rendered line.
- The `indent` value applies to the command block, any output caption, and any
  rendered command output block.
- `Command` entries are executed in order.
- All lines within a single `Command` entry execute together in the same shell
  context.
- A `Command` entry may declare `cleanup`.
- `cleanup` may be either a single string or an array of strings, matching the
  accepted shape of `commands`.
- When `cleanup` is a string, it is normalized into the existing line-array
  model by splitting on newline boundaries before execution.
- Scalar `cleanup` ignores a terminal line break that exists only to terminate
  the scalar, so YAML literal scalars do not add an extra blank cleanup line.
- All lines within a single `cleanup` block execute together in the same shell
  context.
- A `Command` entry may declare `debug`.
- Multi-line shell control structures such as `if ... then ... fi` may be
  expressed across multiple `cleanup` lines and must execute correctly as one
  cleanup script.
- `cleanup` is used after execution in order to release resources started by
  the main command.
- `cleanup` is optional.
- If `cleanup` is omitted, the runtime automatically terminates remaining
  processes started by that command entry after the entry finishes and also
  when the run stops early because of failure or timeout.
- Automatic process cleanup applies only to processes started by that command
  entry, not to unrelated system processes.
- If automatic process cleanup finds that the target process or process group
  has already exited, that is treated as a successful no-op.
- Automatic process cleanup must not surface a noisy "No such process" message
  when the target is already gone.
- Cleanup commands are executed after the run finishes and also when the run
  stops early because of failure.
- If `cleanup` is present, the explicit cleanup block is used instead of
  automatic process cleanup for that command entry.
- Cleanup commands execute in reverse order of the commands that registered
  them.
- If commands `A`, `B`, and `C` each register cleanup, then cleanup runs in the
  order `C`, `B`, `A`.
- If a cleanup block contains multiple command lines, those lines are attempted
  in the order declared.
- If one cleanup line fails, the remaining lines in that cleanup block still run.
- If one cleanup block fails, the remaining registered cleanup blocks still run
  in reverse order.
- Cleanup failures are reported after cleanup execution completes.
- A run with one or more cleanup failures is considered failed.
- A `Command` entry may declare a `timeout`.
- If `timeout` is omitted, the default timeout is `2 minutes`.
- `timeout` is expressed in human-readable form as a number followed by a unit.
- In this increment, supported units are `seconds`, `minutes`, and their common
  singular or abbreviated forms.
- A `Command` entry may include a `preconditions` section.
- `preconditions.checks` is an array of precondition checks.
- Each precondition check declares a `source` so the contract can support
  different command-start readiness checks over time.
- Preconditions execute before the command starts.
- If any precondition check fails, the command body does not execute and the
  run fails.
- `preconditions` supports `checks` only in this increment; it does not
  support an `exit_code` field.
- In this increment, supported precondition check sources are `port`.
- A `port` check declares a single integer `port`.
- A `port` check declares `free: true`.
- A `port` check targets TCP listener availability on the local machine.
- A `port` `free` check passes when no process is listening on that TCP port
  on any local interface.
- A `port` value must be within the inclusive range `1` through `65535`.
- To check more than one port, declare multiple checks rather than an array of
  ports inside one check.
- If a command does not finish within its timeout, the command process is
  terminated and the run fails.
- A `Command` entry without an `assert` section must exit with code `0` for the
  run to continue.
- A `Command` entry may include an `assert` section.
- If `assert.exit_code` is present, the command result is checked against that
  expected exit code instead of the default success requirement.
- If `assert.checks` is present, each declared check must pass for the command
  to be considered successful.
- `assert.checks` is an array of assertion checks.
- Each assertion check declares a `source` so the contract can support
  different targets such as command output and files.
- In this increment, supported assertion check sources are `stdout`, `file`,
  and `port`.
- A `stdout` check declares `contains`.
- A `stdout` `contains` check passes when the command stdout includes the
  expected text.
- A `file` check declares `path`.
- A `file` assertion path is resolved from the same working directory used to
  execute the command.
- A `file` check may declare `exists: true`.
- A `file` `exists` check passes when the referenced file exists.
- A `file` check may declare `sha256`.
- A `file` `sha256` check passes when the referenced file exists and its
  SHA-256 hash matches the expected lowercase hexadecimal digest.
- A `file` check must declare exactly one operator: `exists` or `sha256`.
- A `port` assertion check declares a single integer `port`.
- A `port` assertion check declares `free: true`.
- A `port` assertion check targets TCP listener availability on the local
  machine.
- A `port` `free` assertion passes when no process is listening on that TCP
  port on any local interface after the command body completes.
- `assert.checks` execute before deferred cleanup for that command entry.
- A `port` assertion therefore does not verify port state after explicit
  `cleanup` or automatic process cleanup completes.
- A `Command` entry may include a `capture` section.
- `capture` is an array of named extraction rules.
- Each capture rule declares a variable `name`.
- Captured variable names must be unique across the whole runbook.
- In this increment, the only supported capture `source` is `stdout`.
- Each capture rule declares a `stage` of either `raw` or `rewritten`.
- `raw` captures from the original command stdout before output rewrites are
  applied.
- `rewritten` captures from stdout after `output.rewrite` has been applied.
- Each capture rule declares a regex `pattern`.
- A capture rule succeeds when its pattern matches exactly one value to store.
- Captured variables may be interpolated into later command lines using
  `@{name}` syntax.
- `@@{name}` escapes that syntax and leaves a literal `@{name}` in the command
  or Markdown content.
- A command that references `@{name}` must use a variable captured earlier in
  the runbook.
- If a `Command` entry contains an `output` property, render captured command
  output after the command block.
- If `output.caption` is present, render that caption before the captured
  command output.
- If `output.caption` is a scalar string, ignore a terminal line break that
  exists only to terminate the scalar so YAML literal scalars do not introduce
  an extra blank line before the captured output fence.
- `output` may declare `stream`.
- `output` may declare `content_type`.
- `output` may declare `trim_empty_lines`.
- `output` may declare `trim_trailing_whitespace`.
- `output` may declare `rewrite`.
- If `output.stream` is omitted, captured stdout followed by captured stderr is
  rendered.
- `output.stream` accepts `stdout`, `stderr`, and `combined`.
- `output.stream: stdout` renders only captured command stdout.
- `output.stream: stderr` renders only captured command stderr.
- `output.stream: combined` renders captured stdout followed by captured
  stderr.
- `output.stream` selects the rendered output stream before
  `output.trim_trailing_whitespace`, `output.rewrite`, and
  `output.trim_empty_lines` are applied.
- `output.stream` affects rendering only and does not change command
  assertions, capture sources, or process execution behavior.
- In this increment, `capture.source` remains limited to `stdout` even when
  `output.stream` renders `stderr` or `combined`.
- In this increment, assertion check sources remain unchanged even when
  `output.stream` renders `stderr` or `combined`.
- If `output.content_type` is omitted, captured command output is rendered as
  plain output in an unlabeled fenced block.
- If `output.content_type` is `text`, captured command output is rendered in an
  unlabeled fenced block.
- If `output.content_type` is present, the generated Markdown fenced block uses
  the declared content type.
- In this increment, supported `output.content_type` values are `text`, `json`,
  `xml`, `html`, and `java`.
- If `output.trim_empty_lines` is omitted, the captured output trims leading
  and trailing empty lines.
- `output.trim_empty_lines` accepts `leading_trailing`, `leading`, `trailing`,
  and `none`.
- `output.trim_empty_lines` trims only leading and/or trailing empty lines; it
  does not remove empty lines inside the kept output.
- For `trim_empty_lines`, an empty line is a line that contains only
  whitespace.
- `trim_empty_lines` is applied after `output.rewrite`.
- If `output.trim_trailing_whitespace` is omitted, trailing whitespace is
  removed from the end of each rendered output line.
- `output.trim_trailing_whitespace` affects only trailing whitespace, not
  leading whitespace.
- If `output.trim_trailing_whitespace` is set to `false`, captured output is
  rendered without that trailing-whitespace normalization.
- If `output.rewrite` is present, the captured command output is transformed by
  the declared rewrite rules before rendering.
- `output.rewrite` is an ordered array of rewrite rules.
- Rewrite rules are applied in the declared order.
- In this increment, supported rewrite rule types are `replace`,
  `datetime_shift`, and `keep_between`.
- A `replace` rewrite rule performs a pattern-based replacement on the captured
  command output.
- A rewrite rule may declare `capture_as` to expose both the matched original
  value and the rewritten value as generated captured variables.
- When `capture_as` is present, the rewrite rule creates
  `@{<capture_as>_original}` and `@{<capture_as>_rewritten}`.
- `capture_as` generated variable names participate in the same runbook-wide
  uniqueness rules as explicit `capture` names.
- If a rewrite rule with `capture_as` matches anything other than exactly one
  value, the run fails.
- A rewrite `capture_as` failure reports the failing `Command` entry together
  with the captured stdout and stderr so users can see why the match failed.
- A `replace` rewrite rule may interpolate captured variables inside its
  `pattern` text using `@{name}` syntax.
- `@@{name}` inside a `replace` rule `pattern` preserves the literal
  `@{name}` text.
- A `replace` rule `pattern` that references `@{name}` must use a variable
  captured earlier in the runbook.
- A `replace` rewrite rule may interpolate captured variables inside its
  `replacement` text using `@{name}` syntax.
- `@@{name}` inside a `replace` rule `replacement` preserves the literal
  `@{name}` text.
- A `replace` rule `replacement` that references `@{name}` must use a variable
  captured earlier in the runbook.
- A `keep_between` rewrite rule keeps only the lines between a matched `start`
  line and an optional matched `end` line.
- In this increment, `start` and `end` are matched as literal strings, not
  regular expressions.
- `keep_between` uses line-based offsets.
- `start_offset` defaults to `1`.
- `end_offset` defaults to `-1`.
- `show_trim_markers` defaults to `true`.
- `start_offset: 1` means start on the line after the matched `start` line.
- `end_offset: -1` means stop on the line before the matched `end` line.
- If `end` is omitted, `keep_between` keeps from the matched `start` boundary
  to the end of the output after applying `start_offset`.
- If `end` is omitted, `end_offset` is ignored.
- When `show_trim_markers` is `true`, `keep_between` adds a line containing
  `...` only on sides where output was actually trimmed.
- If lines were trimmed before the kept slice, a leading `...` marker is added.
- If lines were trimmed after the kept slice, a trailing `...` marker is added.
- `show_trim_markers: false` suppresses those trim-marker lines.
- If a required `keep_between` boundary is not found, the rule leaves the output
  unchanged.
- A `datetime_shift` rewrite rule shifts matched timestamps so the first match
  becomes the configured base timestamp and later matches preserve their
  relative distance from that first match.
- A `datetime_shift` rule may establish a shared shift anchor with `id`.
- A `datetime_shift` rule may reuse a previously established shift anchor with
  `use`.
- `id` and `use` are mutually exclusive.
- A `datetime_shift` rule may use a built-in `format` or a custom `pattern`.
- `format` and `pattern` are mutually exclusive.
- In this increment, built-in `format` values are `rfc3339` and `rfc1123`.
- No built-in time-only `format` names are introduced in this increment.
- Built-in `rfc3339` matching supports timestamps with a numeric offset or a
  `Z` suffix, and with optional fractional seconds.
- When built-in `rfc3339` fractional seconds are present, they may use between
  1 and 9 digits.
- If a built-in `rfc3339` match uses `Z`, the rewritten output preserves the
  `Z` suffix.
- If `format` is used, the original matched format is preserved in the
  rewritten output.
- If `pattern` is used for semantic datetime shifting, `custom_format` is
  required.
- `format` and `custom_format` are mutually exclusive.
- If `pattern` and `custom_format` are used together, the matched output is
  parsed and rewritten using that custom format while preserving the same
  textual style.
- If `pattern` and `custom_format` describe a time-only value without a date,
  `datetime_shift` borrows the date and offset from the configured or inherited
  base timestamp, applies the shared shift, and then renders only the original
  time-only textual format.
- Time-only shifting must use `pattern` together with `custom_format`; it does
  not use any special built-in time-only format name.
- If `base` is omitted, `datetime_shift` uses the default base timestamp
  `2077-04-27T12:34:56.789+01:00`.
- If `id` is used, the rule establishes the shift delta for that named anchor.
- `datetime_shift.id` values must be unique across the whole runbook.
- If `use` is used, the rule reuses the shift delta from the named anchor
  instead of establishing a new one.
- A rule that uses `use` must not declare `base`.
- A rule that uses `use` follows the timeline established by the referenced
  anchor, even when the matched datetime format differs.
- A rule that uses `use` may reference an anchor established earlier anywhere
  in the runbook.
- Forward references are invalid: a rule may not use an anchor before that
  anchor is established earlier in the runbook.
- Rewrite rules affect rendered output only and do not change command
  execution or assertions.
- If a `Command` entry does not contain an `output` property, command output is
  not written to the generated document.

## Acceptance Criteria

### Command Invocation And Files

- [ ] `sw` with no subcommand behaves the same as `sw run`.
- [ ] Given no input file argument and a valid `./sw-runbook.json`, with no
      other default runbook file present, `sw` renders the file and writes
      `./README.md`.
- [ ] Given `sw run --input-file <file>` with a valid runbook, the command
      renders entries in order and exits with `0`.
- [ ] Given `sw run --input-file=-` with a valid JSON runbook on stdin, the
      command renders entries in order and exits with `0`.
- [ ] Given `sw --input-file=-` with a valid JSON runbook on stdin, the
      command behaves the same as `sw run --input-file=-`.
- [ ] Given `sw run --input-file=- --input-format=yaml` with a valid YAML
      runbook on stdin, the command renders entries in order and exits with
      `0`.
- [ ] Given `sw run --input-file=-` with YAML on stdin and no
      `--input-format=yaml`, the command exits with `1` and reports a clear
      parsing error.
- [ ] Given `--input-format=json` or `--input-format=yaml` without
      `--input-file=-`, the command still uses the existing default file lookup
      behavior, including ambiguity failures when multiple default runbooks
      exist.
- [ ] Given no input file argument and more than one of `./sw-runbook.json`,
      `./sw-runbook.yaml`, or `./sw-runbook.yml` present, `sw run` returns exit
      code `1` with a clear ambiguity error that requires `--input-file`.
- [ ] Given `--output-file <path>`, the command writes the output to the
      provided path.
- [ ] Given `sw run --verbose`, progress lines are written to stderr without
      changing the existing stdout contract.
- [ ] Given `sw --verbose` with no subcommand, the command behaves the same as
      `sw run --verbose`.
- [ ] Given `sw run --debug`, diagnostic output is written to stderr without
      changing the existing stdout contract.
- [ ] Given `sw --debug` with no subcommand, the command behaves the same as
      `sw run --debug`.
- [ ] Given a runbook with one `Command` entry with `debug: true` and another
      without it, and no global `--debug`, diagnostics are written only for
      the flagged command entry.
- [ ] Given a `Command` entry with `debug: false`, that entry does not emit
      debug diagnostics unless global `--debug` is enabled.
- [ ] Given global `--debug`, all command entries emit debug diagnostics
      regardless of command-level `debug`.
- [ ] Given `sw run --verbose`, entry numbers are padded so summaries align to
      the same starting column.
- [ ] Given `sw run --verbose`, elapsed time is shown as seconds with one
      decimal place under one minute and as minutes plus seconds from one
      minute onward.
- [ ] Given an invalid runbook, the command exits with `2` and does not write a
      partial output file, and the human-readable validation output includes a
      nearby offending block for entry-scoped validation errors.
- [ ] Given a missing input file, the command exits with `1` and reports a
      clear error.
- [ ] Generated Markdown begins with the comment
      `<!-- Generated by Sociable Weaver (https://github.com/albertattard/sw). Manual edits will be overwritten when regenerated. -->`.

### Heading Entries

- [ ] Given a runbook with `Heading` entries, the generated Markdown contains
      the expected heading markers for the configured levels.

### Markdown Entries

- [ ] Given a runbook with `Markdown` entries, the generated Markdown preserves
      the entry content in order.
- [ ] Given a runbook whose `Markdown.contents` is a single string, the
      generated Markdown preserves that content as if the lines had been
      declared explicitly in an array.
- [ ] Given a scalar `Markdown.contents` value that ends with a line break only
      because of YAML literal-scalar termination, the generated Markdown does
      not introduce an extra blank line before the following entry.
- [x] Given a `Markdown` entry with `indent`, each non-empty rendered Markdown
      line is prefixed with that many spaces.
- [x] Given a `Markdown` entry with `indent` and blank lines, blank lines
      remain empty in the rendered output.
- [ ] Markdown entries may interpolate `@{name}` when that variable is
      captured anywhere in the runbook.
- [ ] `@@{name}` in Markdown content preserves the literal `@{name}`.
- [ ] Markdown entries may interpolate values captured later in the runbook.
- [ ] A Markdown entry that references a variable that is never captured
      anywhere in the runbook causes the run to fail.

### DisplayFile Entries

- [ ] Given a runbook with `DisplayFile` entries, the generated Markdown
      includes the referenced file contents in a fenced block.
- [ ] Given a `DisplayFile` entry that references a `.java` file, the
      generated Markdown uses a `java` fenced block.
- [ ] Given a `DisplayFile` entry that references a `.sql` file, the
      generated Markdown uses a `sql` fenced block.
- [ ] Given a `DisplayFile` entry that references a `.xml` file, the
      generated Markdown uses an `xml` fenced block.
- [ ] Given a `DisplayFile` entry whose extension is not recognized, the
      generated Markdown uses a `text` fenced block.
- [ ] Given a `DisplayFile` entry with `start_line`, rendering begins at that
      1-based line.
- [ ] Given a `DisplayFile` entry with `start_line` and `line_count`, only the
      requested slice is rendered.
- [ ] Given a `DisplayFile` entry with `start_line` and no `line_count`,
      rendering continues from that line to the end of the file.
- [ ] Given a `DisplayFile` entry with `indent`, the opening fence, copied
      content lines, and closing fence are all prefixed with that many spaces.
- [ ] Given a `DisplayFile` entry with a positive `offset`, each non-empty
      copied file content line is prefixed with that many spaces inside the
      fenced block.
- [ ] Given a `DisplayFile` entry with a negative `offset`, up to that many
      leading spaces are removed from each non-empty copied file content line
      inside the fenced block.
- [ ] Given a `DisplayFile` entry with blank lines and `offset`, blank copied
      file content lines remain blank in the rendered output.

### Prerequisite Entries

- [ ] Given a runbook with `Prerequisite` entries, the generated Markdown
      includes the declared prerequisite `contents` in order.
- [ ] Given a prerequisite check whose `contents` is a single string, the
      generated Markdown preserves that content as if the lines had been
      declared explicitly in an array.
- [ ] Given a scalar prerequisite `contents` value that ends with a line break
      only because of YAML literal-scalar termination, the generated Markdown
      does not introduce an extra blank line before the following entry.
- [x] Given a prerequisite check whose `commands` is a single string, those
      command lines execute as if they had been declared explicitly in an
      array.
- [x] Given scalar prerequisite `commands` that end with a line break only
      because of YAML literal-scalar termination, execution does not introduce
      an extra blank command line.
- [ ] Given a single `Prerequisite` entry with multiple checks, all of those
      checks are evaluated from that entry's `checks` array.
- [ ] Given a prerequisite check with multiple command lines, those lines
      execute together in the same shell context.
- [ ] Given prerequisite checks, they execute before normal runbook commands.
- [ ] Given a failing prerequisite check, the run exits with `2` before
      executing the main workflow.
- [ ] Given a failing prerequisite check with `help`, the failure output
      includes that remediation message.
- [ ] Given passing prerequisite checks, the run continues to the main
      workflow.

### Patch Entries

- [ ] Given a runbook with `Patch` entries, the generated Markdown includes
      fenced `diff` blocks for those patches.
- [ ] Given a `Patch` entry without an explicit restore setting, the patched
      file is restored automatically after a successful run.
- [ ] Given a `Patch` entry without an explicit restore setting, the patched
      file is restored automatically after a failed or timed-out run.
- [ ] Given multiple `Patch` entries that modify the same file, later patch
      entries may build on the earlier patched state during the run.
- [ ] Given multiple `Patch` entries that modify the same file, restoration
      unwinds in reverse patch-application order and leaves the file in its
      original pre-run state.
- [ ] Given a failure while restoring one patched file, later registered patch
      restores still run.
- [ ] Given a `Patch` entry that cannot be applied cleanly, the run fails
      without waiting for interactive input from the patch tool.
- [ ] Given a `Patch` entry that cannot be applied cleanly, the target file
      remains unchanged and no `.orig` or `.rej` sidecar files are left

### Command Entries

- [x] Given a runbook whose `Command.commands` is a single string, the
      generated Markdown preserves that command text as if the lines had been
      declared explicitly in an array.
- [x] Given scalar `Command.commands` that end with a line break only because
      of YAML literal-scalar termination, rendering and execution do not
      introduce an extra blank command line.
- [x] Given scalar `Command.commands`, all resulting command lines execute
      together in the same shell context.
      behind.

### Command Execution

- [ ] Given a runbook with `Command` entries, the commands are executed in the
      same order as they appear in the runbook.
- [ ] Given a `Command` entry with multiple command lines, those lines execute
      together in the same shell context so values set on one line can be used
      on a later line.
- [ ] Given a runbook with `Command` entries, the generated Markdown includes
      fenced command blocks.
- [ ] Given a `Command` entry with `indent`, each rendered line in that
      command section is prefixed with the configured number of spaces.
- [ ] Given `sw run --verbose`, a `Markdown` entry summary uses the first
      non-empty content line instead of the full block contents.
- [ ] Given `sw run --verbose`, a `Command` entry summary uses the first
      non-empty command line instead of the full command block.
- [ ] Given `sw run --verbose`, a running `Command` entry shows elapsed time
      together with the expected timeout window.
- [ ] Given `sw run --verbose` with stderr attached to a TTY, the current
      entry line shows a live elapsed timer that updates in place while the
      entry is running.
- [ ] Given `sw run --verbose`, the running timer does not use a trailing
      `...`.
- [ ] Given `sw run --verbose` with stderr not attached to a TTY, progress
      output falls back to non-live line-based output.
- [ ] Given `sw run --debug` for a `Command` entry with rewrites and captures,
      stderr includes enough interpolated rewrite and capture information to
      help diagnose matching failures.
- [ ] Given `output.stream: stdout`, rendered command output includes only
      captured stdout.
- [ ] Given `output.stream: stderr`, rendered command output includes only
      captured stderr.
- [ ] Given `output.stream: combined`, rendered command output includes
      captured stdout followed by captured stderr.
- [ ] Given no `output.stream`, rendered command output defaults to combined
      stdout followed by stderr.
- [ ] Given an invalid `output.stream` value, validation rejects the runbook
      with a clear error.
- [ ] Given `output.stream: stderr` together with `output.rewrite`, rewrites
      apply to the selected stderr stream before rendering.
- [ ] Given `output.stream: combined` together with output trimming, trimming
      applies to the selected combined stream before rendering.
- [ ] Given `output.stream: stderr` or `output.stream: combined`,
      `capture.source` and assertion-check sources keep their existing
      contracts and are not implicitly widened.

### Command Cleanup

- [ ] Given a command without `cleanup`, remaining processes started by that
      command are terminated automatically after the entry finishes.
- [ ] Given a command without `cleanup`, remaining processes started by that
      command are terminated automatically after failure or timeout.
- [ ] Given automatic process cleanup for a command whose process group has
      already exited, the cleanup step is treated as a successful no-op.
- [ ] Given automatic process cleanup for a command whose process group has
      already exited, no user-visible "No such process" warning is printed.
- [ ] Given commands that declare `cleanup`, cleanup commands execute in reverse
      order after the run completes.
- [ ] Given a `cleanup` block with multiple command lines, those lines execute
      in the declared order and in the same shell context.
- [ ] Given a command whose `cleanup` is a single string, cleanup executes with
      the same behavior as the existing line-array cleanup model.
- [ ] Given a command whose `cleanup` is a YAML literal scalar with a terminal
      line break, the implicit terminator blank line is ignored.
- [ ] Given a `cleanup` block that expresses a shell control structure across
      multiple lines, that structure executes correctly during cleanup.
- [ ] Given a command with `cleanup`, the explicit cleanup block is used
      instead of automatic process cleanup for that entry.
- [ ] Given a command failure, previously registered cleanup commands still
      execute in reverse order before the run exits.
- [ ] Given a command timeout, previously registered cleanup commands still
      execute in reverse order before the run exits.
- [ ] Given a command without `cleanup`, no cleanup command is registered for
      that entry.
- [ ] Given a failed cleanup line, the remaining lines in that cleanup block
      still execute.
- [ ] Given a failed cleanup block, remaining registered cleanup blocks still
      execute in reverse order.
- [ ] Given one or more cleanup failures, the run is reported as failed after
      cleanup completes.

### Command Timeouts

- [ ] Given a command without a declared timeout, the default timeout of
      `2 minutes` is used.
- [ ] Given a command with a declared timeout such as `30 seconds`,
      `1 minute`, or `5 minutes`, that timeout is used for the command.
- [ ] Given a command that finishes within its timeout, the run continues.
- [ ] Given a command that exceeds its timeout, the command process is
      terminated, the run exits with `2`, and any captured output produced
      before termination is preserved to aid debugging.

### Command Assertions

- [ ] Given a command without an `assert` section that exits successfully, the
      run continues.
- [ ] Given a command without an `assert` section that exits with an error, the
      command exits with `2` and does not write a partial output file.
- [ ] Given a command with `assert.exit_code`, the command is considered
      successful only when the actual exit code matches the asserted value.
- [ ] Given a command with `assert.exit_code` that does not match the actual
      exit code, the command exits with `2` and does not write a partial output
      file, and the error output includes the failing `Command` entry together
      with the captured stdout and stderr.
- [ ] Given a command with `assert.checks` using `source: stdout` and
      `contains`, the command is considered successful only when stdout
      contains the expected text.
- [ ] Given a command with a `stdout` `contains` check that fails, the run
      exits with `2`, does not write a partial output file, and reports the
      failing `Command` entry together with the captured stdout and stderr.
- [ ] Given a command with `preconditions.checks` using `source: port`,
      `port: 8080`, and `free: true`, the command body executes only when TCP
      port `8080` is not listening locally.
- [ ] Given a failing `port` precondition check, the run exits with `2`,
      does not execute the command body, does not write a partial output file,
      and reports the failing `Command` entry.
- [ ] Given a command with `assert.checks` using `source: file`, `path`, and
      `exists: true`, the command is considered successful only when the file
      exists after command execution.
- [ ] Given a command with `assert.checks` using `source: file`, `path`, and
      `sha256`, the command is considered successful only when the file exists
      and its SHA-256 matches the expected digest.
- [ ] Given a command with `assert.checks` using `source: port`, `port: 8080`,
      and `free: true`, the command is considered successful only when TCP
      port `8080` is not listening locally after the command body completes.
- [ ] Given a failing `file` assertion, the run exits with `2`, does not write
      a partial output file, and reports the failing `Command` entry together
      with the captured stdout and stderr.
- [ ] Given a failing `port` assertion, the run exits with `2`, does not write
      a partial output file, and reports the failing `Command` entry together
      with the captured stdout and stderr.
- [ ] Given a command with `cleanup` that releases a port, a same-entry
      `source: port` assertion still evaluates before deferred cleanup and does
      not treat post-cleanup port state as part of that assertion.
- [ ] Given multiple assertion checks, all checks must pass for the command to
      be considered successful.

### Command Capture

- [ ] Given a `Command` entry with `capture`, matching values are stored under
      the declared variable names.
- [ ] Given `stage: raw`, capture uses stdout before rewrite rules are applied.
- [ ] Given `stage: rewritten`, capture uses stdout after rewrite rules are
      applied.
- [ ] Given a later command that uses `@{name}`, the captured value is
      interpolated into the command before execution.
- [ ] Given a command that references `@{name}` before that variable is
      captured earlier in the runbook, validation rejects the runbook.
- [ ] Given duplicate capture variable names anywhere in the runbook,
      validation rejects the runbook.
- [ ] Given a capture rule whose pattern does not resolve to exactly one value,
      the run fails.
- [ ] Given `@@{name}` in command or Markdown content, the literal `@{name}`
      is preserved without interpolation.

### Command Output Rendering

- [ ] Given a `Command` entry with an `output` property, the generated Markdown
      includes the captured command output.
- [ ] Given a `Command` entry with `output.caption`, the generated Markdown
      includes the caption before the captured command output.
- [ ] Given a scalar `output.caption` value that ends with a line break only
      because of YAML literal-scalar termination, the generated Markdown does
      not introduce an extra blank line before the captured output fence.
- [ ] Given a `Command` entry with `output.content_type: json`, the generated
      Markdown uses a `json` fenced block for captured output.
- [ ] Given a `Command` entry with `output.content_type: xml`, the generated
      Markdown uses an `xml` fenced block for captured output.
- [ ] Given a `Command` entry with `output.content_type: html`, the generated
      Markdown uses an `html` fenced block for captured output.
- [ ] Given a `Command` entry with `output.content_type: java`, the generated
      Markdown uses a `java` fenced block for captured output.
- [ ] Given a `Command` entry with `output` but no `content_type`, the
      generated Markdown uses an unlabeled fenced block for captured output.
- [ ] Given a `Command` entry with `output.content_type: text`, the generated
      Markdown uses an unlabeled fenced block for captured output.
- [ ] Given a `Command` entry with `output.trim_empty_lines: leading_trailing`,
      leading and trailing empty lines are removed from rendered output.
- [ ] Given a `Command` entry with `output` and no `trim_empty_lines`,
      leading and trailing empty lines are removed from rendered output.
- [ ] Given a `Command` entry with `output.trim_empty_lines: leading`, only
      leading empty lines are removed from rendered output.
- [ ] Given a `Command` entry with `output.trim_empty_lines: trailing`, only
      trailing empty lines are removed from rendered output.
- [ ] Given a `Command` entry with `output.trim_empty_lines: none`, leading and
      trailing empty lines are preserved in rendered output.
- [ ] Given a `Command` entry with `output` and no
      `trim_trailing_whitespace`, trailing whitespace is removed from the end
      of each rendered output line.
- [ ] Given a `Command` entry with `output.trim_trailing_whitespace: false`,
      trailing whitespace is preserved in rendered output.
- [ ] Given a `Command` entry with `output.rewrite`, rewrite rules are applied
      in the declared order before output is rendered.
- [ ] Given a `replace` rewrite rule, matching text is replaced in rendered
      output.
- [ ] Given a rewrite rule with `capture_as`, the matched pre-rewrite value is
      stored as `@{<capture_as>_original}`.
- [ ] Given a rewrite rule with `capture_as`, the rewritten value is stored as
      `@{<capture_as>_rewritten}`.
- [ ] Given a rewrite rule with `capture_as`, later commands and Markdown may
      reference those generated variables using the normal capture syntax.
- [ ] Given a rewrite rule with `capture_as` whose generated names collide with
      an existing captured variable, validation rejects the runbook.
- [ ] Given a rewrite rule with `capture_as` that matches zero values or more
      than one value, the run exits with `2`, does not write a partial output
      file, and reports the failing `Command` entry together with the captured
      stdout and stderr.
- [ ] Given a `replace` rewrite rule `pattern` that uses `@{name}` after that
      variable is captured earlier in the runbook, the pattern text includes
      the captured value before matching.
- [ ] Given a `replace` rewrite rule `pattern` that uses `@{name}` before that
      variable is captured, validation rejects the runbook.
- [ ] Given `@@{name}` in a `replace` rewrite rule `pattern`, the literal
      `@{name}` is preserved without interpolation.
- [ ] Given a `replace` rewrite rule `replacement` that uses `@{name}` after
      that variable is captured earlier in the runbook, the replacement text
      includes the captured value.
- [ ] Given a `replace` rewrite rule `replacement` that uses `@{name}` before
      that variable is captured, validation rejects the runbook.
- [ ] Given `@@{name}` in a `replace` rewrite rule `replacement`, the literal
      `@{name}` is preserved without interpolation.
- [ ] Given a `keep_between` rewrite rule, only the lines between the matched
      `start` and `end` boundaries are kept.
- [ ] Given a `keep_between` rewrite rule with `start` and no `end`, the kept
      slice runs from the adjusted `start` boundary to the end of the output.
- [ ] Given a `keep_between` rewrite rule without explicit offsets,
      `start_offset: 1` and `end_offset: -1` are used.
- [ ] Given a `keep_between` rewrite rule with explicit offsets, the resulting
      output slice reflects those line-based offsets.
- [ ] Given a `keep_between` rewrite rule without `end`, `end_offset` is
      ignored.
- [ ] Given a `keep_between` rewrite rule without explicit
      `show_trim_markers`, trim-marker lines are added only on the sides where
      output was trimmed.
- [ ] Given a `keep_between` rewrite rule with `show_trim_markers: false`,
      trim-marker lines are omitted.
- [ ] Given a `keep_between` rewrite rule whose `start` or `end` boundary is
      not found, the rule leaves the output unchanged.
- [ ] Given a `datetime_shift` rewrite rule, the first matched timestamp is
      rewritten to the configured base timestamp.
- [ ] Given multiple timestamps matched by the same `datetime_shift` rule,
      later timestamps preserve their relative distance from the first matched
      timestamp.
- [ ] Given a `datetime_shift` rewrite rule without `base`, the default base
      timestamp `2077-04-27T12:34:56.789+01:00` is used.
- [ ] Given a `datetime_shift` rewrite rule with `format: rfc3339`, matched
      timestamps are rewritten and kept in RFC 3339 form.
- [ ] Given a `datetime_shift` rewrite rule with `format: rfc3339` and matched
      timestamps that use 1 to 9 fractional-second digits, those timestamps
      are rewritten successfully without requiring a custom pattern.
- [ ] Given a `datetime_shift` rewrite rule with `format: rfc3339` and a
      matched timestamp that uses `Z`, the timestamp is rewritten
      successfully without requiring a custom pattern.
- [ ] Given a `datetime_shift` rewrite rule with `format: rfc3339` and a
      matched timestamp that uses fractional seconds together with `Z`, the
      timestamp is rewritten successfully without requiring a custom pattern.
- [ ] Given a `datetime_shift` rewrite rule with `format: rfc1123`, matched
      timestamps are rewritten and kept in RFC 1123 form.
- [ ] Given a `datetime_shift` rewrite rule with `id`, later datetime rules may
      reuse that established shift with `use`.
- [ ] Given multiple `datetime_shift` rewrite rules that share one anchor,
      matched datetimes in different supported formats preserve the same shared
      timeline shift.
- [ ] Given duplicate `datetime_shift.id` values anywhere in the runbook,
      validation rejects the runbook.
- [ ] Given a `datetime_shift` rewrite rule that uses `use`, the referenced
      anchor must have been established earlier in the runbook.
- [ ] Given a `datetime_shift` rewrite rule in a later command output block
      that uses an anchor established earlier in the runbook, the rule is
      valid and follows that shared timeline.
- [ ] Given a `datetime_shift` rewrite rule with `pattern` and
      `custom_format`, matched datetimes are rewritten while preserving that
      custom textual format.
- [ ] Given a `datetime_shift` rewrite rule with `pattern` and
      `custom_format` that describe a time-only value, the value is shifted
      using the date and offset from the configured or inherited base
      timestamp and is rendered back in the same time-only textual format.
- [ ] Given multiple time-only values matched by the same `datetime_shift`
      rule, later values preserve their relative distance from the first
      matched time-only value.
- [ ] Given a time-only `datetime_shift` rule that uses `use`, the rule
      follows the same shared anchor established by an earlier datetime rule.
- [ ] Given a `datetime_shift` rewrite rule that uses `use`, the rule does not
      declare its own `base`.
- [ ] Given a `Command` entry without an `output` property, the generated
      Markdown does not include the captured command output.

## Non-goals

- Supporting non-Markdown output formats in v1.
- Mutating the input runbook.
- Providing sandboxing or isolation beyond the local process environment.
- Allowing `Markdown` entries to interpolate values that are captured later in
  the runbook within the same implementation slice as command capture.

## Edge Cases

- Empty runbook.
- Unsupported entry type.
- `Prerequisite` entry with an empty `checks` array.
- A prerequisite check omits `name`.
- A prerequisite check omits `contents`.
- A prerequisite check omits `commands`.
- A prerequisite check fails its assertion before any normal command runs.
- A prerequisite check includes a `help` message that should be surfaced on
  failure.
- `DisplayFile` path points to a missing file.
- `DisplayFile` path points to an unreadable file.
- `DisplayFile` uses a recognized extension such as `.java`.
- `DisplayFile` uses an unrecognized extension and falls back to `text`.
- `DisplayFile` uses `start_line` less than `1`.
- `DisplayFile` uses `line_count` less than `1`.
- `DisplayFile` uses `line_count` without `start_line`.
- `DisplayFile` uses `start_line` beyond the end of the file.
- `DisplayFile` uses positive `indent` to nest a snippet more deeply.
- `DisplayFile` uses negative `indent` to remove surrounding code indentation
  from a sliced method body.
- `Patch` entry targets a missing file.
- Multiple `Patch` entries modify the same file in sequence.
- A `Patch` entry succeeds but the run later fails and restore still needs to
  return the file to its original state.
- A patch restore step fails but later registered patch restores still need to
  run.
- Output path points to an unwritable location.
- Existing output file already present.
- Command entry with multi-line commands.
- Command entry uses `indent` to remain inside a Markdown list item.
- Command output lines contain trailing spaces that should be trimmed by default.
- Command output needs exact trailing whitespace preserved.
- Command output includes paths, dates, or other environment-specific values
  that should be rewritten before publication.
- A Markdown entry interpolates a variable captured earlier in the runbook.
- A Markdown entry references a variable captured later in the runbook.
- A Markdown entry references a variable that is never captured anywhere in the
  runbook.
- Markdown content includes literal `@{name}` that must not be interpolated.
- Multiple rewrite rules are declared for the same output block.
- A `datetime_shift` rule matches timestamps across multiple lines.
- A `datetime_shift` rule omits `base` and relies on the default base timestamp.
- A `datetime_shift` rule uses a built-in format such as `rfc3339` or `rfc1123`.
- A `datetime_shift` rule uses `pattern` instead of `format`.
- A `datetime_shift` rule establishes a shared anchor with `id`.
- Multiple `datetime_shift` rules reuse the same shared anchor with `use`.
- The same `datetime_shift.id` is declared more than once in the runbook.
- A `datetime_shift` rule uses an anchor declared in an earlier command output
  block.
- A `datetime_shift` rule uses an anchor before it is established earlier in
  the runbook.
- A `datetime_shift` rule uses `pattern` together with `custom_format`.
- A `datetime_shift` rule uses `pattern` and `custom_format` to shift
  time-only values such as `12:56:13.902`.
- A time-only `datetime_shift` crosses midnight after shifting and renders the
  wrapped time value.
- A time-only `datetime_shift` reuses an earlier shared anchor with `use`.
- A `datetime_shift` rule declares both `id` and `use`.
- A `datetime_shift` rule declares both `format` and `custom_format`.
- A `datetime_shift` rule declares `use` together with `base`.
- A `keep_between` rule uses the default exclusive boundaries.
- A `keep_between` rule omits `end` and keeps the remainder of the output.
- A `keep_between` rule uses explicit positive or negative offsets.
- A `keep_between` rule uses default trim markers.
- A `keep_between` rule trims only from the start and shows only a leading
  trim marker.
- A `keep_between` rule trims only from the end and shows only a trailing trim
  marker.
- A `keep_between` rule suppresses trim markers.
- A `keep_between` rule does not find its `start` boundary.
- A `keep_between` rule does not find its `end` boundary.
- A capture rule uses `stage: raw`.
- A capture rule uses `stage: rewritten` together with `output.rewrite`.
- Multiple commands capture variables with the same name.
- A command references a captured variable before it is defined.
- Command or Markdown content contains literal `@{name}` that must not be
  interpolated.
- A rewrite rule `capture_as` generated name collides with an existing
  explicit or generated captured variable name.
- A `replace` rewrite rule `replacement` references a captured variable before
  it is defined.
- A `replace` rewrite rule `pattern` references a captured variable before it
  is defined.
- A `replace` rewrite rule `replacement` contains literal `@{name}` that must
  not be interpolated.
- A `replace` rewrite rule `pattern` contains literal `@{name}` that must not
  be interpolated.
- A capture rule matches no value.
- A capture rule matches more than one value when exactly one value is
  required.
- Variable assignment on one command line used by a later line in the same
  entry.
- Command relies on default automatic process cleanup.
- Command finishes before automatic process cleanup runs.
- Command provides manual `cleanup` instead of relying on automatic process
  cleanup.
- Multiple commands register cleanup and require reverse-order execution.
- A cleanup command is present for some commands and omitted for others.
- A cleanup block contains multiple command lines.
- A cleanup line fails but later lines in the same cleanup block still need to run.
- One cleanup block fails but later registered cleanup blocks still need to run.
- A command fails after earlier commands registered cleanup.
- A command times out after earlier commands registered cleanup.
- Command omits `timeout` and uses the default timeout.
- Command declares timeout with supported human-readable units.
- Command declares timeout with an unsupported unit.
- Command exceeds the configured timeout and must be terminated.
- Command exits with non-zero status and no assertion is present.
- Command exits with a non-zero status that is explicitly asserted.
- Command exits with `0` when a non-zero exit code was asserted.
- Command exits with the expected exit code but stdout does not satisfy the
  declared `contains` check.
- Multiple stdout checks declared for a single command.
- File assertion checks use `exists: true`.
- File assertion checks use `sha256`.
- File assertion checks reference a missing file.
- File assertion checks use a non-matching SHA-256 digest.
- Command writes to stderr but exits successfully.
- Command caption supplied as a string or array of strings.
- Output content type omitted and defaults to `text`.
- Output content type uses a supported rendering value such as `json`, `xml`,
  or `html`.
- Output content type uses an unsupported value.
- Command output is large.
- `working_dir` points outside the runbook directory via `..`.
- `working_dir` resolves to a path that does not exist.
- `working_dir` resolves to a file rather than a directory.
- `working_dir` includes shell-sensitive characters that require quoting in the
  rendered command block.

## Notes for Reimplementation

This feature establishes the first rendering contract for runbooks. Parsing and
validation should remain shared with `sw validate` so `run` and `validate`
enforce the same input contract. Command execution should be deterministic from
the CLI perspective: execution order, failure handling, and output capture
rules should be explicit and stable. Assertion structure should remain
extensible: `assert.exit_code` handles process-level expectations, while
`assert.checks` supports source-specific checks. In this increment,
`assert.checks` supports `source: stdout` with the `contains` operator,
`source: file` with `exists` and `sha256`, and `source: port` with
`free: true`, while leaving room for future operators such as regular-
expression or equality checks. `preconditions.checks` uses the same general
check model for command-start readiness, but remains separate from `assert`
because it runs before the command body and does not support process-result
fields such as `exit_code`. Port checks intentionally target one TCP port per
check so multiple failures can be reported against explicit entries rather than
being grouped into one ambiguous array assertion. Command execution must also
enforce a bounded runtime so runaway processes do not remain after a failed
run. First-class patch execution should snapshot original target files
before the first patch touches them so stacked patches can be restored safely
without requiring authors to hand-write reverse patches. Patch restoration
should unwind in reverse registration order and restore files to their
original pre-run bytes rather than to intermediate patched states. Cleanup
behavior should remain deterministic so resources started by earlier commands
are reliably released even when the run stops early. Cleanup should be best
effort rather than fail-fast: all registered cleanup blocks and all cleanup
lines should be attempted before the run reports cleanup failures. Output
rendering should remain extensible so captured command output can be tagged
with a content type such as `json`, `xml`, or `html` without changing the
surrounding output contract.
Command working-directory behavior should remain explicit at the process level
rather than being simulated by prepending `cd ... &&` to shell scripts. That
keeps command execution, cleanup, timeout handling, and file assertions aligned
to the same directory contract without making shell composition responsible for
path semantics.
