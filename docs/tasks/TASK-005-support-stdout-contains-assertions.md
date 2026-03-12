---
id: TASK-005
title: Support Stdout Contains Assertions
status: pending
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

- [ ] Given a command with a `stdout` `contains` check and matching stdout, the run continues.
- [ ] Given a command with a `stdout` `contains` check and non-matching stdout, the run exits with `2`.
- [ ] Given multiple `stdout` `contains` checks, all checks must pass.
- [ ] Given a failed `stdout` assertion, no partial output file is written.
- [ ] Invalid `assert.checks` structure is rejected by validation.

## Notes

This task is the first content-assertion increment for `SPEC-003` and should
preserve the existing exit-code assertion behavior.
