---
id: TASK-043
title: Implement Explain Command
status: pending
category: explain
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
- Make the output agent-first so it helps choose between `help`, `example`,
  and `explain`
- Update CLI help output and help-focused tests
- Add integration coverage for successful topic explanations, `--all`, unknown
  topics, and missing-argument behavior
- Add scenario-oriented coverage for agent questions such as "how do I check
  for Java 21?"

## Assumptions

- `explain` complements `help` and `example`; it does not replace either one.
- The first increment is command/topic oriented rather than entry-type or
  nested-feature oriented.
- If a more structured format is better for reliable agent use than more
  natural prose, prefer the structured format.

## Acceptance Criteria

- [ ] Given `sw explain run`, the CLI exits with `0` and prints a concise
      summary of the run contract.
- [ ] Given `sw explain --all`, the CLI exits with `0` and prints all
      supported topic explanations.
- [ ] Given an agent-style question such as "how do I check for Java 21?",
      the `explain` output gives enough guidance to choose the correct next
      `sw` command and topic.
- [ ] Given an agent deciding between `help`, `example`, and `explain`, the
      `explain` output makes those boundaries clear.
- [ ] Given `sw explain RUN`, the CLI behaves the same as `sw explain run`.
- [ ] Given `sw explain unknown`, the CLI exits with `1`.
- [ ] Given `sw explain` with no topic and no `--all`, the CLI exits with `1`.
- [ ] Top-level help includes `explain`.
- [ ] `sw explain --help` documents the topic argument and `--all`.

## Notes

This command is intended to expose repository-backed product knowledge through
the CLI in a form that is useful to both humans and other models.
