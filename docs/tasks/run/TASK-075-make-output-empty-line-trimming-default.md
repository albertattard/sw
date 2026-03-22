---
id: TASK-075
title: Make Output Empty Line Trimming Default
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-22
updated: 2026-03-22
---

## Summary

Change the default `Command.output` behavior so leading and trailing empty
lines are trimmed unless a runbook explicitly overrides that behavior.

## Scope

- Change the default `trim_empty_lines` mode from `none` to
  `leading_trailing`
- Keep the explicit `leading_trailing`, `leading`, `trailing`, and `none`
  values unchanged
- Update the run contract and explain guidance to reflect the new default
- Add automated coverage for the omitted-field default behavior

## Assumptions

- Trimming outer empty lines is the more useful default for documentation
  output
- Authors can still preserve outer empty lines explicitly with
  `trim_empty_lines: none`
- This change does not alter how empty lines inside the retained output are
  handled

## Acceptance Criteria

- [x] Given a `Command` entry with `output` and no `trim_empty_lines`, leading
      and trailing empty lines are removed from rendered output.
- [x] Given `output.trim_empty_lines: none`, leading and trailing empty lines
      are still preserved.
- [x] The documented run and explain contracts reflect the new default.

## Notes

This keeps the common documentation-cleanup behavior on by default while
preserving an explicit escape hatch for exact output rendering.
