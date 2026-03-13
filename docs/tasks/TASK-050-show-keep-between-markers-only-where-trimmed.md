---
id: TASK-050
title: Show Keep Between Markers Only Where Trimmed
status: open
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Adjust `keep_between` trim markers so `...` appears only on sides where output
was actually removed.

## Scope

- Update `keep_between` rendering to decide leading and trailing trim markers
  independently
- Preserve existing `show_trim_markers: false` behavior
- Add integration coverage for one-sided trim-marker output

## Assumptions

- A leading marker means lines before the kept slice were removed.
- A trailing marker means lines after the kept slice were removed.
- If nothing was removed on one side, that side shows no trim marker.

## Acceptance Criteria

- [ ] Given a `keep_between` rewrite rule that trims only lines before the kept
      slice, the output shows only a leading `...` marker.
- [ ] Given a `keep_between` rewrite rule that trims only lines after the kept
      slice, the output shows only a trailing `...` marker.
- [ ] Given a `keep_between` rewrite rule that trims both sides, the output
      shows both trim markers.
- [ ] Given `show_trim_markers: false`, no trim markers are rendered even when
      output was trimmed.

## Notes

This keeps trim markers honest to the actual rewrite result, which makes the
rendered output easier to trust when `keep_between` keeps the remainder of the
stream.
