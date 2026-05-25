---
id: SPEC-002
title: Validate Runbook Input
status: in_progress
priority: high
owner: @aattard
last_updated: 2026-04-15
---

## Problem

Before any execution step, users and agents need a fast and deterministic way
to confirm that a runbook file is valid input.

## User-facing Behavior

The CLI provides a validation command:

```bash
sw validate --input-file <sw-runbook.yaml> --output-format json
sw validate --input-file=- --output-format json
sw validate --input-file <sw-runbook.yaml> --working-directory /path/to/project
```

If no input file is provided, the command uses the implicit default runbook
only when exactly one of these files exists:

- `./sw-runbook.json`
- `./sw-runbook.yaml`
- `./sw-runbook.yml`

For example:

```bash
sw validate --output-format json
```

The command validates runbook structure and required fields without changing
files.

## Inputs/Outputs

Input:
- Optional named input file parameter: `--input-file <runbook.{json|yaml|yml}>`
  or `--input-file=-` to read the runbook from stdin.
- Optional input format (`json` or `yaml`) via `--input-format`.
- Optional output format (`json` or `human`) via `--output-format`.
- Optional execution root via `--working-directory <path>`.

Default input behavior:
- File-backed workflows elsewhere in the CLI default to YAML authoring, but
  stdin-backed validation keeps JSON as the default machine-oriented format.
- If `--input-file=-` is provided, read the runbook from stdin.
- If `--input-file=-` is provided and `--input-format` is omitted, parse stdin
  as JSON.
- If `--input-file=-` is provided and `--input-format=yaml`, parse stdin as
  YAML.
- If `--input-file` is provided with a path, use that path.
- When no file path is provided and exactly one of `sw-runbook.json`,
  `sw-runbook.yaml`, or `sw-runbook.yml` exists in the current directory, use
  that file.
- When no file path is provided and more than one of `sw-runbook.json`,
  `sw-runbook.yaml`, or `sw-runbook.yml` exists in the current directory,
  return exit code `1` with a clear error that the default input is ambiguous
  and `--input-file` must be specified.
- When reading from a file path or from the default file lookup, infer the
  input format from the file extension or default file name.
- `--input-format` does not bypass this default file ambiguity check when
  `--input-file=-` is not used.
- If `--working-directory` is not provided, validation resolves
  runbook-relative paths from the runbook file's directory. For stdin-backed
  runbooks, it resolves them from the shell current directory.
- If `--working-directory` is provided, resolve it relative to the shell
  current directory when it is relative.
- If `--working-directory` is provided, it must already exist and be a
  directory.
- `--input-file` remains a normal CLI path and resolves relative to the shell
  current directory when it is relative.
- Runbook-relative path validation uses the execution root selected by
  `--working-directory`.
- Supported input formats are JSON, YAML, and YML for files, and JSON or YAML
  for stdin.
- `Markdown.contents` may be either a single string or an array of strings.
- `Command.commands` may be either a single string or an array of strings.
- `Patch.patch` may be either a single string or an array of strings.
- `Prerequisite.checks[*].contents` may be either a single string or an array
  of strings.
- `Prerequisite.checks[*].commands` may be either a single string or an array
  of strings.
- When any of those fields are provided as a string, validation accepts that
  as shorthand for the existing line-array model.
- If `--output-format` is not provided, default to `human`.

Supported output formats:
- `human` (default): readable console text for interactive use.
- `json`: machine-readable output for automation and agents.

Output (`--output-format human`):
- Human-readable validation summary.
- Validation errors listed with their paths and messages.
- Validation warnings listed with their paths and messages.
- For validation errors scoped to runbook entry content, the output also prints
  a nearby offending block for each error to aid debugging.
- Warnings do not make the runbook invalid.

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

- [x] Given a valid runbook file,
      `sw validate --input-file <file> --output-format json`
      returns `valid: true`, an empty `errors` array, and exit code `0`.
- [x] Given a runbook that triggers a warning but no validation errors,
      `sw validate --input-file <file> --output-format json`
      returns `valid: true`, a non-empty `warnings` array, and exit code `0`.
- [x] Given an invalid runbook file,
      `sw validate --input-file <file> --output-format json`
      returns `valid: false`, at least one structured error, and exit code `2`.
- [x] Given a missing input file, command returns exit code `1` with a clear
      error message.
- [x] Given no input file argument and a valid `./sw-runbook.json`, with no
      other default runbook file present, `sw validate --output-format json`
      validates that file and returns exit code `0`.
- [x] Given no input file argument, no `./sw-runbook.json`, and a valid
      `./sw-runbook.yaml`, with no other default runbook file present,
      `sw validate --output-format json` validates that file and returns exit
      code `0`.
- [x] Given no input file argument, no `./sw-runbook.json` or
      `./sw-runbook.yaml`, and a valid `./sw-runbook.yml`,
      `sw validate --output-format json` validates that file and returns exit
      code `0`.
- [x] Given no input file argument and more than one of `./sw-runbook.json`,
      `./sw-runbook.yaml`, or `./sw-runbook.yml` present,
      `sw validate --output-format json` returns exit code `1` with a clear
      ambiguity error that requires `--input-file`.
- [x] Given no input file argument and none of `./sw-runbook.json`,
      `./sw-runbook.yaml`, or `./sw-runbook.yml` present, the command returns
      exit code `1` with a clear missing-file error.
- [x] Given `sw validate --input-file <file.yaml>` with a valid YAML runbook,
      the command validates that file and returns exit code `0`.
- [x] Given a runbook whose `Markdown.contents` is a single string,
      `sw validate --input-file <file> --output-format json` accepts that
      shorthand and returns `valid: true`.
- [x] Given a runbook whose `Command.commands` is a single string,
      `sw validate --input-file <file> --output-format json` accepts that
      shorthand and returns `valid: true`.
- [x] Given a runbook whose `Patch.patch` is a single string,
      `sw validate --input-file <file> --output-format json` accepts that
      shorthand and returns `valid: true`.
- [x] Given a runbook whose `Prerequisite.checks[*].contents` is a single
      string, `sw validate --input-file <file> --output-format json` accepts
      that shorthand and returns `valid: true`.
- [x] Given a runbook whose `Prerequisite.checks[*].commands` is a single
      string, `sw validate --input-file <file> --output-format json` accepts
      that shorthand and returns `valid: true`.
- [x] Given `sw validate --input-file=- --output-format json` with a valid
      JSON runbook on stdin, the command validates stdin and returns exit code
      `0`.
- [x] Given `sw validate --input-file=- --input-format yaml --output-format json`
      with a valid YAML runbook on stdin, the command validates stdin and
      returns exit code `0`.
- [x] Given `sw validate --input-file=- --output-format json` with YAML on
      stdin and no `--input-format=yaml`, the command exits with `1` and
      reports a clear parsing error.
- [x] Given `--input-format=json` or `--input-format=yaml` without
      `--input-file=-`, the command still uses the existing default file lookup
      behavior, including ambiguity failures when multiple default runbooks
      exist.
- [x] Given no `--output-format` option, command uses `human` output by
      default.
- [x] Given a human-readable validation failure for `entries[N]`, the output
      includes a nearby offending block for that error.
- [x] Validation performs no write operations (read-only behavior).

## Non-goals

- Executing runbook steps.
- Applying patches or mutating files.
- Generating README output.
- Detecting documentation drift or command correctness on target platforms.

## Edge Cases

- Empty input file.
- Invalid JSON syntax.
- Invalid YAML syntax.
- Unknown top-level keys.
- Missing required fields.
- A `Command` entry that appears to start a background process with `&`
  without redirecting stdout and stderr away from the captured command pipes.

## Test Cases

- Valid minimal runbook fixture.
- Invalid JSON fixture.
- Missing required field fixture.
- File path that does not exist.
- Runbook fixture with a background command warning.

## Warnings

- Validation may emit non-blocking warnings for runbook patterns that are
  structurally valid but likely to cause confusing runtime behavior.
- In this increment, validation warns when a `Command` entry appears to start
  a background process with `&` without redirecting stdout and stderr away
  from the command pipes.
- This warning explains that the background process may keep the entry open
  until it exits or the timeout is reached, which can make progress behavior
  confusing.
- The warning recommends redirecting output to a file and saving `$!` to a PID
  file when the process needs to keep running across later steps.
- In this increment, validation also warns when a `DisplayFile` entry uses a
  negative `offset` that cannot be fully applied to all non-empty copied file
  content lines.
- This warning explains that some lines have fewer leading spaces than the
  requested negative offset and that the content shift will therefore be only
  partially applied on those lines.

## Notes for Reimplementation

This feature is the first contract slice for agent-first usage and should remain
stable as other commands are added.
