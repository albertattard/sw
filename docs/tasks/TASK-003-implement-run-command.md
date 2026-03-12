---
id: TASK-003
title: Implement SPEC-003 Run Command
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-11
updated: 2026-03-11
---

## Summary

Implement runbook execution and rendering for `sw run`.

## Scope

- `sw` defaults to `run`
- `sw run --input-file <runbook.json>`
- `sw run --output-format markdown`
- `sw run --output-file <path>`
- Render supported runbook entries to Markdown in order
- Execute command entries in order
- Fail when a command exits with an error
- Include command output only when requested by the `output` property
- Write generated output to `./readme.md` by default

## Assumptions

- Supported output formats in v1: `markdown`.
- `run` reuses the same runbook validation rules as `validate`.

## Acceptance Criteria

- [x] `sw` with no subcommand behaves the same as `sw run`.
- [x] Valid runbook input renders Markdown and exits with `0`.
- [x] Invalid runbook input exits with `2` and does not write a partial file.
- [x] Missing input file exits with `1`.
- [x] `--output-file` writes to the provided path.
- [x] Command entries are executed in order.
- [x] Command failures exit with `2` and do not write a partial output file.
- [x] Command output is included only when the `output` property is present.

## Notes

Implemented with integration coverage for command execution order, failure
handling, conditional output capture, and default-command behavior.
