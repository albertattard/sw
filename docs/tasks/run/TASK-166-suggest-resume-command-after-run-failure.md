---
id: TASK-166
title: Suggest Resume Command After Run Failure
status: done
category: run
related_features:
  - SPEC-003
owner: albertattard
created: 2026-06-09
updated: 2026-06-09
---

## Summary

Print a resume command when `sw run` fails at a concrete runbook entry, so long
debugging runs can be restarted from the failed entry without manually
translating verbose progress output into CLI flags.

## Scope

- Track the 1-based runbook entry number for command, timeout, patch, and other
  entry-scoped run failures
- Print a final `Run: sw run ... --start-at <entry-number> to resume from this
  entry` hint on stderr
- Preserve applicable original run options in the hint
- Use the existing `--start-at` flag name
- Avoid printing resume hints for validation failures or failures that occur
  before a concrete runbook entry starts

## Assumptions

- The hint is a debugging convenience and does not guarantee that skipped
  earlier entries are safe to omit; users still need preserved or otherwise
  reproducible state.
- The explicit `sw run` form is clearer for hints than reproducing the implicit
  top-level `sw` invocation shape.

## Acceptance Criteria

- [x] Given a command failure at entry 2, stderr includes
      `Run: sw run --start-at 2 to resume from this entry`.
- [x] Given `--verbose --preserve-on-failure` and a command failure at entry 2,
      stderr includes those options before `--start-at 2`.
- [x] Given run input/output options and a command failure, the resume hint
      preserves those options.
- [x] Given a timeout at a concrete entry, stderr includes a resume hint for
      that entry.
- [x] Given a validation failure before execution, stderr does not include a
      resume hint.
