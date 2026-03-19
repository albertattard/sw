---
id: TASK-069
title: Support YAML Runbook Input
status: done
category: run
related_features:
  - SPEC-002
  - SPEC-003
  - SPEC-005
  - SPEC-009
owner: @aattard
created: 2026-03-18
updated: 2026-03-18
---

## Summary

Allow `sw` to read runbooks from YAML files in addition to JSON so users can
author executable documentation in either format without changing command
behavior.

## Scope

- Add shared runbook parsing support for JSON, YAML, and YML files
- Keep the in-memory runbook model and validation pipeline unchanged
- Support YAML input for `sw validate`, `sw run`, and `sw check`
- When no `--input-file` is provided, look for `sw-runbook.json`,
  `sw-runbook.yaml`, then `sw-runbook.yml` in that order
- Preserve the existing default preference for JSON when multiple default file
  names are present
- Report clear syntax errors for invalid YAML input
- Update `sw explain` content so it documents YAML support and default input
  lookup order
- Add integration coverage for YAML success cases and default-file fallback

## Assumptions

- YAML support is an input-format addition, not a schema change
- JSON remains the first default file name for compatibility with existing
  workflows
- Commands that only emit JSON snippets or JSON output formats keep those
  output contracts unchanged

## Acceptance Criteria

- [x] Given `sw validate --input-file <file.yaml>` with a valid YAML runbook,
      the CLI validates the file and exits with `0`.
- [x] Given `sw run --input-file <file.yaml>` with a valid YAML runbook, the
      CLI renders the runbook and exits with `0`.
- [x] Given `sw check --input-file <file.yaml>` with a valid YAML runbook, the
      CLI executes prerequisite checks and exits according to the existing
      contract.
- [x] Given no `--input-file`, no `./sw-runbook.json`, and a valid
      `./sw-runbook.yaml`, `validate`, `run`, and `check` use that file.
- [x] Given no `--input-file`, no `./sw-runbook.json` or `./sw-runbook.yaml`,
      and a valid `./sw-runbook.yml`, `validate`, `run`, and `check` use that
      file.
- [x] Given an invalid YAML runbook, the CLI exits with `1` and reports a
      clear YAML parsing error.
- [x] `sw explain validate`, `sw explain run`, and `sw explain check` document
      YAML input support and the default lookup order.

## Notes

This increment should keep YAML support inside the shared runbook loading path
so future commands can inherit the same behavior instead of adding parallel
parsers.
