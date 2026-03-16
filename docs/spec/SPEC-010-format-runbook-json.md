---
id: SPEC-010
title: Format Runbook JSON
status: proposed
priority: medium
owner: @aattard
last_updated: 2026-03-15
---

## Problem

Users and agents need a built-in way to normalize runbook JSON into the
project’s canonical formatting style instead of relying on external formatters
or manual editing.

## User-facing Behavior

The CLI provides a formatting command:

```bash
sw format --input-file <sw-runbook.json>
```

If no input file is provided, the command uses `./sw-runbook.json` by default:

```bash
sw format
```

The command rewrites the input file in place using deterministic JSON
formatting that matches the established runbook style used in project
examples such as
`workshop-application-performance/examples/continuous-recording/sw-runbook.json`.

Before writing, the command must confirm that the file is valid JSON and that
the runbook passes the same structural validation contract as `sw validate`.
If parsing or validation fails, the command does not rewrite the file.

## Inputs/Outputs

Input:
- Optional named input file parameter: `--input-file <runbook.json>`.

Default input behavior:
- If `--input-file` is provided, use that path.
- If no file path is provided, use `./sw-runbook.json`.

Output:
- On success, the command rewrites the target JSON file in place.
- On success, the command prints a short human-readable confirmation.
- On failure, the command prints a clear error message and leaves the file
  unchanged.

### Exit Codes

- `0`: runbook file was formatted successfully.
- `1`: operational error (missing file, unreadable file, invalid JSON, write
  failure, internal error).
- `2`: runbook is valid JSON but fails structural validation.

## Formatting Rules

- Output uses pretty-printed JSON with two-space indentation.
- Object and array indentation follows the canonical style already used in
  repository runbook examples.
- Property order is preserved exactly as it appears in the input file.
- String contents are preserved exactly, aside from JSON escaping required by
  serialization.
- The formatted file ends with a trailing newline.
- The command does not add, remove, or infer runbook fields.
- The command does not make schema corrections; it only rewrites valid
  runbooks into the canonical JSON layout.

## Acceptance Criteria

- [ ] Given a valid runbook file,
      `sw format --input-file <file>` rewrites the file in place using the
      canonical JSON formatting style and exits with `0`.
- [ ] Given no `--input-file` option and a valid `./sw-runbook.json`,
      `sw format` formats that file in place and exits with `0`.
- [ ] Given an input file with valid JSON but invalid runbook structure,
      `sw format --input-file <file>` exits with `2` and does not rewrite the
      file.
- [ ] Given invalid JSON syntax,
      `sw format --input-file <file>` exits with `1` and does not rewrite the
      file.
- [ ] Given a file that is already formatted canonically, running `sw format`
      keeps the semantic content unchanged and leaves the file in the same
      canonical layout.
- [ ] Formatting preserves object property order from the input file.

## Non-goals

- Reordering runbook properties.
- Repairing invalid JSON or invalid runbook structure.
- Converting non-JSON formats into runbooks.
- Applying semantic runbook upgrades or migrations.

## Notes for Reimplementation

This command should share parsing and validation logic with `sw validate`
instead of introducing a second validation path. The formatter should remain
deterministic so repeated runs are stable for both humans and agents.
