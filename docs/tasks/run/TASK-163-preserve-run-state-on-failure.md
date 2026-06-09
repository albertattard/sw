---
id: TASK-163
title: Preserve Run State On Failure
status: done
category: run
related_features:
  - SPEC-003
owner: albertattard
created: 2026-06-09
updated: 2026-06-09
---

## Summary

Add a `sw run --preserve-on-failure` debugging flag that leaves registered
cleanup and patch restore state in place when a long run fails.

## Scope

- Add the `--preserve-on-failure` run flag
- Support the flag on explicit and implicit `run`
- Skip explicit command cleanup blocks after failure when the flag is present
- Skip automatic patch restoration after failure when the flag is present
- Keep normal cleanup and patch restoration on successful runs
- Keep breakpoint behavior successful, including normal cleanup and restore
- Keep timeout process-group termination active even when preserving failure
  state
- Report when cleanup or patch restoration was skipped to preserve failure
  state

## Assumptions

- This is a debugging/operator flag, not a runbook-authored default.
- Automatic timeout termination remains a safety boundary and is not disabled by
  this flag.

## Acceptance Criteria

- [x] Given a failing command with cleanup and `--preserve-on-failure`, cleanup
      does not run and the run exits with code `2`.
- [x] Given a successful run with `--preserve-on-failure`, cleanup still runs.
- [x] Given an applied patch followed by a failure and
      `--preserve-on-failure`, the patched file remains in its patched state.
- [x] Given a breakpoint and `--preserve-on-failure`, cleanup and patch restore
      still run.
- [x] Given preserved state after failure, stderr reports that run state was
      preserved.
- [x] Help and spec output document the debugging flag.
