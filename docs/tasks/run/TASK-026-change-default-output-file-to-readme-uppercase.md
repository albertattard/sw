---
id: TASK-026
title: Change Default Output File To README Uppercase
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Change the default generated output filename from `readme.md` to `README.md`.

## Scope

- Update the `run` command default output file to `README.md`
- Update CLI success messaging to reflect the new default filename
- Update integration tests that rely on the default filename

## Assumptions

- Explicit `--output-file` values remain unchanged.
- This task only changes the default filename, not any other rendering
  behavior.

## Acceptance Criteria

- [x] Given `sw run` without `--output-file`, the generated file is
      `./README.md`.
- [x] Given `sw` with no subcommand, the generated file is `./README.md`.
- [x] Given an explicit `--output-file`, that explicit path is still used.
- [x] CLI success output refers to `README.md` when the default path is used.

## Notes

This keeps the generated output aligned with the conventional Markdown
filename most repositories already expect.
