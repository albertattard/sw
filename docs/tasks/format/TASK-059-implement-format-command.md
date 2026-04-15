---
id: TASK-059
title: Implement Format Command
status: done
category: format
related_features:
  - SPEC-010
owner: @aattard
created: 2026-03-15
updated: 2026-04-15
---

## Summary

Add a `format` command that rewrites valid runbook JSON and YAML files into the
canonical repository formatting style for their existing format.

## Scope

- Add a `format` CLI command
- Support `--input-file <runbook.{json|yaml|yml}>`
- Reuse shared default file discovery across `sw-runbook.json`,
  `sw-runbook.yaml`, and `sw-runbook.yml`
- Parse and validate the runbook before rewriting it
- Rewrite valid JSON and YAML input files in place using deterministic
  formatting for their current format
- Keep invalid JSON, invalid YAML, and invalid runbooks unchanged
- Add CLI coverage for success, invalid JSON, invalid runbook structure, and
  default input-file behavior, including ambiguous default-file selection
- Update help output and help-focused tests for the new command

## Assumptions

- Formatting is in-place in this increment; no separate output-file option is
  introduced.
- The command preserves the existing file format and is not a JSON/YAML
  conversion feature in this increment.
- Canonical JSON formatting uses the same two-space pretty-printed style shown
  in the repository’s example runbooks.
- Canonical YAML formatting should be deterministic and editing-friendly.
- Property order must be preserved.

## Acceptance Criteria

- [x] Given a valid runbook file,
      `sw format --input-file <file>` rewrites the file in place in canonical
      format for that file’s existing JSON or YAML syntax and exits with `0`.
- [x] Given no `--input-file` argument and a valid `./sw-runbook.yaml`, with
      no other default runbook file present, `sw format` rewrites that file in
      place and exits with `0`.
- [x] Given no `--input-file` argument, no `./sw-runbook.yaml`, and a valid
      `./sw-runbook.json`, with no other default runbook file present,
      `sw format` rewrites that file in place and exits with `0`.
- [x] Given no `--input-file` argument and more than one of
      `./sw-runbook.json`, `./sw-runbook.yaml`, or `./sw-runbook.yml` present,
      `sw format` exits with `1` and requires `--input-file`.
- [x] Given invalid JSON syntax,
      `sw format --input-file <file>` exits with `1` and does not modify the
      file.
- [x] Given invalid YAML syntax,
      `sw format --input-file <file>` exits with `1` and does not modify the
      file.
- [x] Given valid JSON or YAML that fails runbook validation,
      `sw format --input-file <file>` exits with `2` and does not modify the
      file.
- [x] Help output documents the `format` command and its options.

## Notes

This command should reuse the existing runbook validation path so formatting
never accepts input that `sw validate` would reject.
