---
id: TASK-056
title: Warn About Background Commands Holding Command Pipes Open
status: done
category: validate
related_features:
  - SPEC-002
owner: @aattard
created: 2026-03-15
updated: 2026-03-15
---

## Summary

Add a validation warning for command entries that appear to start a background
process without redirecting stdout and stderr away from the captured command
pipes.

## Scope

- Detect `Command` entries that appear to launch a background process with `&`
- Warn when that pattern does not also redirect stdout and stderr away from the
  command pipes
- Keep the warning non-blocking so the runbook remains valid
- Include the warning in both human and JSON validation output
- Add validation coverage for the new warning

## Assumptions

- The warning is heuristic and should guide users without pretending to prove
  process intent perfectly.
- Redirecting output to a file is the recommended mitigation for long-lived
  background processes that must survive into later steps.

## Acceptance Criteria

- [x] Given a command entry that starts a background process with `&` and does
      not redirect stdout and stderr away from the command pipes, validation
      returns a warning.
- [x] Given that warning pattern without any validation errors, the runbook
      still validates successfully with exit code `0`.
- [x] Given human validation output, the warning explains that the background
      process may keep the entry open and make timeout or progress behavior
      misleading.
- [x] Given JSON validation output, the warning is included in the `warnings`
      array.

## Notes

This warning helps catch a common runbook authoring mistake without blocking
intentional advanced usage.
