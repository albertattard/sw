---
id: TASK-008
title: Support Command Cleanup
status: pending
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Support per-command cleanup in `sw run`.

## Scope

- Allow a `Command` entry to declare `cleanup`
- Support `cleanup` as a list of command lines
- Register cleanup commands during normal execution
- Execute cleanup commands in reverse order
- Run cleanup both after successful completion and after early failure
- Continue running cleanup lines and cleanup blocks even when an earlier cleanup step fails
- Keep cleanup optional per command entry

## Assumptions

- `cleanup` is a list of command lines executed in the local shell environment.
- Cleanup executes after the main run path, not interleaved with successful commands.
- Cleanup failures should be reported after all cleanup work has been attempted.

## Acceptance Criteria

- [ ] Given commands with `cleanup`, cleanup runs in reverse order after a successful run.
- [ ] Given a `cleanup` block with multiple command lines, those lines run in order.
- [ ] Given a run failure, previously registered cleanup commands still run in reverse order.
- [ ] Given a run timeout, previously registered cleanup commands still run in reverse order.
- [ ] Given commands without `cleanup`, no cleanup command is registered for those entries.
- [ ] Given a failed cleanup line, remaining cleanup lines and remaining cleanup blocks still run.
- [ ] Given one or more cleanup failures, the overall run is reported as failed.
- [ ] Invalid cleanup values are rejected by validation.

## Notes

This task introduces deterministic resource release for commands that start
background services or other temporary dependencies, while ensuring cleanup is
best effort rather than fail-fast.
