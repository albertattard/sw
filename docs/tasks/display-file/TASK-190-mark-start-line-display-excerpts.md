---
id: TASK-190
title: Mark Start Line Display Excerpts
status: done
category: display-file
related_features:
  - SPEC-003
owner: albertattard
created: 2026-09-01
updated: 2026-09-01
---

## Summary

Mark `DisplayFile` and `DisplayUrl` excerpts that begin after line 1, even
when they continue to the source end without a `line_count`.

## Scope

- Add a leading `...` marker when `start_line` omits one or more leading lines
- Preserve trailing-marker behavior for bounded `line_count` ranges
- Honor `show_trim_markers: false`
- Keep file and URL display behavior aligned
- Update the specification, guide, and automated coverage

## Acceptance Criteria

- [x] Given `DisplayFile.start_line` greater than 1 without `line_count`, the
      generated fenced block begins with `...`.
- [x] Given `DisplayUrl.start_line` greater than 1 without `line_count`, the
      generated fenced block begins with `...`.
- [x] Given `show_trim_markers: false`, a start-line-only excerpt has no
      leading marker.
- [x] Automated tests pass after the change.
