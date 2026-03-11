---
id: TASK-002
title: Implement SPEC-002 Validate Command
status: pending
related_features:
  - SPEC-002
owner: @aattard
created: 2026-03-05
updated: 2026-03-05
---

## Summary

Implement runbook input validation for `sw validate`.

## Scope

- `sw validate --input-file <runbook.json> --output-format <human|json>`
- Default file behavior: use `./sw-runbook.json` when `--file` is omitted
- Read-only validation only (no writes)
- Exit code contract (`0`, `1`, `2`)

## Assumptions

- Supported output formats in v1: `human` and `json`.
- Schema details will be defined by SPEC-002 fixtures/examples.

## Acceptance Criteria

- [ ] Valid runbook returns success (`0`).
- [ ] Invalid runbook returns validation failure (`2`) with structured errors.
- [ ] Missing file returns operational error (`1`).
- [ ] No file argument uses `./sw-runbook.json`.

## Notes

This task should start after SPEC-001 baseline help behavior is confirmed.
