---
id: TASK-006
title: Support Command Timeouts
status: pending
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
- Keep partial output file handling consistent with other run failures

## Assumptions

- Timeouts apply per `Command` entry, not to the whole runbook.
- Supported units in this increment are seconds and minutes.
- Timeout parsing remains human-readable rather than ISO duration based.

## Acceptance Criteria

- [ ] Given a command without `timeout`, the default timeout of `2 minutes` is used.
- [ ] Given a command with a supported human-readable timeout, that timeout is applied.
- [ ] Given a command that finishes within its timeout, the run continues.
- [ ] Given a command that exceeds its timeout, the process is terminated and the run exits with `2`.
- [ ] Given a timed-out command, no partial output file is written.
- [ ] Invalid timeout values are rejected by validation.

## Notes

This task bounds command execution so runaway processes do not outlive the run.
