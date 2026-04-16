---
id: SPEC-010
title: Format Runbook Files
status: implemented
priority: medium
owner: @aattard
last_updated: 2026-04-16
---

## Problem

Users and agents need a built-in way to normalize runbook files into the
project’s canonical formatting style instead of relying on external formatters
or manual editing.

## User-facing Behavior

The CLI provides a formatting command:

```bash
sw format --input-file <sw-runbook.yaml>
sw format --input-file <sw-runbook.json>
```

If no input file is provided, the command uses the implicit default runbook
only when exactly one of these files exists:

- `./sw-runbook.json`
- `./sw-runbook.yaml`
- `./sw-runbook.yml`

For example:

```bash
sw format
```

The command rewrites the input file in place using deterministic formatting for
the file’s existing format. JSON input is formatted as canonical JSON. YAML or
YML input is formatted as canonical YAML. The command does not convert one
format into another.

Before writing, the command must confirm that the file parses successfully and
that the runbook passes the same structural validation contract as
`sw validate`. If parsing or validation fails, the command does not rewrite the
file.

## Inputs/Outputs

Input:
- Optional named input file parameter:
  `--input-file <runbook.{json|yaml|yml}>`.

Default input behavior:
- If `--input-file` is provided, use that path.
- If no file path is provided and exactly one of `./sw-runbook.json`,
  `./sw-runbook.yaml`, or `./sw-runbook.yml` exists, use that file.
- If no file path is provided and more than one of `./sw-runbook.json`,
  `./sw-runbook.yaml`, or `./sw-runbook.yml` exists, return exit code `1` with
  a clear error that the default input is ambiguous and `--input-file` must be
  specified.
- When reading from a file path or from the default file lookup, infer the
  format from the file extension or default file name.
- File-based runbook workflows default to YAML elsewhere in the CLI, but
  `format` preserves the existing file format instead of converting it.

Output:
- On success, the command rewrites the target runbook file in place.
- On success, the command prints a short human-readable confirmation.
- On failure, the command prints a clear error message and leaves the file
  unchanged.

### Exit Codes

- `0`: runbook file was formatted successfully.
- `1`: operational error (missing file, unreadable file, invalid JSON, invalid
  YAML, write failure, internal error).
- `2`: runbook file parses successfully but fails structural validation.

## Formatting Rules

- Formatting is in-place and preserves the existing file format.
- JSON output uses pretty-printed JSON with two-space indentation.
- YAML output uses deterministic YAML formatting suitable for repository
  editing, with stable indentation and line breaks.
- YAML sequences nested under mapping keys are indented by two spaces beneath
  the owning key, including the top-level `entries` list as
  `entries:\n  - ...`.
- YAML output inserts a single blank line between adjacent items in the
  top-level `entries` list.
- Property order is preserved exactly as it appears in the input file.
- String contents are preserved semantically, aside from format-specific
  escaping or block-scalar representation required by serialization.
- The formatted file ends with a trailing newline.
- The command does not add, remove, or infer runbook fields.
- The command does not make schema corrections; it only rewrites valid
  runbooks into the canonical layout for their current format.

## Acceptance Criteria

- [x] Given a valid runbook file,
      `sw format --input-file <file>` rewrites the file in place using the
      canonical formatting style for that file’s format and exits with `0`.
- [x] Given no `--input-file` option and a valid `./sw-runbook.yaml`, with no
      other default runbook file present, `sw format` formats that file in
      place and exits with `0`.
- [x] Given no `--input-file` option, no `./sw-runbook.yaml`, and a valid
      `./sw-runbook.json`, with no other default runbook file present,
      `sw format` formats that file in place and exits with `0`.
- [x] Given no `--input-file` option and more than one of
      `./sw-runbook.json`, `./sw-runbook.yaml`, or `./sw-runbook.yml` present,
      `sw format` exits with `1` and reports a clear ambiguity error that
      requires `--input-file`.
- [x] Given an input file with valid JSON or YAML but invalid runbook
      structure,
      `sw format --input-file <file>` exits with `2` and does not rewrite the
      file.
- [x] Given invalid JSON syntax,
      `sw format --input-file <file>` exits with `1` and does not rewrite the
      file.
- [x] Given invalid YAML syntax,
      `sw format --input-file <file>` exits with `1` and does not rewrite the
      file.
- [x] Given a file that is already formatted canonically, running `sw format`
      keeps the semantic content unchanged and leaves the file in the same
      canonical layout.
- [x] Formatting preserves object property order from the input file.
- [x] Formatting preserves mapping property order from the input YAML file.
- [x] Given a YAML runbook with two or more adjacent top-level `entries`,
      `sw format` rewrites the file so those adjacent entry items are
      separated by a single blank line.
- [x] Given a YAML runbook with sequences nested under mapping keys, `sw format`
      rewrites the file so those sequence item markers are indented by two
      spaces beneath their owning keys.

## Non-goals

- Reordering runbook properties.
- Repairing invalid JSON or invalid runbook structure.
- Converting YAML to JSON or JSON to YAML.
- Applying semantic runbook upgrades or migrations.

## Notes for Reimplementation

This command should share parsing and validation logic with `sw validate`
instead of introducing a second validation path. The formatter should remain
deterministic so repeated runs are stable for both humans and agents.
