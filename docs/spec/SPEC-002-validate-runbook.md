---
id: SPEC-002
title: Validate Runbook Input
status: In Progress
priority: High
owner: @aattard
last_updated: 2026-03-04
---

## Problem

Before any execution step, users and agents need a fast and deterministic way
to confirm that a runbook file is valid input.

## User-facing Behavior

The CLI provides a validation command:

```bash
sw validate --file <sw-runbook.json> --output json
```

If no file is provided, the command uses `./sw-runbook.json` by default:

```bash
sw validate --output json
```

The command validates runbook structure and required fields without changing
files.

## Inputs/Outputs

Input:
- Optional named file parameter: `--file <runbook.json>`.
- Optional output mode (`json` or `human`).

Default input behavior:
- If `--file` is provided, use that path. When no file path is provided, use
  `sw-runbook.json` in the current directory.
- If `--output` is not provided, default to `human`.

Supported output formats:
- `human` (default): readable console text for interactive use.
- `json`: machine-readable output for automation and agents.

Reserved for future consideration (not part of this feature):
- `yaml`
- `ndjson`
- `sarif`

Output (`--output json`):
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

- [ ] Given a valid runbook file, `sw validate --file <file> --output json`
      returns `valid: true`, an empty `errors` array, and exit code `0`.
- [ ] Given an invalid runbook file, `sw validate --file <file> --output json`
      returns `valid: false`, at least one structured error, and exit code `2`.
- [ ] Given a missing input file, command returns exit code `1` with a clear
      error message.
- [ ] Given no input file argument and a valid `./sw-runbook.json`,
      `sw validate --output json` validates that file and returns exit code `0`.
- [ ] Given no input file argument and no `./sw-runbook.json` present, command
      returns exit code `1` with a clear missing-file error.
- [ ] Given no `--output` option, command uses `human` output by default.
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
