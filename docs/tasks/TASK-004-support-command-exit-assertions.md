---
id: TASK-004
title: Support Command Exit Assertions
status: pending
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Support `assert.exit_code` for command result validation in `sw run`.

## Scope

- Allow a `Command` entry to declare `assert.exit_code`
- Use `assert.exit_code` as the expected result when present
- Preserve the current default behavior when `assert` is absent
- Return exit code `2` when the asserted exit code does not match
- Keep partial output file handling consistent with other run failures

## Assumptions

- `assert.exit_code` is the first assertion capability added to `SPEC-003`.
- More assertion types may be added later in separate tasks.

## Acceptance Criteria

- [ ] Given a command without `assert`, exit code `0` is still required.
- [ ] Given a command with `assert.exit_code`, the asserted value overrides the default success expectation.
- [ ] Given a command whose actual exit code matches `assert.exit_code`, the run continues.
- [ ] Given a command whose actual exit code does not match `assert.exit_code`, the run exits with `2`.
- [ ] Given an asserted exit-code mismatch, no partial output file is written.

## Notes

This task is the first assertion increment for `SPEC-003` and should keep the
default command behavior unchanged for runbooks that do not declare assertions.
