---
id: TASK-211
title: Show Command Environment Bindings In Examples
status: done
category: example
related_features:
  - SPEC-008
  - SPEC-013
owner: albertattard
created: 2026-11-25
updated: 2026-11-25
---

## Summary

Make command-scoped secret environment bindings discoverable in the Command
example, in addition to the existing complete Secrets runbook example.

## Scope

- Add an environment binding referencing a named generated secret to the YAML
  and JSON Command examples, preserving their equivalent structure and the
  single-entry example contract.
- Update the example guide and help text to explain that the referenced secret
  needs a runbook-level declaration and point to `sw example Secrets` for a
  complete runnable example.
- Update integration and help-focused tests for both output formats and the
  discovery guidance.

## Acceptance Criteria

- [x] `sw example Command` shows a valid secret environment binding in YAML.
- [x] JSON output shows the equivalent binding and the formats remain equivalent.
- [x] Example help and the guide direct users to the complete Secrets example
      and explain that the referenced secret needs a runbook-level declaration.
- [x] Automated CLI and help tests cover the updated behavior.
