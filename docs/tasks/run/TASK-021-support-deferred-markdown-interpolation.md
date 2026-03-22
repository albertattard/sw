---
id: TASK-021
title: Support Deferred Markdown Interpolation
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-22
---

## Summary

Allow `Markdown` entries to interpolate captured variables even when those
variables are only captured later in the runbook.

## Scope

- Support `@{name}` interpolation in `Markdown` entries
- Allow `Markdown` entries to resolve variables captured later in the runbook
- Preserve `@@{name}` as an escape for literal text
- Keep command interpolation rules unchanged: command references must still
  refer to variables captured earlier in the runbook

## Assumptions

- This feature requires deferred Markdown rendering or an equivalent two-phase
  rendering approach.
- Deferred interpolation applies to Markdown rendering only, not to command
  execution ordering.

## Acceptance Criteria

- [x] Given a `Markdown` entry that uses `@{name}` before a later command
      captures that variable, the rendered Markdown includes the captured
      value.
- [x] Given `@@{name}` in a `Markdown` entry, the literal `@{name}` is
      preserved.
- [x] Given a `Markdown` interpolation reference that is never captured
      anywhere in the runbook, the run fails.
- [x] Given a command that references `@{name}` before it is captured, that
      command reference remains invalid.

## Notes

This task is separate from command capture because it changes the document
rendering model: Markdown can no longer be treated as a strict one-pass
top-to-bottom render when it depends on values only discovered later.
