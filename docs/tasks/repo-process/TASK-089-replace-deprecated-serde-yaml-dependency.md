---
id: TASK-089
title: Replace Deprecated Serde YAML Dependency
status: pending
category: repo-process
related_features:
  - SPEC-002
  - SPEC-003
  - SPEC-005
owner: @aattard
created: 2026-04-02
updated: 2026-04-02
---

## Summary

Replace the deprecated `serde_yaml` dependency with a maintained YAML parsing
approach while preserving the current runbook input behavior for users and
agents.

## Scope

- Evaluate and select a maintained replacement for YAML parsing and
  deserialization
- Update the implementation to use the replacement dependency
- Preserve current YAML input support for `validate`, `run`, and `check`
- Keep existing JSON input behavior unchanged
- Update dependency hygiene documentation if the replacement changes local or
  CI verification expectations
- Add or update automated tests only where replacement behavior requires it

## Assumptions

- This task is intended to remove deprecated dependency risk, not to redesign
  the runbook input contract.
- YAML parsing behavior should remain aligned with the current documented
  command contracts unless a spec is intentionally updated.
- Some edge-case parsing differences may need explicit regression tests once a
  replacement crate is chosen.

## Acceptance Criteria

- [ ] The repository no longer depends on the deprecated `serde_yaml` crate.
- [ ] `sw validate` continues to accept supported YAML input from files and
      stdin.
- [ ] `sw run` continues to accept supported YAML input from files and stdin.
- [ ] `sw check` continues to accept supported YAML input from files and stdin.
- [ ] Existing automated tests continue to pass after the dependency
      replacement.
- [ ] Dependency hygiene checks remain green after the change.

## Notes

This task is a dependency-maintenance slice with user-visible compatibility
constraints. The goal is to remove a deprecated dependency without expanding or
loosening the runbook format contract.
