---
id: TASK-028
title: Implement Init Command
status: pending
related_features:
  - SPEC-004
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Add an `init` command that writes a realistic sample runbook file for users to
 customize and extend.

## Scope

- Add the `init` subcommand to the CLI
- Write a sample `sw-runbook.json` by default
- Support `--output-file <path>`
- Support `--force`
- Ensure the generated sample is valid according to the current runbook
  contract

## Assumptions

- The first increment provides one sample template only.
- The generated sample should demonstrate supported entry types and common
  options without trying to cover every edge case.

## Acceptance Criteria

- [ ] Given `sw init` in a directory without `sw-runbook.json`, the command
      writes `./sw-runbook.json` and exits with `0`.
- [ ] Given `sw init --output-file <path>`, the command writes the sample to
      the provided path.
- [ ] Given `sw init` when the target file already exists, the command exits
      with `1` and does not overwrite the file.
- [ ] Given `sw init --force` when the target file already exists, the command
      overwrites the file and exits with `0`.
- [ ] The generated sample file passes `sw validate`.

## Notes

This gives users a concrete starting point for the product and lowers the cost
of learning the runbook format by example.
