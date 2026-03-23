---
id: TASK-080
title: Make Patch Application Non-Interactive
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-23
updated: 2026-03-23
---

## Summary

Make `Patch` entries fail fast when a patch cannot be applied cleanly so a run
does not hang waiting for interactive input from the external patch tool.

## Scope

- Run patch application in non-interactive mode
- Preserve the current patch success path and automatic restore behavior
- Fail the run with a clear operational error when patch application fails
- Avoid leaving `.orig` or `.rej` sidecar files behind after patch failures
- Add integration coverage for a patch that is already applied or otherwise
  cannot be applied cleanly

## Assumptions

- Patch application should be deterministic and automation-safe because
  runbooks are intended for both humans and agents.
- A failed patch should not silently reverse, skip, or partially apply changes.

## Acceptance Criteria

- [x] Given a `Patch` entry that cannot be applied cleanly, the run fails
      without waiting for interactive input from the patch tool.
- [x] Given a `Patch` entry that cannot be applied cleanly, the target file
      remains unchanged.
- [x] Given a `Patch` entry that cannot be applied cleanly, no `.orig` or
      `.rej` sidecar files are left behind.
- [x] Given a `Patch` entry that cannot be applied cleanly, the error output
      reports the target path and the patch-tool failure detail.

## Notes

This change is targeted at rerunnable examples and agent-driven workflows where
interactive patch prompts are operationally unsafe.
