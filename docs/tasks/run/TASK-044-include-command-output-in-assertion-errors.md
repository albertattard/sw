---
id: TASK-044
title: Include Command Output In Assertion Errors
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Include captured command stdout and stderr in `sw run` assertion failures so
users can diagnose failing runbook steps from the reported error alone.

## Scope

- Extend command assertion failure messages to print captured stdout and stderr
- Preserve the existing failing `Command` entry in assertion errors
- Cover both `assert.exit_code` mismatches and `assert.checks` failures
- Preserve the existing exit codes and partial-output handling

## Assumptions

- Human-readable stderr output can include full captured stdout and stderr for a
  failing command without introducing a separate structured error format.
- Empty stdout or stderr should still be reported clearly so users can tell the
  stream was captured but had no content.

## Acceptance Criteria

- [x] Given an `assert.exit_code` mismatch, stderr includes the failing
      `Command` entry together with captured stdout and stderr.
- [x] Given an `assert.checks` failure, stderr includes the failing `Command`
      entry together with captured stdout and stderr.
- [x] Given a failing command with empty stdout or stderr, the error output
      makes the empty stream explicit.
- [x] Given an assertion failure, the run still exits with `2`.
- [x] Given an assertion failure, no partial output file is written unless the
      failure is a timeout or cleanup failure under the existing rules.

## Notes

Implemented by extending assertion-failure formatting to print the serialized
runbook entry and both captured command streams in stderr.
