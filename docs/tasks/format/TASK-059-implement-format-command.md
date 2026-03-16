---
id: TASK-059
title: Implement Format Command
status: open
category: format
related_features:
  - SPEC-010
owner: @aattard
created: 2026-03-15
updated: 2026-03-15
---

## Summary

Add a `format` command that rewrites valid runbook JSON files into the
canonical repository formatting style.

## Scope

- Add a `format` CLI command
- Support `--input-file <runbook.json>` with `./sw-runbook.json` as the default
- Parse and validate the runbook before rewriting it
- Rewrite valid input files in place using deterministic pretty-printed JSON
- Keep invalid JSON and invalid runbooks unchanged
- Add CLI coverage for success, invalid JSON, invalid runbook structure, and
  default input-file behavior
- Update help output and help-focused tests for the new command

## Assumptions

- Formatting is in-place in this increment; no separate output-file option is
  introduced.
- Canonical formatting uses the same two-space pretty-printed JSON style shown
  in the repository’s example runbooks.
- Property order must be preserved.

## Acceptance Criteria

- [ ] Given a valid runbook file,
      `sw format --input-file <file>` rewrites the file in place in canonical
      JSON format and exits with `0`.
- [ ] Given no `--input-file` argument and a valid `./sw-runbook.json`,
      `sw format` rewrites that file in place and exits with `0`.
- [ ] Given invalid JSON syntax,
      `sw format --input-file <file>` exits with `1` and does not modify the
      file.
- [ ] Given valid JSON that fails runbook validation,
      `sw format --input-file <file>` exits with `2` and does not modify the
      file.
- [ ] Help output documents the `format` command and its options.

## Notes

This command should reuse the existing runbook validation path so formatting
never accepts input that `sw validate` would reject.
