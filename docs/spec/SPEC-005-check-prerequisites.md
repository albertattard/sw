---
id: SPEC-005
title: Check Runbook Prerequisites
status: proposed
priority: medium
owner: @aattard
last_updated: 2026-03-13
---

## Problem

Before running a full runbook, users and agents need a fast way to verify that
the current environment satisfies the runbook prerequisites without generating
README output or executing the main workflow.

## User-facing Behavior

The CLI provides a prerequisite-check command:

```bash
sw check --input-file <sw-runbook.json>
```

If no input file is provided, the command uses `./sw-runbook.json` by default:

```bash
sw check
```

The command validates the runbook input, then executes only `Prerequisite`
entries and reports whether the current environment is ready for `sw run`.

## Inputs

- Optional named input file parameter: `--input-file <runbook.json>`.

Default input behavior:
- If `--input-file` is provided, use that path.
- If no file path is provided, use `./sw-runbook.json`.

## Outputs

- Human-readable console output describing whether all prerequisite checks
  passed or which prerequisite failed.
- No Markdown output file is written.

### Exit Codes

- `0`: the runbook is valid and all prerequisite checks passed.
- `2`: the runbook is valid but at least one prerequisite check failed.
- `1`: operational error (missing file, unreadable file, invalid runbook,
  internal error).

## Execution Rules

- `sw check` validates the runbook before executing any prerequisite checks.
- If the runbook is invalid, `sw check` stops and returns exit code `1`.
- `sw check` executes only `Prerequisite` entries.
- `sw check` does not execute `Command` entries.
- `sw check` does not render or write `README.md`.
- Prerequisite checks execute in the same order they appear in the runbook.
- If a prerequisite check fails, `sw check` stops on that failure and reports
  the failing check.
- If a failing prerequisite check includes `help`, that remediation message is
  surfaced in the command output.

## Acceptance Criteria

- [ ] Given a valid runbook whose prerequisite checks all pass, `sw check`
      exits with `0`.
- [ ] Given no `--input-file` and a valid `./sw-runbook.json`, `sw check`
      checks that file.
- [ ] Given a missing input file, `sw check` exits with `1` and reports a
      clear file error.
- [ ] Given an invalid runbook, `sw check` exits with `1` and reports that the
      runbook is invalid.
- [ ] Given a failing prerequisite check, `sw check` exits with `2` before any
      normal `Command` entry executes.
- [ ] Given a failing prerequisite check with `help`, the failure output
      includes that remediation message.
- [ ] Given a passing prerequisite check followed by a `Command` entry,
      `sw check` does not execute that `Command` entry.
- [ ] `sw check` performs no README rendering or file writes.

## Non-goals

- Executing normal runbook `Command` entries.
- Rendering Markdown output.
- Replacing `sw validate`.
- Providing machine-readable output in this increment.

## Edge Cases

- Runbook contains no `Prerequisite` entries.
- First prerequisite passes and a later prerequisite fails.
- A prerequisite check times out.
- A prerequisite check fails an assertion.
- A runbook is structurally invalid before any checks run.

## Notes for Reimplementation

This command is distinct from `sw validate`:
- `validate` checks runbook structure.
- `check` verifies execution readiness by running prerequisite checks only.
