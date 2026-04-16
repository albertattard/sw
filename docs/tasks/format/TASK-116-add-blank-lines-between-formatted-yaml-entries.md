---
id: TASK-116
title: Add Blank Lines Between Formatted YAML Entries
status: done
category: format
related_features:
  - SPEC-010
owner: @aattard
created: 2026-04-16
updated: 2026-04-16
---

## Summary

Make `sw format` emit a more editable YAML layout by inserting a blank line
between adjacent items in the top-level `entries` list.

## Scope

- Update the YAML formatting contract for `sw format`
- Keep JSON formatting unchanged
- Keep the change scoped to adjacent top-level `entries` items
- Preserve existing property order within each formatted entry
- Add automated CLI coverage for the YAML spacing rule

## Assumptions

- This is a formatting-style improvement for file-based authoring, not a schema
  change.
- The blank-line rule should apply only to the top-level `entries` list in this
  increment, rather than to every YAML sequence in the document.
- `sw format` should converge toward the same editing-friendly YAML layout that
  `sw import` already emits for imported runbooks.

## Acceptance Criteria

- [x] Given a YAML runbook with adjacent top-level `entries`, `sw format`
      rewrites the file so each adjacent entry is separated by exactly one
      blank line.
- [x] Given the same YAML runbook formatted twice, the second run produces no
      further layout change.
- [x] JSON formatting behavior remains unchanged.

## Notes

This keeps the formatter focused on readable authoring output without
introducing a broader YAML pretty-print policy for nested arrays that the spec
does not yet define.
