---
id: TASK-020
title: Support Captured Output Variables
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Allow commands to capture named values from stdout and reuse those values in
later commands.

## Scope

- Support `capture` on `Command` entries
- Support capture from `stdout`
- Support `stage: raw` and `stage: rewritten`
- Support `@{name}` interpolation in later command lines
- Support `@@{name}` as an escape for literal text
- Validate capture name uniqueness and reference ordering across the runbook

## Assumptions

- Captured variables are runbook-wide and follow declaration order.
- In this increment, capture stores exactly one value per rule.
- Rewritten capture uses the same rewritten stdout that would be rendered.

## Acceptance Criteria

- [x] Given a command with `capture`, the named value is stored for later use.
- [x] Given `stage: raw`, capture runs before output rewrite rules.
- [x] Given `stage: rewritten`, capture runs after output rewrite rules.
- [x] Given a later command with `@{name}`, the captured value is interpolated
      before execution.
- [x] Given duplicate capture names anywhere in the runbook, validation rejects
      the runbook.
- [x] Given a variable reference before the variable is captured, validation
      rejects the runbook.
- [x] Given a capture rule that matches zero or multiple values, the run fails.
- [x] Given `@@{name}`, the literal `@{name}` is preserved without
      interpolation.

## Notes

This task makes multi-step runbooks easier to automate when a command emits a
useful value such as a generated path, token, or identifier that later steps
need to consume.
