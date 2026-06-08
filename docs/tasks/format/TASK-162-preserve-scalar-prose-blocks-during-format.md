---
id: TASK-162
title: Preserve Scalar Prose Blocks During Format
status: done
category: format
related_features:
  - SPEC-010
owner: albertattard
created: 2026-06-08
updated: 2026-06-08
---

## Summary

Make `sw format` keep scalar-capable prose fields readable in YAML by rendering
them as literal block scalars instead of quoted strings with escaped newlines.

## Scope

- Normalize scalar-capable YAML fields during `sw format` before serialization
- Render multiline YAML strings as literal block scalars in the canonical YAML
  serializer
- Preserve semantic string content, including terminal newlines and trailing
  spaces inside prose lines
- Cover `Prerequisite.checks[*].contents` with integration-style format tests

## Assumptions

- This is a presentation-only formatting change; it must not change runbook
  validation or execution behavior.
- Scalar-capable field normalization should match the existing JSON-to-YAML
  conversion contract.

## Acceptance Criteria

- [x] Given a YAML `Prerequisite.checks[*].contents` literal block scalar,
      `sw format` keeps it as a literal block scalar.
- [x] Given scalar-capable line arrays in YAML, `sw format` may normalize them
      to literal block scalars just as JSON-to-YAML conversion already does.
- [x] Given multiline prose containing trailing spaces before a newline,
      `sw format` does not fall back to an escaped quoted string.
- [x] Automated format tests pass after the change.
