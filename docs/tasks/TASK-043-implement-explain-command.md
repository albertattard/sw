---
id: TASK-043
title: Implement Explain Command
status: pending
related_features:
  - SPEC-009
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Add an `explain` subcommand that exposes concise feature-contract summaries
through the CLI so users and agents can understand how `sw` works without
reading the repository specs directly.

## Scope

- Add `sw explain <topic>`
- Add `sw explain --all`
- Support case-insensitive topic matching
- Cover the initial topics `help`, `validate`, `run`, `check`, `init`,
  `import`, and `example`
- Return stable, concise text summaries instead of raw spec-file dumps
- Update CLI help output and help-focused tests
- Add integration coverage for successful topic explanations, `--all`, unknown
  topics, and missing-argument behavior

## Assumptions

- `explain` complements `help` and `example`; it does not replace either one.
- The first increment is command/topic oriented rather than entry-type or
  nested-feature oriented.

## Acceptance Criteria

- [ ] Given `sw explain run`, the CLI exits with `0` and prints a concise
      summary of the run contract.
- [ ] Given `sw explain --all`, the CLI exits with `0` and prints all
      supported topic explanations.
- [ ] Given `sw explain RUN`, the CLI behaves the same as `sw explain run`.
- [ ] Given `sw explain unknown`, the CLI exits with `1`.
- [ ] Given `sw explain` with no topic and no `--all`, the CLI exits with `1`.
- [ ] Top-level help includes `explain`.
- [ ] `sw explain --help` documents the topic argument and `--all`.

## Notes

This command is intended to expose repository-backed product knowledge through
the CLI in a form that is useful to both humans and other models.
