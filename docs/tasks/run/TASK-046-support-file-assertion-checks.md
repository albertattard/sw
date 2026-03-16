---
id: TASK-046
title: Support File Assertion Checks
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Allow command assertions to verify files created by a command by checking for
existence or a specific SHA-256 digest.

## Scope

- Support `source: file` inside `assert.checks`
- Require `path` for file assertions
- Support `exists: true`
- Support `sha256` as a lowercase hexadecimal digest
- Resolve file assertion paths from the same working directory as command
  execution
- Reject invalid combinations such as missing operators or multiple operators
- Add validation and integration coverage for success and failure cases

## Assumptions

- File assertions apply after the command finishes, using the filesystem state
  left by that command.
- A `sha256` assertion implicitly requires the file to exist.

## Acceptance Criteria

- [x] Given a `file` assertion with `exists: true` and an existing file, the
      run continues.
- [x] Given a `file` assertion with `exists: true` and a missing file, the run
      exits with `2`.
- [x] Given a `file` assertion with `sha256` and a matching digest, the run
      continues.
- [x] Given a `file` assertion with `sha256` and a non-matching digest, the run
      exits with `2`.
- [x] Given an invalid `file` assertion shape, validation fails.

## Notes

This makes it possible to validate downloaded artifacts and other file-based
side effects directly in the command assertion model instead of forcing those
checks into shell logic.
