---
id: TASK-124
title: Apply Command Timeout To Full Entry Lifecycle
status: done
category: run
related_features:
  - SPEC-003
  - SPEC-002
owner: @aattard
created: 2026-04-23
updated: 2026-04-23
---

## Summary

Make `sw run` enforce `Command.timeout` until the full command entry has
actually finished, not just until the shell process exits.

## Scope

- Keep the timeout active until the command shell has exited and captured
  stdout and stderr streams have both closed
- Treat background processes that inherit the command pipes as part of the
  timed command lifecycle for timeout purposes
- Preserve partial output when timeout occurs during post-exit pipe draining
- Keep manual `cleanup` as the normal teardown mechanism, but do not let it
  suppress timeout-driven termination of a stuck command lifecycle
- Update validation and explain wording so the documented contract matches the
  runtime behavior
- Add regression coverage for a background process plus explicit cleanup that
  would previously hang past the declared timeout

## Assumptions

- Users who want a background process to survive into later entries should
  redirect its stdout and stderr away from the captured command pipes.
- A command entry should not be considered complete while inherited command
  pipes are still open, even if the shell process itself has already exited.
- Timeout semantics should stay predictable for both humans and agents and
  should not depend on shell-versus-descendant timing races.

## Acceptance Criteria

- [x] Given a command whose shell exits quickly but whose inherited stdout or
      stderr pipe remains open, `sw run` still treats that command entry as
      active.
- [x] Given that active command entry exceeds its declared timeout, `sw run`
      exits with `2` instead of hanging indefinitely.
- [x] Given that timeout path, any output captured before termination is still
      preserved in the partial Markdown output.
- [x] Given the timeout path above, explicit `cleanup` does not suppress
      timeout-driven termination of the remaining command processes.
- [x] `sw explain run` and validation warning text describe the corrected
      lifecycle semantics without implying that timeout itself becomes
      misleading.

## Notes

This is a bounded correctness fix for command execution lifecycle handling. It
should not change the normal contract for deliberately long-lived background
services that redirect their output away and rely on explicit cleanup later in
the runbook.
