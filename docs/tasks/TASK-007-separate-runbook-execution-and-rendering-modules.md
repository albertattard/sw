---
id: TASK-007
title: Separate Runbook Execution and Rendering Modules
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Refactor runbook execution and rendering into dedicated modules.

## Scope

- Keep `src/commands/run.rs` as the CLI command entrypoint
- Keep runbook validation in `src/runbook/validate.rs`
- Move runbook rendering logic out of `src/runbook/mod.rs`
- Move command execution, timeout, and assertion logic out of `src/runbook/mod.rs`
- Preserve current behavior and test coverage

## Assumptions

- This task is a structural refactor only and does not change the user-visible contract.
- Existing tests should continue to prove behavior after the refactor.

## Acceptance Criteria

- [x] `src/runbook/mod.rs` is reduced to module wiring and shared types/helpers.
- [x] Rendering logic lives in a dedicated runbook module.
- [x] Command execution and assertion logic lives in a dedicated runbook module.
- [x] Existing CLI behavior remains unchanged.
- [x] Existing automated tests continue to pass.

## Notes

This task improves consistency with the validate command structure and keeps the
runbook module layout scalable as more execution features are added.
