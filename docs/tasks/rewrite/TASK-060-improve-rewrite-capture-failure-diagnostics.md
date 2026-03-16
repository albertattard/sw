---
id: TASK-060
title: Improve Rewrite Capture Failure Diagnostics
status: open
category: rewrite
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-15
updated: 2026-03-15
---

## Summary

Make rewrite `capture_as` failures easier to diagnose by reporting the command
context and captured output when a rewrite matches the wrong number of values.

## Scope

- Update rewrite `capture_as` failure handling to include the failing
  `Command` entry
- Include the captured stdout and stderr in rewrite `capture_as` failure output
- Preserve the current rule that `capture_as` must match exactly one value
- Add CLI coverage for a rewrite `capture_as` failure that matches zero values

## Assumptions

- The failure remains a run failure with exit code `2`.
- The diagnostic format should stay aligned with existing command assertion
  failure reporting where practical.

## Acceptance Criteria

- [ ] Given a rewrite rule with `capture_as` that matches zero values,
      `sw run` exits with `2`, does not write a partial output file, and
      reports the failing `Command` entry together with captured stdout and
      stderr.
- [ ] Given a rewrite rule with `capture_as` that matches more than one value,
      `sw run` exits with `2`, does not write a partial output file, and
      reports the failing `Command` entry together with captured stdout and
      stderr.
- [ ] Existing successful rewrite `capture_as` behavior remains unchanged.

## Notes

This keeps rewrite diagnostics at the same level of usefulness as command
assertion diagnostics, which is especially helpful when matching command output
from tools such as `jcmd`.
