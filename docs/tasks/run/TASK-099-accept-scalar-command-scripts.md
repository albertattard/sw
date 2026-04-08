---
id: TASK-099
title: Accept Scalar Command Scripts
status: completed
category: run
related_features:
  - SPEC-002
  - SPEC-003
  - SPEC-005
owner: @aattard
created: 2026-04-08
updated: 2026-04-08
---

## Summary

Accept `Command.commands` and command-based prerequisite `commands` as either a
single string or an array of strings so YAML runbooks can use block scalars
for multi-line shell scripts without rewriting them into explicit arrays.

## Scope

- Allow `Command.commands` to be either a string or an array of strings
- Allow `Prerequisite.checks[*].commands` to be either a string or an array of
  strings
- Normalize the accepted scalar form into the existing line-array model before
  rendering, execution, and warning analysis
- Drop the implicit terminal blank line that YAML literal scalars add by
  default so scalar command scripts match the explicit array form
- Preserve existing array-based behavior for JSON and YAML runbooks
- Update discovery text for the new shorthand
- Add or update automated tests for `validate`, `run`, and `check`

## Assumptions

- Scalar `commands` means one command script body that is normalized into the
  existing line-array model, not a new execution mode.
- Existing array-based command declarations remain valid and unchanged.
- This increment does not expand scalar shorthand to `cleanup`, `patch`, or
  other execution-oriented arrays.

## Acceptance Criteria

- [x] Given a runbook whose `Command.commands` is a single string, `sw
      validate` accepts that runbook as valid input.
- [x] Given a runbook whose `Prerequisite.checks[*].commands` is a single
      string, `sw validate` accepts that runbook as valid input.
- [x] Given `sw run` with scalar `Command.commands`, the generated Markdown and
      command execution match the existing line-array contract.
- [x] Given `sw run` with scalar prerequisite `commands`, prerequisite
      execution matches the existing line-array contract.
- [x] Given `sw check` with scalar prerequisite `commands`, prerequisite
      execution still follows the current contract and exit codes.
- [x] Existing array-based runbooks continue to pass automated tests.

## Notes

This change improves YAML authoring ergonomics without introducing a separate
script-execution model for scalar command blocks.
