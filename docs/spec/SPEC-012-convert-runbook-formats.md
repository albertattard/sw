---
id: SPEC-012
title: Convert Runbook Formats
status: implemented
priority: medium
owner: @aattard
last_updated: 2026-04-17
---

## Problem

Users and agents can already read, validate, run, import, and format runbooks
 in both JSON and YAML, but there is no built-in way to convert an existing
 runbook file from one supported format to the other.

## User-facing Behavior

The CLI provides an explicit conversion command:

```bash
sw convert
sw convert --input-file sw-runbook.json
sw convert --input-file sw-runbook.yaml --output-file converted.json
```

The command converts a valid runbook file from JSON to YAML or from YAML/YML
to JSON and writes the converted file to a separate output path. The command
does not modify the source file in place.

If no input file is provided, the command uses the implicit default runbook
only when exactly one of these files exists:

- `./sw-runbook.json`
- `./sw-runbook.yaml`
- `./sw-runbook.yml`

When the input file is discovered implicitly and no output file is provided,
the command selects the opposite default runbook name:

- `sw-runbook.json` converts to `sw-runbook.yaml`
- `sw-runbook.yaml` converts to `sw-runbook.json`
- `sw-runbook.yml` converts to `sw-runbook.json`

If `--input-file` is provided and `--output-file` is omitted, the command
derives a sibling output path by replacing the input extension with the target
format’s canonical extension:

- `.json` converts to `.yaml`
- `.yaml` converts to `.json`
- `.yml` converts to `.json`

Before writing, the command must confirm that the source file parses
successfully and that the runbook passes the same structural validation
contract as `sw validate`. If parsing or validation fails, the command does
not write the output file.

## Inputs/Outputs

Input:
- Optional named input file parameter:
  `--input-file <runbook.{json|yaml|yml}>`.
- Optional named output file parameter:
  `--output-file <runbook.{json|yaml|yml}>`.
- Optional explicit target format parameter:
  `--output-format json|yaml`.
- Optional `--force` flag to allow overwriting an existing output file.

Default input behavior:
- If `--input-file` is provided, use that path.
- If no input file is provided and exactly one of `./sw-runbook.json`,
  `./sw-runbook.yaml`, or `./sw-runbook.yml` exists, use that file.
- If no input file is provided and more than one of `./sw-runbook.json`,
  `./sw-runbook.yaml`, or `./sw-runbook.yml` exists, return exit code `1` with
  a clear error that the default input is ambiguous and `--input-file` must be
  specified.
- If no input file is provided and none of the default runbook files exist,
  return exit code `1` with a clear missing-input error.

Default output behavior:
- If `--output-file` is provided, infer the target format from its file
  extension unless `--output-format` is also provided.
- If both `--output-file` and `--output-format` are provided and the
  recognized output-file extension conflicts with `--output-format`, return
  exit code `1` with a clear mismatch error.
- If `--output-file` is omitted, derive the output path from the input path by
  replacing `.json` with `.yaml` or replacing `.yaml`/`.yml` with `.json`.
- If `--output-format` is provided without `--output-file`, use it to choose
  the derived output extension.
- If the requested target format is the same as the source format, return exit
  code `1` with a clear error that `convert` requires the opposite format.
- If the derived or explicit output path matches the input path, return exit
  code `1` with a clear error that `convert` does not overwrite the source file
  in place.
- If the target file already exists and `--force` is not provided, return exit
  code `1` and leave the existing output file unchanged.

Output:
- On success, the command writes the converted runbook to the target path.
- On success, the command prints a short human-readable confirmation.
- On failure, the command prints a clear error message and leaves the source
  file unchanged.

### Exit Codes

- `0`: runbook file was converted successfully.
- `1`: operational or usage error (missing file, ambiguous default input,
  invalid JSON, invalid YAML, unsupported extension, same-format conversion,
  conflicting output format, existing output without `--force`, write failure,
  internal error).
- `2`: runbook file parses successfully but fails structural validation.

## Conversion Rules

- Conversion is file-to-file only in this increment.
- `sw convert` does not accept `--input-file=-`.
- Conversion preserves the semantic runbook content rather than preserving
  source-format-specific presentation details.
- JSON output uses pretty-printed JSON with two-space indentation.
- YAML output uses the same canonical YAML formatting contract as `sw format`,
  including indented sequences under mapping keys and a single blank line
  between adjacent top-level `entries`.
- Property order is preserved exactly as it appears in the parsed input
  document.
- The written output ends with a trailing newline.
- YAML-specific authoring details such as comments, anchors, aliases, and
  source scalar style are not preserved.
- The command does not repair invalid runbooks and does not apply schema
  migrations.

## Acceptance Criteria

- [x] Given only `./sw-runbook.json` present, `sw convert` writes
      `./sw-runbook.yaml` and exits with `0`.
- [x] Given only `./sw-runbook.yaml` present, `sw convert` writes
      `./sw-runbook.json` and exits with `0`.
- [x] Given only `./sw-runbook.yml` present, `sw convert` writes
      `./sw-runbook.json` and exits with `0`.
- [x] Given `sw convert --input-file example.json`, the command writes
      `example.yaml` and exits with `0`.
- [x] Given `sw convert --input-file example.yaml`, the command writes
      `example.json` and exits with `0`.
- [x] Given `sw convert --input-file example.yml`, the command writes
      `example.json` and exits with `0`.
- [x] Given `sw convert --input-file example.json --output-file converted.yaml`,
      the command writes YAML to `converted.yaml` and exits with `0`.
- [x] Given `sw convert --input-file example.yaml --output-file converted.json`,
      the command writes JSON to `converted.json` and exits with `0`.
- [x] Given no `--input-file` and more than one default runbook file present,
      `sw convert` exits with `1` and requires `--input-file`.
- [x] Given no `--input-file` and no default runbook file present, `sw convert`
      exits with `1` and reports that no input runbook was found.
- [x] Given `--output-file` and `--output-format` that disagree, `sw convert`
      exits with `1` and reports the mismatch.
- [x] Given an existing output file without `--force`, `sw convert` exits with
      `1` and does not overwrite the file.
- [x] Given an existing output file with `--force`, `sw convert` overwrites the
      file and exits with `0`.
- [x] Given an invalid JSON or YAML input file, `sw convert` exits with `1` and
      does not write the output file.
- [x] Given a structurally invalid runbook input file, `sw convert` exits with
      `2` and does not write the output file.
- [x] Given a requested target format that matches the source format,
      `sw convert` exits with `1` and reports that the opposite format is
      required.
- [x] Given an output path equal to the input path, `sw convert` exits with `1`
      and reports that in-place conversion is not supported.
- [x] Help output documents the `convert` command and its options.

## Non-goals

- In-place format conversion.
- Preserving YAML comments, anchors, aliases, or original scalar styling.
- Supporting stdin-to-stdout or stdin-to-file conversion in this increment.
- Supporting output formats beyond JSON and YAML.

## Notes for Reimplementation

This command should reuse the existing shared runbook parsing, validation, and
serialization logic. The main implementation work in this increment is command
contract enforcement, target-path resolution, and integration-style coverage.
