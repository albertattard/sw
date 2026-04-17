---
id: TASK-118
title: Implement Convert Command
status: pending
category: format
related_features:
  - SPEC-012
owner: @aattard
created: 2026-04-17
updated: 2026-04-17
---

## Summary

Add a `convert` command that converts valid runbook files between JSON and YAML
using the project's shared parsing, validation, and canonical serialization
paths.

## Scope

- Add a `convert` CLI command
- Support `--input-file <runbook.{json|yaml|yml}>`
- Support `--output-file <runbook.{json|yaml|yml}>`
- Support `--output-format json|yaml`
- Support `--force` for overwriting an existing target file
- Reuse shared default runbook discovery across `sw-runbook.json`,
  `sw-runbook.yaml`, and `sw-runbook.yml`
- Reuse shared runbook validation before writing converted output
- Derive the default target path from the source file when `--output-file` is
  omitted
- Reject same-format conversions and in-place output paths
- Add CLI coverage for implicit input detection, derived output paths,
  ambiguity handling, overwrite behavior, and invalid input handling
- Update help output and help-focused tests for the new command

## Assumptions

- This increment is file-to-file only; stdin conversion is intentionally left
  out to keep the initial contract predictable.
- Conversion is semantic rather than presentation-preserving, so YAML comments
  and other source-format-specific authoring details will be lost.
- JSON output should follow the existing pretty-printed contract and YAML
  output should follow the same canonical YAML formatting contract used by
  `sw format`.
- `convert` should fail fast when asked to produce the same format because
  `format` already owns canonicalization within a format.

## Acceptance Criteria

- [ ] Given only `./sw-runbook.json`, `sw convert` writes `./sw-runbook.yaml`
      and exits with `0`.
- [ ] Given only `./sw-runbook.yaml`, `sw convert` writes `./sw-runbook.json`
      and exits with `0`.
- [ ] Given `sw convert --input-file example.json`, the command writes
      `example.yaml` and exits with `0`.
- [ ] Given `sw convert --input-file example.yaml`, the command writes
      `example.json` and exits with `0`.
- [ ] Given an explicit output path, the command writes the converted runbook
      to that path.
- [ ] Given conflicting `--output-file` and `--output-format`, the command
      exits with `1`.
- [ ] Given an existing output path without `--force`, the command exits with
      `1` and does not overwrite the file.
- [ ] Given invalid syntax in the source file, the command exits with `1` and
      does not write the output file.
- [ ] Given valid syntax but invalid runbook structure, the command exits with
      `2` and does not write the output file.
- [ ] Given a requested target format equal to the source format, the command
      exits with `1`.
- [ ] Help output documents the `convert` command and its options.

## Notes

The highest risk in this increment is not serialization itself but ambiguous
default behavior. The implementation should keep the no-argument workflow
convenient while failing clearly whenever more than one interpretation is
possible.
