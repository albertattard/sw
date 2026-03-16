---
id: TASK-057
title: Support Multiline Cleanup Control Structures
status: open
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-15
updated: 2026-03-15
---

## Summary

Make cleanup execution support multi-line shell control structures such as
`if ... then ... fi` without requiring users to collapse them into one line.

## Scope

- Execute all lines in a cleanup block as one cleanup script
- Preserve the existing best-effort cleanup behavior across multiple cleanup
  blocks
- Preserve the existing behavior where later lines in the same cleanup block
  still run after an earlier line fails
- Add coverage for a multi-line `if ... then ... fi` cleanup block

## Assumptions

- Cleanup remains a list of strings so the runbook shape does not change.
- This is a runtime bug fix, not a schema change.

## Acceptance Criteria

- [ ] Given a cleanup block that expresses `if ... then ... fi` across multiple
      lines, cleanup executes successfully.
- [ ] Given a multi-line cleanup control structure, cleanup still runs in the
      same shell context.
- [ ] Given a failure inside one cleanup block, later registered cleanup blocks
      still execute.

## Notes

This aligns the cleanup runtime with the documented command-like shape of
cleanup blocks and removes the need for awkward one-line shell control
structures in runbooks.
