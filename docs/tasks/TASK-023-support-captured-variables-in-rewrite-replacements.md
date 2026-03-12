---
id: TASK-023
title: Support Captured Variables In Rewrite Replacements
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Allow rewrite replacement text to interpolate variables that were already
captured earlier in the runbook.

## Scope

- Support `@{name}` interpolation inside `replace` rewrite rule
  `replacement` text
- Only allow interpolation when the variable was captured earlier in the
  runbook
- Support `@@{name}` as an escape for literal text
- Keep interpolation in rewrite `pattern` text out of scope

## Assumptions

- Rewrite replacement interpolation follows the same earlier-only ordering
  discipline already used for command lines and same-pass Markdown
  interpolation.
- Interpolation applies to `replace` rule `replacement` values only in this
  increment.

## Acceptance Criteria

- [x] Given a `replace` rewrite rule `replacement` that uses `@{name}` after
      that variable is captured earlier in the runbook, the replacement text
      includes the captured value.
- [x] Given a `replace` rewrite rule `replacement` that uses `@{name}` before
      that variable is captured, validation rejects the runbook.
- [x] Given `@@{name}` in a `replace` rewrite rule `replacement`, the literal
      `@{name}` is preserved without interpolation.
- [x] Rewrite rule `pattern` values remain literal in this task.

## Notes

This task lets output anonymisation and normalization reuse values discovered
earlier in the runbook without expanding interpolation into every part of the
rewrite model at once.
