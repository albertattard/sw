---
id: SPEC-003
title: Run Runbook to Markdown
status: in_progress
priority: high
owner: @aattard
last_updated: 2026-03-11
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
sw run
sw run --input-file <sw-runbook.json>
```

Initial version behavior:
- Read `sw-runbook.json` from the current directory when no file is provided.
- Render the runbook entries in order.
- Execute command entries in order.
- Produce Markdown output.
- Write the generated document to `./readme.md` by default.

This command generates documentation output and executes the commands declared
in the runbook.

## Inputs/Outputs

Input:
- Optional named input file parameter: `--input-file <runbook.json>`.
- Optional output format parameter: `--output-format markdown`.
- Optional output file parameter: `--output-file <path>`.

Default input behavior:
- If `--input-file` is provided, use that path. Otherwise use
  `./sw-runbook.json`.
- If `--output-format` is not provided, default to `markdown`.
- If `--output-file` is not provided, default to `./readme.md`.

Supported output formats in v1:
- `markdown`

Initial rendering rules for Markdown output:
- `Heading` entries render as Markdown headings based on their `level`.
- `Markdown` entries copy their `contents` into the output in order.
- `Command` entries render their `commands` as fenced shell code blocks.
- `Command` entries are executed in order.
- All lines within a single `Command` entry execute together in the same shell
  context.
- A `Command` entry must complete successfully for the command to continue.
- If a `Command` entry contains an `output` property, render captured command
  output after the command block.
- If `output.caption` is present, render that caption before the captured
  command output.
- If a `Command` entry does not contain an `output` property, command output is
  not written to the generated document.
- Entries are rendered in the same order as they appear in the runbook.

Output:
- Generated Markdown file written to the target path.
- Human-readable status on stdout.

Exit codes:
- `0`: runbook executed and rendered successfully.
- `1`: operational error (missing file, unreadable file, invalid JSON, write
  failure, internal error).
- `2`: invalid runbook input or command execution failure.

## Acceptance Criteria

- [ ] `sw` with no subcommand behaves the same as `sw run`.
- [ ] Given no input file argument and a valid `./sw-runbook.json`, `sw`
      renders the file and writes `./readme.md`.
- [ ] Given `sw run --input-file <file>` with a valid runbook, the command
      renders entries in order and exits with `0`.
- [ ] Given a runbook with `Command` entries, the commands are executed in the
      same order as they appear in the runbook.
- [ ] Given a `Command` entry with multiple command lines, those lines execute
      together in the same shell context so values set on one line can be used
      on a later line.
- [ ] Given a command that exits successfully, the run continues.
- [ ] Given a command that exits with an error, the command exits with `2` and
      does not write a partial output file.
- [ ] Given a runbook with `Heading` entries, the generated Markdown contains
      the expected heading markers for the configured levels.
- [ ] Given a runbook with `Markdown` entries, the generated Markdown preserves
      the entry content in order.
- [ ] Given a runbook with `Command` entries, the generated Markdown includes
      fenced command blocks.
- [ ] Given a `Command` entry with an `output` property, the generated Markdown
      includes the captured command output.
- [ ] Given a `Command` entry with `output.caption`, the generated Markdown
      includes the caption before the captured command output.
- [ ] Given a `Command` entry without an `output` property, the generated
      Markdown does not include the captured command output.
- [ ] Given an invalid runbook, the command exits with `2` and does not write a
      partial output file.
- [ ] Given a missing input file, the command exits with `1` and reports a
      clear error.
- [ ] Given `--output-file <path>`, the command writes the output to the
      provided path.

## Non-goals

- Supporting non-Markdown output formats in v1.
- Mutating the input runbook.
- Providing sandboxing or isolation beyond the local process environment.

## Edge Cases

- Empty runbook.
- Unsupported entry type.
- Output path points to an unwritable location.
- Existing output file already present.
- Command entry with multi-line commands.
- Variable assignment on one command line used by a later line in the same
  entry.
- Command exits with non-zero status.
- Command writes to stderr but exits successfully.
- Command caption supplied as a string or array of strings.
- Command output is large.

## Notes for Reimplementation

This feature establishes the first rendering contract for runbooks. Parsing and
validation should remain shared with `sw validate` so `run` and `validate`
enforce the same input contract. Command execution should be deterministic from
the CLI perspective: execution order, failure handling, and output capture
rules should be explicit and stable.
