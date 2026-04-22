---
id: TASK-122
title: Shorten Default Command Timeout
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-22
updated: 2026-04-22
---

## Summary

Reduce the default timeout for normal `Command` entries so blocked or hung
commands fail faster during local development and agent-driven iteration.

## Scope

- Change the default `Command` timeout from `2 minutes` to `30 seconds`
- Keep explicit `timeout` values unchanged
- Keep the separate `5 seconds` default for command-based prerequisite checks
- Update help, explain, and automated tests that mention the default timeout

## Assumptions

- Commands that legitimately need longer than `30 seconds` should declare an
  explicit timeout in the runbook.
- Faster failure is more useful than waiting two minutes on a command that is
  blocked or otherwise stalled.
- This change does not alter timeout parsing or the behavior of explicit
  timeout values.

## Acceptance Criteria

- [x] Given a `Command` entry without `timeout`, `sw run` uses `30 seconds` as
      the default timeout.
- [x] Given a `Command` entry with an explicit timeout such as `1 minute`, that
      explicit timeout still overrides the default.
- [x] Given a command-based prerequisite check without `timeout`, `sw check`
      still uses the separate `5 seconds` default timeout.
- [x] Help and explain output document the new `30 seconds` default for normal
      `Command` entries.

## Notes

This is a user-visible behavior change and must stay aligned across runtime,
specs, and command-discovery output.
