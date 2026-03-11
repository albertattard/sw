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

Implement runbook rendering for `sw run`.

## Scope

- `sw` defaults to `run`
- `sw run --input-file <runbook.json>`
- `sw run --output-format markdown`
- `sw run --output-file <path>`
- Render supported runbook entries to Markdown in order
- Write generated output to `./readme.md` by default
- Do not execute commands from the runbook

## Assumptions

- Supported output formats in v1: `markdown`.
- `run` reuses the same runbook validation rules as `validate`.

## Acceptance Criteria

- [x] `sw` with no subcommand behaves the same as `sw run`.
- [x] Valid runbook input renders Markdown and exits with `0`.
- [x] Invalid runbook input exits with `2` and does not write a partial file.
- [x] Missing input file exits with `1`.
- [x] `--output-file` writes to the provided path.

## Notes

Implemented with integration coverage for default-command behavior, output file
generation, invalid runbook handling, and missing-file errors.
