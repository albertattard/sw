---
id: TASK-005
title: Support Stdout Contains Assertions
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Support `assert.checks` for `stdout` `contains` assertions in `sw run`.

## Scope

- Allow a `Command` entry to declare `assert.checks`
- Support `source: stdout`
- Support the `contains` operator
- Require all declared checks to pass
- Return exit code `2` when a check fails
- Keep partial output file handling consistent with other run failures

## Assumptions

- `assert.exit_code` remains supported alongside `assert.checks`.
- `stdout` is the first supported assertion source for content checks.
- Future tasks may add sources such as files and operators such as regex or equals.

## Acceptance Criteria

- [x] Given a command with a `stdout` `contains` check and matching stdout, the run continues.
- [x] Given a command with a `stdout` `contains` check and non-matching stdout, the run exits with `2`.
- [x] Given multiple `stdout` `contains` checks, all checks must pass.
- [x] Given a failed `stdout` assertion, no partial output file is written.
- [x] Invalid `assert.checks` structure is rejected by validation.

## Notes

Implemented with validation and integration coverage for passing, failing, and
multi-check stdout `contains` assertions.
