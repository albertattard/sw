---
id: TASK-105
title: Accept Scalar Cleanup Scripts
status: pending
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-14
updated: 2026-04-14
---

## Summary

Accept `Command.cleanup` as either a single string or an array of strings so
YAML runbooks can use block scalars for cleanup scripts in the same way they
already can for `commands`.

## Scope

- Allow `Command.cleanup` to be either a string or an array of strings
- Normalize the accepted scalar form into the existing line-array cleanup model
- Drop the implicit terminal blank line that YAML literal scalars add by
  default so scalar cleanup scripts match the explicit array form
- Preserve existing array-based cleanup behavior for JSON and YAML runbooks
- Add or update automated tests for `validate` and `run`

## Assumptions

- Scalar `cleanup` means one cleanup script body that is normalized into the
  existing cleanup line-array model, not a new execution mode.
- Existing array-based cleanup declarations remain valid and unchanged.
- This increment is limited to `Command.cleanup` and does not expand scalar
  shorthand to unrelated fields.

## Acceptance Criteria

- [ ] Given a runbook whose `Command.cleanup` is a single string, `sw validate`
      accepts that runbook as valid input.
- [ ] Given `sw run` with scalar `Command.cleanup`, cleanup execution matches
      the existing line-array contract.
- [ ] Given scalar cleanup expressed as a YAML literal block with a terminal
      line break, no extra blank cleanup line is executed.
- [ ] Existing array-based cleanup runbooks continue to pass automated tests.

## Notes

This extends the scalar script ergonomics introduced for `commands` to cleanup
blocks without changing cleanup ordering or failure semantics.
