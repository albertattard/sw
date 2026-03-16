---
id: TASK-063
title: Separate DisplayFile Indent And Offset
status: done
category: display-file
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-15
updated: 2026-03-15
---

## Summary

Split `DisplayFile` block placement from snippet content shifting so fenced
blocks can be nested cleanly while copied file contents can still be shifted
left or right independently.

## Scope

- Change `DisplayFile.indent` to a non-negative block-level indentation value
- Add `DisplayFile.offset` as a signed content-level shift for copied file
  lines inside the fenced block
- Render `indent` across the whole fenced block, including fences
- Render `offset` only across copied file content lines
- Update validation for the new `indent` and `offset` rules
- Add a non-blocking validation warning when a negative `offset` cannot be
  fully applied to all non-empty copied file content lines
- Add CLI coverage for block indentation and content offset behavior

## Assumptions

- `indent` and `offset` serve different purposes and should not be overloaded
  onto one property.
- Blank copied file content lines should remain blank when `offset` is
  applied.

## Acceptance Criteria

- [x] Given a `DisplayFile` entry with `indent: 3`, the opening fence, content
      lines, and closing fence are all indented by three spaces.
- [x] Given a `DisplayFile` entry with `offset: -8`, up to eight leading
      spaces are removed from each non-empty copied file content line.
- [x] Given a `DisplayFile` entry with `offset: 4`, each non-empty copied file
      content line is prefixed with four spaces inside the fenced block.
- [x] Validation rejects negative `DisplayFile.indent` values.
- [x] Validation accepts signed integer `DisplayFile.offset` values.
- [x] Validation warns, without failing, when a negative `DisplayFile.offset`
      cannot be fully applied to all non-empty copied file content lines.

## Notes

This should replace the previous mixed `indent` behavior, which combined block
placement and content shifting in one property and made fenced block nesting
confusing.
