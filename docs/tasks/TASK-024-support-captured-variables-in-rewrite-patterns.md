---
id: TASK-024
title: Support Captured Variables In Rewrite Patterns
status: pending
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Allow rewrite pattern text to interpolate variables that were already captured
earlier in the runbook.

## Scope

- Support `@{name}` interpolation inside `replace` rewrite rule `pattern` text
- Only allow interpolation when the variable was captured earlier in the
  runbook
- Support `@@{name}` as an escape for literal text
- Keep interpolation in `datetime_shift` patterns out of scope

## Assumptions

- Rewrite pattern interpolation follows the same earlier-only ordering
  discipline already used for command lines, same-pass Markdown
  interpolation, and rewrite replacements.
- Interpolation applies to `replace` rule `pattern` values only in this
  increment.

## Acceptance Criteria

- [ ] Given a `replace` rewrite rule `pattern` that uses `@{name}` after that
      variable is captured earlier in the runbook, the pattern text includes
      the captured value before matching.
- [ ] Given a `replace` rewrite rule `pattern` that uses `@{name}` before that
      variable is captured, validation rejects the runbook.
- [ ] Given `@@{name}` in a `replace` rewrite rule `pattern`, the literal
      `@{name}` is preserved without interpolation.
- [ ] `datetime_shift` `pattern` values remain literal in this task.

## Notes

This task lets rewrite matching target previously discovered values without
requiring hard-coded regex or literal text for every generated identifier,
while keeping the broader datetime-shift matching model unchanged for now.
