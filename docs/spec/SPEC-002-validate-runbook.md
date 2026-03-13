---
id: SPEC-002
title: Validate Runbook Input
status: in_progress
priority: high
owner: @aattard
last_updated: 2026-03-04
---

## Problem

Before any execution step, users and agents need a fast and deterministic way
to confirm that a runbook file is valid input.

## User-facing Behavior

The CLI provides a validation command:

```bash
sw validate --input-file <sw-runbook.json> --output-format json
```

If no input file is provided, the command uses `./sw-runbook.json` by default:

```bash
sw validate --output-format json
```

The command validates runbook structure and required fields without changing
files.

## Inputs/Outputs

Input:
- Optional named input file parameter: `--input-file <runbook.json>`.
- Optional output format (`json` or `human`) via `--output-format`.

Default input behavior:
- If `--input-file` is provided, use that path. When no file path is provided,
  use `sw-runbook.json` in the current directory.
- If `--output-format` is not provided, default to `human`.

Supported output formats:
- `human` (default): readable console text for interactive use.
- `json`: machine-readable output for automation and agents.

Output (`--output-format human`):
- Human-readable validation summary.
- Validation errors listed with their paths and messages.
- For validation errors scoped to runbook entry content, the output also prints
  a nearby offending block for each error to aid debugging.

Reserved for future consideration (not part of this feature):
- `yaml`
- `ndjson`
- `sarif`

Output (`--output-format json`):
- Machine-readable JSON result including:
  - `schema_version`
  - `valid` (boolean)
  - `errors` (array)
  - `warnings` (array)

Exit codes:
- `0`: runbook is valid.
- `2`: runbook is invalid.
- `1`: operational error (file not found, unreadable file, internal error).

## Acceptance Criteria

- [ ] Given a valid runbook file,
      `sw validate --input-file <file> --output-format json`
      returns `valid: true`, an empty `errors` array, and exit code `0`.
- [ ] Given an invalid runbook file,
      `sw validate --input-file <file> --output-format json`
      returns `valid: false`, at least one structured error, and exit code `2`.
- [ ] Given a missing input file, command returns exit code `1` with a clear
      error message.
- [ ] Given no input file argument and a valid `./sw-runbook.json`,
      `sw validate --output-format json` validates that file and returns exit
      code `0`.
- [ ] Given no input file argument and no `./sw-runbook.json` present, command
      returns exit code `1` with a clear missing-file error.
- [ ] Given no `--output-format` option, command uses `human` output by
      default.
- [ ] Given a human-readable validation failure for `entries[N]`, the output
      includes a nearby offending block for that error.
- [ ] Validation performs no write operations (read-only behavior).

## Non-goals

- Executing runbook steps.
- Applying patches or mutating files.
- Generating README output.
- Detecting documentation drift or command correctness on target platforms.

## Edge Cases

- Empty JSON file.
- Invalid JSON syntax.
- Unknown top-level keys.
- Missing required fields.

## Test Cases

- Valid minimal runbook fixture.
- Invalid JSON fixture.
- Missing required field fixture.
- File path that does not exist.

## Notes for Reimplementation

This feature is the first contract slice for agent-first usage and should remain
stable as other commands are added.
