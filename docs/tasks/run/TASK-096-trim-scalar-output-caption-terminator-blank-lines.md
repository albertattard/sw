---
id: TASK-096
title: Trim Scalar Output Caption Terminator Blank Lines
status: completed
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-08
updated: 2026-04-08
---

## Summary

Make scalar `Command.output.caption` behave like the existing line-array form
by ignoring the terminal line break that YAML literal scalars add by default,
so captions do not create an extra blank line before the rendered output
fence.

## Scope

- Normalize scalar `output.caption` using the same terminator-trimming
  behavior already used for scalar prose content
- Preserve existing array-based caption behavior
- Add or update automated rendering coverage for scalar captions

## Assumptions

- This increment changes rendering only; it does not change command execution
  or output rewrite semantics.
- Explicit blank lines that users author inside the caption should still be
  preserved.

## Acceptance Criteria

- [x] Given a `Command` entry whose `output.caption` is a scalar string, the
      generated Markdown renders that caption before the captured command
      output.
- [x] Given a scalar `output.caption` value that ends with a line break only
      because of YAML literal-scalar termination, the generated Markdown does
      not introduce an extra blank line before the output fence.
- [x] Existing array-based `output.caption` behavior continues to pass
      automated tests.

## Notes

This closes the same YAML-scalar normalization gap that previously affected
`Markdown.contents`, but for command output captions.
