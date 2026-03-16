---
id: TASK-037
title: Support DisplayFile Line Ranges
status: done
category: display-file
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Extend `DisplayFile` entries so a runbook can render only a selected portion of
the referenced file instead of always including the whole file.

## Scope

- Support optional `start_line` on `DisplayFile`
- Support optional `line_count` on `DisplayFile`
- Treat `start_line` as 1-based
- Render from `start_line` to end of file when `line_count` is omitted
- Reject `line_count` when `start_line` is missing
- Validate invalid values such as line numbers below `1`
- Add integration coverage for full-file, bounded-slice, and start-to-end cases

## Assumptions

- `DisplayFile` remains a rendering-only feature and does not execute file
  contents.
- Existing content-type detection continues to apply to sliced file output.

## Acceptance Criteria

- [x] Given a `DisplayFile` entry with `start_line`, rendering begins at that
      line.
- [x] Given a `DisplayFile` entry with `start_line` and `line_count`, only the
      requested number of lines is rendered.
- [x] Given a `DisplayFile` entry with `start_line` and no `line_count`,
      rendering continues to the end of the file.
- [x] Given `line_count` without `start_line`, validation fails.
- [x] Given `start_line` or `line_count` less than `1`, validation fails.

## Notes

This keeps large source files readable in generated documentation by allowing a
runbook to focus on the relevant excerpt rather than forcing the full file into
the output.
