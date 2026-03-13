---
id: TASK-030
title: Support Keep Between Trim Markers
status: pending
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Enhance `keep_between` so trimmed output shows visible marker lines by default,
with an opt-out for cases where the surrounding omission should remain hidden.

## Scope

- Support `show_trim_markers` under `keep_between`
- Default `show_trim_markers` to `true`
- Render `...` as standalone lines before and after the kept slice
- Support `show_trim_markers: false` to suppress those marker lines
- Leave output unchanged when either boundary is not found

## Assumptions

- Trim markers are a presentation concern within the existing rewrite pipeline.
- The marker text is fixed as `...` in this increment.
- Trim markers are only added when the boundary match succeeds.

## Acceptance Criteria

- [ ] Given a `keep_between` rewrite rule without explicit
      `show_trim_markers`, the rendered kept slice is wrapped in `...` marker
      lines.
- [ ] Given a `keep_between` rewrite rule with `show_trim_markers: false`,
      the rendered kept slice is not wrapped in marker lines.
- [ ] Given a `keep_between` rewrite rule whose `start` or `end` boundary is
      not found, no trim markers are added and the output remains unchanged.

## Notes

This keeps trimmed output visually honest by default without forcing users to
manually add explanatory marker lines around extracted sections.
