---
id: TASK-072
title: Support Output Empty Line Trimming
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-22
updated: 2026-03-22
---

## Summary

Add a first-class `trim_empty_lines` output option so runbooks can remove
leading and trailing blank command-output lines without relying on brittle
regex rewrite rules.

## Scope

- Accept `output.trim_empty_lines` on `Command` entries
- Support `leading_trailing`, `leading`, `trailing`, and `none`
- Apply empty-line trimming after output rewrites and before rendering
- Keep empty lines inside the retained body unchanged
- Treat whitespace-only lines as empty for trimming purposes
- Add validation, rendering, and example coverage

## Assumptions

- This option is independent from `trim_trailing_whitespace`
- The default should preserve current behavior for leading and trailing empty
  lines
- Users should not need a regex `replace` rule for this common output-cleanup
  case

## Acceptance Criteria

- [x] Given `output.trim_empty_lines: leading_trailing`, rendered output removes
      only leading and trailing empty lines.
- [x] Given `output.trim_empty_lines: leading`, rendered output removes only
      leading empty lines.
- [x] Given `output.trim_empty_lines: trailing`, rendered output removes only
      trailing empty lines.
- [x] Given `output.trim_empty_lines: none`, rendered output preserves leading
      and trailing empty lines.
- [x] Given an invalid `output.trim_empty_lines` value, validation rejects the
      runbook with a clear error.
- [x] The command example documents `trim_empty_lines` as part of the output
      contract.

## Notes

This increment adds a focused formatting control without changing the broader
rewrite model.
