---
id: TASK-022
title: Support Markdown Interpolation For Earlier Captures
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Allow `Markdown` entries to interpolate variables that were already captured
earlier in the runbook.

## Scope

- Support `@{name}` interpolation in `Markdown` entries
- Only allow interpolation when the variable was captured earlier in the
  runbook
- Preserve `@@{name}` as an escape for literal text
- Keep deferred Markdown interpolation for later-captured values out of scope

## Assumptions

- This task does not require deferred rendering because only earlier captures
  are in scope.
- Command interpolation rules remain unchanged.

## Acceptance Criteria

- [x] Given a `Markdown` entry that uses `@{name}` after that variable is
      captured earlier in the runbook, the rendered Markdown includes the
      captured value.
- [x] Given a `Markdown` entry that uses `@{name}` before that variable is
      captured, the run fails or validation rejects the runbook according to
      the final implementation choice.
- [x] Given `@@{name}` in a `Markdown` entry, the literal `@{name}` is
      preserved.
- [x] Deferred Markdown interpolation for variables captured later remains out
      of scope for this task.

## Notes

This task provides the simpler Markdown interpolation model first, matching the
same ordering discipline already used for command interpolation.
