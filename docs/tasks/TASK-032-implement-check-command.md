---
id: TASK-032
title: Implement Check Command
status: done
related_features:
  - SPEC-005
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Add `sw check` so users can verify prerequisite readiness without running the
main workflow or generating Markdown output.

## Scope

- Add a `check` CLI subcommand
- Support `--input-file` with default `./sw-runbook.json`
- Validate the runbook before executing prerequisite checks
- Execute only `Prerequisite` entries
- Stop on the first failing prerequisite
- Return the exit codes defined by `SPEC-005`

## Assumptions

- This increment uses human-readable console output only.
- `check` reuses the existing prerequisite execution behavior where possible.
- Invalid runbooks are treated as operational failures for `check`.

## Acceptance Criteria

- [x] Given a valid runbook whose prerequisite checks all pass, `sw check`
      exits with `0`.
- [x] Given a failing prerequisite check, `sw check` exits with `2` before any
      normal command executes.
- [x] Given an invalid runbook, `sw check` exits with `1`.
- [x] Given a runbook with no `Prerequisite` entries, `sw check` exits with
      `0`.
- [x] `sw check` does not write a README output file.

## Notes

This provides a fast preflight command that complements `sw validate` and
`sw run` by checking runtime readiness only.
