---
id: TASK-078
title: Support Stdin Runbook Input
status: pending
category: run
related_features:
  - SPEC-001
  - SPEC-002
  - SPEC-003
  - SPEC-005
  - SPEC-009
owner: @aattard
created: 2026-03-22
updated: 2026-03-22
---

## Summary

Allow `sw` to read a runbook from stdin so agents can generate runbook content
on the fly and pipe it directly into `run`, `check`, and `validate` without
creating a temporary file first.

## Scope

- Add shared `--input-file=-` support for `run`, `check`, and `validate`
- Ensure `sw --input-file=-` follows the same contract as
  `sw run --input-file=-`
- Add shared `--input-format=json|yaml` support for stdin-backed runbook input
- Default stdin parsing to JSON when `--input-file=-` is used without
  `--input-format`
- Require `--input-format=yaml` for YAML piped through stdin
- Preserve the current default file lookup and file-based format inference when
  `--input-file=-` is not used, even if `--input-format` is present
- Report a clear operational error when stdin is selected but the piped input
  cannot be parsed
- Update `sw help` for `run`, `check`, and `validate` so the stdin contract is
  discoverable to agents
- Update `sw explain run`, `sw explain check`, `sw explain validate`, and skill
  export so the stdin contract is discoverable without repository access
- Add integration coverage for stdin-backed success and failure paths

## Assumptions

- `--input-file=-` is the preferred stdin convention for this CLI increment
- Stdin support applies to the runbook input source, not to individual runbook
  entry types such as `DisplayFile`
- JSON remains the default stdin format because it is the lowest-friction
  agent-generated representation already supported by `sw`
- `--input-format` exists primarily to disambiguate stdin input and must not
  silently change the current file-backed defaults

## Acceptance Criteria

- [ ] Given `sw run --input-file=-` with a valid JSON runbook on stdin, the
      command renders the runbook and exits with `0`.
- [ ] Given `sw --input-file=-` with a valid JSON runbook on stdin, the
      command behaves the same as `sw run --input-file=-`.
- [ ] Given `sw validate --input-file=- --output-format=json` with a valid JSON
      runbook on stdin, the command validates stdin and exits with `0`.
- [ ] Given `sw check --input-file=-` with a valid JSON runbook on stdin, the
      command applies the existing prerequisite-check contract and exit codes.
- [ ] Given `run`, `check`, or `validate` with
      `--input-file=- --input-format=yaml` and a valid YAML runbook on stdin,
      the command parses stdin as YAML and applies the existing contract for
      that subcommand.
- [ ] Given `run`, `check`, or `validate` with `--input-file=-` and YAML on
      stdin but without `--input-format=yaml`, the command exits with `1` and
      reports a clear parsing error.
- [ ] Given `--input-format=json` or `--input-format=yaml` without
      `--input-file=-`, `run`, `check`, and `validate` keep the current default
      file lookup behavior.
- [ ] Help output for `run`, `check`, and `validate` documents `--input-file=-`
      and `--input-format`.
- [ ] `sw explain run`, `sw explain check`, and `sw explain validate` document
      stdin-backed runbook input, default JSON stdin parsing, and explicit YAML
      stdin selection.
- [ ] `sw explain --output-format=skill` preserves the stdin guidance for
      agents.

## Notes

This increment should reuse the shared runbook-loading path so stdin support
does not fork command behavior across `run`, `check`, and `validate`.
