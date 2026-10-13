---
id: SPEC-010
title: Format Runbook Files
status: implemented
priority: medium
owner: albertattard
last_updated: 2026-10-13
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

For a Java prerequisite, `sw format` normalizes an integer exact `version` to
its quoted-string form. For example, `version: 25` becomes `version: "25"` in
JSON and `version: '25'` in YAML. This gives authors a single canonical output
while still accepting the convenient numeric YAML input.

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
- YAML output renders multiline strings as literal block scalars so authored
  prose remains readable instead of becoming quoted text with escaped newline
  sequences.
- YAML output may normalize fields whose documented input contract accepts
  either a string or an array of strings into literal block scalars, matching
  the JSON-to-YAML conversion contract for those fields.
- Property order is preserved exactly as it appears in the input file, except
  for direct fields of all entries, which use the canonical orders below.
- String contents are preserved semantically, aside from format-specific
  escaping or block-scalar representation required by serialization.
- The formatted file ends with a trailing newline.
- The command does not add, remove, or infer runbook fields.
- The command does not make schema corrections; it only rewrites valid
  runbooks into the canonical layout for their current format.

### Entry Field Orders (TASK-201)

Direct fields of every supported entry type use the following order, omitting
absent fields. Nested objects retain their input field order. Any extra fields
accepted by validation are retained after the listed fields in their original
relative order.

| Entry type | Field order |
|---|---|
| `Heading` | `type`, `level`, `title` |
| `Markdown` | `type`, `contents`, `indent` |
| `ChangeDirectory` | `type`, `path`, `contents` |
| `DisplayFile` | `type`, `path`, `content_type`, `start_line`, `line_count`, `transform`, `show_trim_markers`, `offset`, `indent` |
| `DisplayUrl` | `type`, `url`, `timeout`, `start_line`, `line_count`, `content_type`, `show_trim_markers`, `offset`, `indent` |
| `Patch` | `type`, `path`, `patch`, `restore`, `indent`, `debug` |
| `Prerequisite` | `type`, `checks` |
| `Breakpoint` | `type`, `message` |

TASK-203 places `DisplayFile.content_type` immediately after `path` when present.

### Command Field Order

In both JSON and YAML, `sw format` orders the direct fields of each `Command`
entry as follows, omitting fields that are absent:

1. `type`
2. `debug`
3. `execute_when`
4. `working_directory`
5. `working_dir` (legacy spelling, when present)
6. `commands`
7. `assert`
8. `output`
9. `capture`
10. `cleanup`
11. `timeout`
12. `indent`

Formatting does not rename `working_dir`, add defaults, or reorder fields
inside nested objects.
Every array retains its original item order. Repeated formatting produces
identical file contents. TASK-200 introduced Command ordering; TASK-202
places `output` before `capture`. Field order does not control execution:
`stage: raw` captures original stdout and `stage: rewritten` captures stdout
after rewriting.

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
- [x] Formatting preserves object property order from the input file, except
      for direct entry fields, which use their canonical order.
- [x] Formatting preserves mapping property order from the input YAML file,
      except for direct entry fields, which use their canonical order.
- [x] Given a YAML runbook with two or more adjacent top-level `entries`,
      `sw format` rewrites the file so those adjacent entry items are
      separated by a single blank line.
- [x] Given a YAML runbook with sequences nested under mapping keys, `sw format`
      rewrites the file so those sequence item markers are indented by two
      spaces beneath their owning keys.
- [x] Given a YAML runbook with multiline scalar-capable prose such as
      `Prerequisite.checks[*].contents`, `sw format` writes that prose as a
      literal block scalar instead of a quoted string with escaped newlines.

### Command Ordering Increment

- [x] JSON and YAML `Command` fields are written in the canonical order above,
      regardless of their input order.
- [x] Absent fields remain absent and the legacy `working_dir` spelling is
      retained in its specified position.
- [x] Nested object fields and array items retain their input order.
- [x] Formatting an already ordered runbook produces identical file contents.
- [x] CLI help and help-focused tests describe canonical `Command` field order.

### All Entry Ordering Increment

- [x] JSON and YAML direct fields of every supported entry type follow the
      specified order, with absent fields omitted.
- [x] Accepted extra fields are retained after known fields in input order.
- [x] Nested objects, arrays, and semantic values are preserved, and repeated
      formatting is byte-identical.
- [x] Help and formatter guidance describe ordering for all entry types.

## Non-goals

- Reordering properties outside the direct fields of entries.
- Repairing invalid JSON or invalid runbook structure.
- Converting YAML to JSON or JSON to YAML.
- Applying semantic runbook upgrades or migrations.

## Notes for Reimplementation

This command should share parsing and validation logic with `sw validate`
instead of introducing a second validation path. The formatter should remain
deterministic so repeated runs are stable for both humans and agents.
