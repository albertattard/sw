---
id: TASK-006
title: Support Command Timeouts
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Support per-command timeouts in `sw run`.

## Scope

- Allow a `Command` entry to declare `timeout`
- Use `2 minutes` as the default timeout when `timeout` is omitted
- Parse human-readable time values such as `30 seconds`, `1 minute`, and
  `5 minutes`
- Terminate commands that exceed their timeout
- Return exit code `2` on timeout
- Preserve captured output produced before timeout to aid debugging

## Assumptions

- Timeouts apply per `Command` entry, not to the whole runbook.
- Supported units in this increment are seconds and minutes.
- Timeout parsing remains human-readable rather than ISO duration based.

## Acceptance Criteria

- [x] Given a command without `timeout`, the default timeout of `2 minutes` is used.
- [x] Given a command with a supported human-readable timeout, that timeout is applied.
- [x] Given a command that finishes within its timeout, the run continues.
- [x] Given a command that exceeds its timeout, the process is terminated and the run exits with `2`.
- [x] Given a timed-out command, any captured output produced before termination is preserved to aid debugging.
- [x] Invalid timeout values are rejected by validation.

## Notes

Implemented with validation and integration coverage for default timeouts,
human-readable timeout parsing, process termination, and partial-output
preservation on timeout.
