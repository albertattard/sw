---
id: TASK-051
title: Default To Automatic Command Process Cleanup
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Add default automatic cleanup for processes started by a `Command` entry,
unless the entry provides an explicit manual cleanup block.

## Scope

- Automatically terminate remaining processes started by a command entry when
  no explicit manual `cleanup` block is provided
- Use manual `cleanup` as the explicit override for command teardown
- Validate the new cleanup contract and cover it with CLI tests

## Assumptions

- Automatic cleanup targets only processes started by the current command
  entry.
- Manual `cleanup` replaces automatic process cleanup for that command entry.

## Acceptance Criteria

- [x] Given a command without `cleanup`, remaining processes started by that
      command are terminated automatically after the entry finishes.
- [x] Given a command without `cleanup`, remaining processes started by that
      command are terminated automatically after failure or timeout.
- [x] Given a command with `cleanup`, the explicit cleanup block is used
      instead of automatic process cleanup for that entry.
- [x] Existing cleanup ordering and cleanup-failure behavior remain unchanged
      for commands that provide manual `cleanup`.

## Notes

This keeps the common case simple for background services while still allowing
manual teardown when a command needs something more controlled than the default
automatic process cleanup behavior.
