---
id: TASK-097
title: Add YAML Import Output Format
status: completed
category: import
related_features:
  - SPEC-006
owner: @aattard
created: 2026-04-08
updated: 2026-04-08
---

## Summary

Add YAML output support to `sw import` and make YAML the default import format
so README imports produce a runbook file that is easier to edit by hand.

## Scope

- Add `--output-format json|yaml` to `sw import`
- Default `sw import` output to YAML
- Default the output path to `./sw-runbook.yaml` for YAML output
- Infer output format from `.json`, `.yaml`, and `.yml` output file extensions
  when `--output-format` is omitted
- Reject explicit `--output-format` values that conflict with a recognized
  output file extension
- Update help, explain output, and automated tests

## Assumptions

- This change is scoped to `sw import`; it does not change the default input
  lookup order for `run`, `check`, or `validate`.
- Users should still be able to request JSON explicitly when they want a JSON
  starter runbook.

## Acceptance Criteria

- [x] Given `sw import` with a `./README.md` present, the command writes
      `./sw-runbook.yaml` and exits with `0`.
- [x] Given `sw import --output-format json`, the command writes
      `./sw-runbook.json` and exits with `0`.
- [x] Given `sw import --output-file generated-runbook.json` without
      `--output-format`, the command writes JSON to that path.
- [x] Given `sw import --output-file generated-runbook.yaml` without
      `--output-format`, the command writes YAML to that path.
- [x] Given `sw import --output-format json --output-file generated-runbook.yaml`,
      the command exits with `1` and reports the mismatch.
- [x] The generated YAML and JSON runbooks both pass `sw validate`.

## Notes

This keeps `sw import` aligned with teams that prefer YAML for day-to-day
authoring without removing JSON as an explicit option.
