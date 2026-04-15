---
id: TASK-113
title: Default Init Output To YAML
status: done
category: init
related_features:
  - SPEC-004
owner: @aattard
created: 2026-04-15
updated: 2026-04-15
---

## Summary

Change `sw init` to generate YAML by default so the starter runbook matches the
project's preferred authoring format.

## Scope

- Change the default init output path from `./sw-runbook.json` to
  `./sw-runbook.yaml`
- Infer JSON or YAML from a recognized `--output-file` extension
- Reject unsupported output-file extensions with a clear error
- Update help, explain, and automated coverage

## Assumptions

- YAML is now the preferred default authoring format for runbooks in this
  project.
- A custom output path must still determine the actual serialized format, so
  `starter.json` should not silently contain YAML.
- `init` does not need a separate `--output-format` flag in this increment if
  extension-based inference is clear and deterministic.

## Acceptance Criteria

- [x] Given `sw init`, the command writes `./sw-runbook.yaml`.
- [x] Given `sw init --output-file starter.json`, the command writes valid JSON
      to `starter.json`.
- [x] Given `sw init --output-file starter.yaml`, the command writes valid YAML
      to `starter.yaml`.
- [x] Given `sw init --output-file starter.txt`, the command exits with `1`
      and reports a clear unsupported-format error.
- [x] Help and explain output document the new YAML default.

## Notes

This changes the starter-runbook default only. It does not change `run`,
`check`, or `validate` input behavior.
