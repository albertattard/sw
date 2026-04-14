---
id: TASK-108
title: Support Command Working Directory
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-15
updated: 2026-04-15
---

## Summary

Allow a `Command` entry to declare `working_dir` so command steps can execute
from a subdirectory relative to the runbook file.

## Scope

- Accept `Command.working_dir` as a string
- Resolve `working_dir` relative to the runbook file's directory
- Apply `working_dir` consistently to command execution, explicit cleanup, and
  file assertions for that command entry
- Reject absolute paths and normalized paths that escape the runbook directory
- Add automated validation and run coverage

## Assumptions

- Authors need to run commands from nested project directories without changing
  the meaning of unrelated runbook paths.
- The working-directory contract should be enforced by process execution
  settings rather than shell-prefixed `cd ... &&` wrappers.
- Relative paths for file assertions should stay aligned with the directory in
  which the command actually ran.

## Acceptance Criteria

- [x] Given `working_dir: reverse-proxy`, `sw run` executes that command entry
      from the runbook-relative `reverse-proxy/` directory.
- [x] If `cleanup` is present on that command entry, cleanup runs from the same
      resolved working directory.
- [x] File assertions for that command entry resolve relative paths from the
      same working directory.
- [x] Absolute `working_dir` values are rejected by validation.
- [x] `working_dir` values that normalize outside the runbook directory are
      rejected by validation.

## Notes

This supports multi-directory examples without changing the meaning of
runbook-level paths such as `DisplayFile.path`, which remain anchored to the
runbook itself.
