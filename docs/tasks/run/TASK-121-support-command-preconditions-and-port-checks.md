---
id: TASK-121
title: Support Command Preconditions And Port Checks
status: proposed
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-22
updated: 2026-04-22
---

## Summary

Add command-level preconditions plus port-based checks so runbooks can verify
that required TCP ports are free before a command starts and can assert port
state immediately after the command body completes.

## Scope

- Add `Command.preconditions.checks`
- Support `source: port` checks in `preconditions.checks`
- Support `source: port` checks in `assert.checks`
- Validate single-port `port` values and `free: true`
- Report failing preconditions and failing assertions against the owning
  `Command` entry
- Cover lifecycle timing, including that assertions run before deferred cleanup

## Assumptions

- Port checks target TCP listener availability on the local machine.
- One check targets one port; multiple ports are expressed as multiple checks.
- `preconditions` remains a check container only and does not gain an
  `exit_code` field.
- Same-entry `assert.checks` do not verify post-cleanup port state.

## Acceptance Criteria

- [ ] Given `preconditions.checks` with `source: port`, `port: 8080`, and
      `free: true`, the command body executes only when TCP port `8080` is not
      listening locally.
- [ ] Given a failing port precondition, the run exits with `2`, does not
      execute the command body, and reports the failing `Command` entry.
- [ ] Given `assert.checks` with `source: port`, `port: 8080`, and
      `free: true`, the command is considered successful only when TCP port
      `8080` is not listening locally after the command body completes.
- [ ] Given a failing port assertion, the run exits with `2`, does not write a
      partial output file, and reports the failing `Command` entry together
      with captured stdout and stderr.
- [ ] Given a command with deferred cleanup that releases a port, a same-entry
      port assertion still evaluates before cleanup and does not treat
      post-cleanup port state as part of that assertion.

## Notes

This task intentionally stops short of a post-cleanup assertion phase. If
users need to verify teardown state after cleanup, that should be designed as
an explicit later increment instead of being implied by same-entry assertions.
