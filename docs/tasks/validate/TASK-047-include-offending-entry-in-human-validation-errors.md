---
id: TASK-047
title: Include Offending Entry In Human Validation Errors
status: done
category: validate
related_features:
  - SPEC-002
  - SPEC-003
  - SPEC-005
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Include the offending runbook entry in human-readable validation failures so
users can identify which entry needs fixing without manually locating it from
the validation path alone.

## Scope

- Extend human-readable validation output to print offending `entries[N]`
- Print each offending runbook entry once even when multiple validation errors
  point at the same entry
- Preserve the existing machine-readable JSON validation output
- Apply the shared formatter to `sw validate`, `sw run`, and `sw check`

## Assumptions

- Validation paths outside `entries[N]` do not need extra entry context.
- Pretty-printed JSON is an acceptable human-readable way to show the offending
  runbook entry.

## Acceptance Criteria

- [x] Given `sw validate` in human mode with an `entries[N]` validation error,
      the output includes the offending runbook entry.
- [x] Given multiple validation errors for the same entry, the entry is printed
      once.
- [x] Given `sw run` with an invalid runbook, the shared validation output
      includes the offending runbook entry.
- [x] Given JSON validation output, the structured validation result remains
      unchanged.

## Notes

Implemented in the shared human validation formatter so all commands that
surface validation failures present the same entry-level debugging context.
