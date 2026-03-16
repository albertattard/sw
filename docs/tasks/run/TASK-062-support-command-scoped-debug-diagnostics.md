---
id: TASK-062
title: Support Command-Scoped Debug Diagnostics
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-15
updated: 2026-03-15
---

## Summary

Allow individual `Command` entries to opt into debug diagnostics without
requiring global `--debug`, so users can troubleshoot one command at a time.

## Scope

- Add `debug` as an optional boolean property on `Command` entries
- Default command-level `debug` to `false` when omitted
- Emit debug diagnostics for a command when either global `--debug` is enabled
  or that command sets `debug: true`
- Keep global `--debug` behavior unchanged so it still enables diagnostics for
  all commands
- Add CLI coverage for mixed runbooks where only some commands opt into debug
- Update validation as needed for the new `Command.debug` field

## Assumptions

- Command-scoped `debug` is additive and must not change generated Markdown or
  the stable stdout contract.
- Global `--debug` remains the simplest way to diagnose the whole run.

## Acceptance Criteria

- [x] Given a runbook with one `Command` entry with `debug: true` and another
      without it, and no global `--debug`, only the flagged command emits
      debug diagnostics.
- [x] Given a `Command` entry with `debug: false`, that entry does not emit
      debug diagnostics unless global `--debug` is enabled.
- [x] Given global `--debug`, all command entries emit debug diagnostics
      regardless of command-level `debug`.
- [x] Validation accepts `Command.debug` when it is a boolean and rejects it
      when it is any other type.

## Notes

This is a narrower troubleshooting control than global `--debug` and should
help users inspect one problematic command without flooding stderr for the
whole run.
