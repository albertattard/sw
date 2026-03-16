---
id: TASK-061
title: Add Debug Run Diagnostics
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-15
updated: 2026-03-15
---

## Summary

Add a `--debug` execution mode that prints richer troubleshooting details for
runbook commands, rewrites, and captures without changing the stable stdout
contract.

## Scope

- Add `--debug` to `sw run`
- Support `sw --debug` as the default no-subcommand form
- Emit debug diagnostics to stderr
- Keep debug output additive so generated Markdown and normal stdout behavior
  stay unchanged
- Include enough rewrite and capture detail to diagnose interpolation and
  matching failures
- Add CLI coverage for the new debug mode
- Update help output and help-focused tests for the new flag

## Assumptions

- `--debug` is for troubleshooting and is not a stable machine-readable
  contract.
- `--debug` may be used together with `--verbose`.

## Acceptance Criteria

- [x] Given `sw run --debug`, debug diagnostics are written to stderr without
      changing stdout or generated Markdown output.
- [x] Given `sw --debug` with no subcommand, the command behaves the same as
      `sw run --debug`.
- [x] Given a `Command` entry with rewrites and captures, debug output includes
      enough interpolated rewrite and capture information to diagnose matching
      failures.
- [x] Help output documents the `--debug` flag.

## Notes

This should complement `--verbose`: verbose explains progress, while debug
explains execution details.
