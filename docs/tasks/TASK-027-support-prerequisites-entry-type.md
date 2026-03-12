---
id: TASK-027
title: Support Prerequisites Entry Type
status: pending
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Add a `Prerequisites` entry type that both renders prerequisite documentation
and verifies environment requirements before the main workflow runs.

## Scope

- Support a `Prerequisites` entry with a `checks` array
- Render each check’s `contents` into the generated Markdown
- Execute each check’s `commands` before normal runbook commands
- Support `assert` and `help` on prerequisite checks
- Stop the run before the main workflow when a prerequisite check fails

## Assumptions

- Prerequisite assertions reuse the existing command assertion structure.
- A `Prerequisites` entry is distinct from `Command` because it exists to gate
  the main workflow rather than document a normal step inside it.
- This task does not remove the ability to document prerequisites with
  separate `Heading` and `Markdown` entries if users still prefer that style.

## Acceptance Criteria

- [ ] Given a runbook with `Prerequisites` entries, the generated Markdown
      includes the declared prerequisite `contents` in order.
- [ ] Given prerequisite checks, they execute before normal runbook commands.
- [ ] Given a prerequisite check with multiple command lines, those lines
      execute together in the same shell context.
- [ ] Given a failing prerequisite check, the run exits with `2` before
      executing the main workflow.
- [ ] Given a failing prerequisite check with `help`, the failure output
      includes that remediation message.
- [ ] Given passing prerequisite checks, the run continues to the main
      workflow.

## Notes

This provides one source of truth for both prerequisite documentation and
runtime enforcement, reducing duplication between explanatory Markdown and
manual environment checks.
