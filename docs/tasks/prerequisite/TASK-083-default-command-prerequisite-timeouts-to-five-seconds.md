---
id: TASK-083
title: Default Command Prerequisite Timeouts To Five Seconds
status: done
category: prerequisite
related_features:
  - SPEC-003
  - SPEC-005
owner: @aattard
created: 2026-03-24
updated: 2026-03-24
---

## Summary

Make command-based prerequisite checks fail fast by default so `sw check`
surfaces stuck environment probes quickly without shortening normal workflow
commands.

## Scope

- Keep the existing `2 minutes` default timeout for `Command` entries
- Default `Prerequisite` checks with `kind: "command"` to `5 seconds`
- Continue honoring explicit `timeout` values on command-based prerequisites
- Update help, explain output, and automated coverage for the new default

## Assumptions

- `sw check` should behave like a readiness probe and not wait as long as
  normal workflow execution.
- Java prerequisite checks keep their built-in behavior and do not use the
  command-based default timeout.
- Existing runbooks can opt into slower prerequisite checks by declaring an
  explicit `timeout`.

## Acceptance Criteria

- [x] Given a command-based prerequisite check without `timeout`, `sw check`
      treats it as `5 seconds`.
- [x] Given a command-based prerequisite check with an explicit `timeout`,
      that timeout overrides the default.
- [x] Given a normal `Command` entry without `timeout`, `sw run` still treats
      it as `2 minutes`.
- [x] Help and explain output document the timeout split between `check` and
      `run`.

## Notes

This keeps environment validation responsive while preserving the current
headroom for longer workflow commands.
