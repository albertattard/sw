---
id: SPEC-005
title: Check Runbook Prerequisites
status: proposed
priority: medium
owner: @aattard
last_updated: 2026-03-18
---

## Problem

Before running a full runbook, users and agents need a fast way to verify that
the current environment satisfies the runbook prerequisites without generating
README output or executing the main workflow.

## User-facing Behavior

The CLI provides a prerequisite-check command:

```bash
sw check --input-file <sw-runbook.yaml>
```

If no input file is provided, the command uses the first existing default
runbook file in this order:

- `./sw-runbook.json`
- `./sw-runbook.yaml`
- `./sw-runbook.yml`

For example:

```bash
sw check
```

The command validates the runbook input, then executes only `Prerequisite`
entries and reports whether the current environment is ready for `sw run`.

## Inputs

- Optional named input file parameter: `--input-file <runbook.{json|yaml|yml}>`.

Default input behavior:
- If `--input-file` is provided, use that path.
- If no file path is provided, use the first existing path from
  `./sw-runbook.json`, `./sw-runbook.yaml`, and `./sw-runbook.yml`.
- Supported input formats are JSON, YAML, and YML.

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
- `sw check` supports built-in prerequisite kinds.
- In this increment, built-in prerequisite kinds include `java`.
- A `java` prerequisite check validates the resolved Java runtime against the
  declared version rule.
- A `java` prerequisite check may target Java from `PATH`, a literal
  `java_home`, or a `java_home_env` environment variable.
- If a failing prerequisite check includes `help`, that remediation message is
  surfaced in the command output.

## Acceptance Criteria

- [ ] Given a valid runbook whose prerequisite checks all pass, `sw check`
      exits with `0`.
- [ ] Given no `--input-file` and a valid `./sw-runbook.json`, `sw check`
      checks that file.
- [ ] Given no `--input-file`, no `./sw-runbook.json`, and a valid
      `./sw-runbook.yaml`, `sw check` checks that file.
- [ ] Given no `--input-file`, no `./sw-runbook.json` or
      `./sw-runbook.yaml`, and a valid `./sw-runbook.yml`, `sw check` checks
      that file.
- [ ] Given a missing input file, `sw check` exits with `1` and reports a
      clear file error.
- [ ] Given `sw check --input-file <file.yaml>` with a valid YAML runbook,
      `sw check` applies the same prerequisite-check contract and exit codes as
      a JSON runbook.
- [ ] Given an invalid runbook, `sw check` exits with `1` and reports that the
      runbook is invalid, including a nearby offending block for
      entry-scoped validation errors.
- [ ] Given a failing prerequisite check, `sw check` exits with `2` before any
      normal `Command` entry executes.
- [ ] Given a `java` prerequisite check with `version: "24+"`, `sw check`
      passes when the resolved Java runtime is Java 24 or higher.
- [ ] Given a `java` prerequisite check with `version: "17"`, `sw check`
      passes only when the resolved Java runtime is exactly Java 17.
- [ ] Given a `java` prerequisite check with `java_home_env`, `sw check`
      resolves Java from that environment variable.
- [ ] Given a `java` prerequisite check with an unset `java_home_env`,
      `sw check` fails with a clear prerequisite error.
- [ ] Given a `java` prerequisite check with both `java_home` and
      `java_home_env`, the runbook is invalid.
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
- A Java prerequisite uses `version: "24+"`.
- A Java prerequisite uses `version: "17"`.
- A Java prerequisite resolves from `JAVA_17_HOME`.
- A Java prerequisite names an unset Java home environment variable.
- A runbook is structurally invalid before any checks run.

## Notes for Reimplementation

This command is distinct from `sw validate`:
- `validate` checks runbook structure.
- `check` verifies execution readiness by running prerequisite checks only.
