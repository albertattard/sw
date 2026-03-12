---
id: TASK-013
title: Trim Command Output Trailing Whitespace
status: pending
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Trim trailing whitespace from rendered command output by default while allowing
runbooks to preserve exact output when needed.

## Scope

- Support `output.trim_trailing_whitespace`
- Default trailing-whitespace trimming to enabled when `output` is present
- Trim only trailing whitespace at the end of each output line
- Preserve leading whitespace
- Allow trimming to be disabled with `output.trim_trailing_whitespace: false`

## Assumptions

- This option affects rendering only and does not change command execution or
  assertions.
- The option applies only to rendered command output, not to the shell command
  block itself.

## Acceptance Criteria

- [ ] Given a `Command` entry with `output` and no
      `trim_trailing_whitespace`, trailing whitespace is removed from the end
      of each rendered output line.
- [ ] Given a `Command` entry with `output.trim_trailing_whitespace: false`,
      trailing whitespace is preserved in rendered output.
- [ ] Leading whitespace remains unchanged in both modes.

## Notes

This task reduces noisy whitespace in generated documentation while still
allowing exact output rendering when whitespace carries meaning.
