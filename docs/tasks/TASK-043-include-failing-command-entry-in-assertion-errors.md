---
id: TASK-043
title: Include Failing Command Entry in Assertion Errors
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Include the failing `Command` entry in `sw run` assertion failures so users can
identify which runbook step caused the error.

## Scope

- Extend command assertion failure messages to print the failing `Command` entry
- Cover both `assert.exit_code` mismatches and `assert.checks` failures
- Preserve the existing exit codes and partial-output handling

## Assumptions

- Printing the serialized runbook entry is a stable enough debugging contract
  for human-readable stderr output.
- This change applies to assertion failures only, not every operational error in
  the run pipeline.

## Acceptance Criteria

- [x] Given an `assert.exit_code` mismatch, stderr includes the failing
      `Command` entry.
- [x] Given an `assert.checks` failure, stderr includes the failing `Command`
      entry.
- [x] Given an assertion failure, the run still exits with `2`.
- [x] Given an assertion failure, no partial output file is written unless the
      failure is a timeout or cleanup failure under the existing rules.

## Notes

Implemented by formatting assertion failures with a pretty-printed copy of the
runbook entry so the offending command block is visible in stderr.
