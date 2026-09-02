---
id: TASK-189
title: Mark Truncated Display Excerpts
status: done
category: display-file
related_features:
  - SPEC-003
owner: albertattard
created: 2026-09-01
updated: 2026-09-01
---

## Summary

Make bounded `DisplayFile` and `DisplayUrl` excerpts visibly incomplete when
their `line_count` leaves trailing source content unrendered.

## Scope

- Add `...` markers inside the generated fenced block on each boundary where a
  bounded `line_count` range omits source content
- Add optional `show_trim_markers`, defaulting to `true`, so authors can
  suppress those markers explicitly
- Keep the marker outside copied content so `offset` does not modify it
- Preserve existing output when the requested range reaches the source end
- Keep `DisplayFile` and `DisplayUrl` line-slicing behavior aligned
- Update the runbook specification, entry guide, help, and automated coverage

## Assumptions

- A marker at the end of a bounded excerpt communicates omitted trailing
  content without changing the runbook input format.
- The marker is presentation metadata, not copied file or response content.

## Acceptance Criteria

- [x] Given `DisplayFile` uses `start_line` and `line_count` to omit leading
      and trailing file lines, `sw run`
      renders `...` before and after the copied lines inside the fenced block.
- [x] Given `DisplayUrl` uses `start_line` and `line_count` to omit leading
      and trailing response lines, `sw run`
      renders `...` before and after the copied lines inside the fenced block.
- [x] Given a requested range reaches either source boundary, no marker is
      rendered on that boundary.
- [x] Given `show_trim_markers: false`, no markers are rendered.
- [x] Given `offset`, the marker remains exactly `...` while copied lines use
      the requested offset.
- [x] Specification, guidance, and CLI help describe the behavior.
- [x] Automated tests pass after the change.
