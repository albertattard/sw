---
id: TASK-053
title: Treat Missing Process As No-op In Automatic Cleanup
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-15
updated: 2026-03-15
---

## Summary

Make automatic command process cleanup ignore already-exited process groups
without printing noisy warnings.

## Scope

- Treat missing processes or process groups as a successful automatic-cleanup
  no-op
- Suppress user-visible "No such process" noise from automatic cleanup
- Add CLI coverage for the race where cleanup runs after the process already
  exited

## Assumptions

- This applies only to automatic process cleanup, not to explicit manual
  `cleanup` commands.
- An already-exited process group does not represent a cleanup failure.

## Acceptance Criteria

- [x] Given automatic cleanup for a command whose process group has already
      exited, the run still succeeds if nothing else failed.
- [x] Given automatic cleanup for a command whose process group has already
      exited, stderr does not include a "No such process" warning.
- [x] Existing automatic cleanup behavior remains unchanged when the process
      group still exists and must be terminated.

## Notes

This removes misleading noise from successful runs and makes automatic cleanup
behave like a true best-effort teardown step.
