---
id: TASK-140
title: Report Total Verbose Run Time
status: done
category: run
related_features:
  - SPEC-003
owner: albertattard
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Add a final total elapsed-time summary to verbose run output so users can see
how long the whole run took after processing stops.

## Scope

- Emit `Total run time: <duration>` to stderr when `sw run --verbose` is used
- Use the existing readable elapsed-time formatting
- Keep stdout unchanged
- Emit the total time for successful runs and runs that stop because of
  failures, timeouts, cleanup failures, patch restore failures, or breakpoints
- Add CLI coverage for successful and failing verbose runs
- Surface the behavior through help and explain output

## Assumptions

- The summary line is informational and does not change run exit codes.
- The total timer measures run processing from the start of runbook rendering
  and execution through cleanup and patch restoration.

## Acceptance Criteria

- [x] Given `sw run --verbose` for a successful run, stderr includes
      `Total run time:`.
- [x] Given `sw run --verbose` for a failing command, stderr still includes
      `Total run time:`.
- [x] Given `sw run --verbose`, stdout remains unchanged.
- [x] Help and explain output mention the total elapsed-time summary.
