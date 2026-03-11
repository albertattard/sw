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
readable Markdown document without manually copying sections from the input
file.

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
- Produce Markdown output.
- Write the generated document to `./readme.md` by default.

This command generates documentation output. It does not execute shell commands
from the runbook.

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
- If a `Command` entry contains `output.caption`, render that caption after the
  command block.
- Entries are rendered in the same order as they appear in the runbook.

Output:
- Generated Markdown file written to the target path.
- Human-readable status on stdout.

Exit codes:
- `0`: runbook rendered successfully.
- `1`: operational error (missing file, unreadable file, invalid JSON, write
  failure, internal error).
- `2`: invalid runbook input.

## Acceptance Criteria

- [ ] `sw` with no subcommand behaves the same as `sw run`.
- [ ] Given no input file argument and a valid `./sw-runbook.json`, `sw`
      renders the file and writes `./readme.md`.
- [ ] Given `sw run --input-file <file>` with a valid runbook, the command
      renders entries in order and exits with `0`.
- [ ] Given a runbook with `Heading` entries, the generated Markdown contains
      the expected heading markers for the configured levels.
- [ ] Given a runbook with `Markdown` entries, the generated Markdown preserves
      the entry content in order.
- [ ] Given a runbook with `Command` entries, the generated Markdown includes
      fenced command blocks and any configured captions.
- [ ] Given an invalid runbook, the command exits with `2` and does not write a
      partial output file.
- [ ] Given a missing input file, the command exits with `1` and reports a
      clear error.
- [ ] Given `--output-file <path>`, the command writes the output to the
      provided path.

## Non-goals

- Executing runbook commands.
- Verifying command correctness on the target machine.
- Supporting non-Markdown output formats in v1.
- Mutating the input runbook.

## Edge Cases

- Empty runbook.
- Unsupported entry type.
- Output path points to an unwritable location.
- Existing output file already present.
- Command entry with multi-line commands.
- Command caption supplied as a string or array of strings.

## Notes for Reimplementation

This feature establishes the first rendering contract for runbooks. Parsing and
validation should remain shared with `sw validate` so `run` and `validate`
enforce the same input contract.
