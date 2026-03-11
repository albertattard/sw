---
id: TASK-002
title: Implement SPEC-002 Validate Command
status: done
related_features:
  - SPEC-002
owner: @aattard
created: 2026-03-05
updated: 2026-03-11
---

## Summary

Implement runbook input validation for `sw validate`.

## Scope

- `sw validate --input-file <runbook.json> --output-format <human|json>`
- Default file behavior: use `./sw-runbook.json` when `--input-file` is omitted
- Read-only validation only (no writes)
- Exit code contract (`0`, `1`, `2`)

## Assumptions

- Supported output formats in v1: `human` and `json`.
- Schema details will be defined by SPEC-002 fixtures/examples.

## Acceptance Criteria

- [x] Valid runbook returns success (`0`).
- [x] Invalid runbook returns validation failure (`2`) with structured errors.
- [x] Missing file returns operational error (`1`).
- [x] No file argument uses `./sw-runbook.json`.

## Notes

Implemented with integration coverage for valid, invalid, and missing-file
scenarios.
