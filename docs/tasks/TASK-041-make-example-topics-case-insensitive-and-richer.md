---
id: TASK-041
title: Make Example Entity Types Case-Insensitive And Richer
status: done
related_features:
  - SPEC-008
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Refine the `example` command so entity-type matching is case-insensitive and
entry-level examples return fuller, more practical snippets instead of very
minimal shells.

## Scope

- Make example entity-type matching case-insensitive
- Expand the `Command` example so it includes common nested properties such as
  output and rewrite usage
- Reject unsupported nested feature requests such as rewrite-only topics
- Add integration coverage for lower-case entity-type usage
- Add integration coverage for richer `Command` example output

## Assumptions

- Entry-level examples should be broad enough to remove fields from, rather
  than forcing users to assemble the common shape from multiple smaller
  snippets.
- The command remains focused on full entry examples rather than nested
  feature examples.

## Acceptance Criteria

- [x] Given `sw example command`, the CLI behaves the same as
      `sw example Command`.
- [x] Given `sw example Command`, the CLI prints a fuller `Command` example
      that includes commonly used nested properties.
- [x] Given a nested rewrite topic such as `rewrite.replace`, the CLI exits
      with `1`.

## Notes

This keeps the `example` command focused on complete entry examples that users
can trim down locally instead of forcing them to assemble common shapes from
multiple smaller snippets.
