---
id: TASK-110
title: Render Working Dir As Copyable Subshell
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-14
updated: 2026-04-14
---

## Summary

Render `Command.working_dir` in generated Markdown as a copy-pasteable shell
subshell wrapper so readers can execute the displayed command block from the
correct directory.

## Scope

- Leave runtime `working_dir` execution behavior unchanged
- Wrap rendered command blocks with a subshell when `working_dir` is present
- Render `cd '<working_dir>' &&` before the command lines
- Quote the rendered directory safely for shell copy-paste
- Add automated rendering coverage

## Assumptions

- Readers benefit more from a copy-pasteable rendered command block than from a
  separate explanatory line about the working directory.
- The rendered wrapper is presentation only; the internal execution model
  remains process-level `current_dir` handling.
- The subshell wrapper should preserve the user's original shell directory when
  pasted interactively.

## Acceptance Criteria

- [x] Given a `Command` entry with `working_dir`, the rendered Markdown command
      block starts with a subshell wrapper.
- [x] The rendered subshell uses `cd '<working_dir>' &&` before the command
      text.
- [x] The wrapped command block remains copy-pasteable as shell input.
- [x] Command entries without `working_dir` keep the existing rendered command
      shape.

## Notes

This makes the documentation output reflect the directory requirement without
changing how `sw` internally executes command entries.
