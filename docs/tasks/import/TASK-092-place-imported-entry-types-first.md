---
id: TASK-092
title: Place Imported Entry Types First
status: completed
category: import
related_features:
  - SPEC-006
owner: @aattard
created: 2026-04-07
updated: 2026-04-08
---

## Summary

Make `sw import` serialize each generated runbook entry with `type` as the
first field so the output is easier for humans to scan and edit, including for
future imported entry types added later.

## Scope

- Update import output so every generated runbook entry serializes `type`
  before its other fields
- Preserve the existing imported entry content and entry ordering
- Add coverage for serialized field order in imported runbook output
- Keep the change limited to `sw import` output rather than changing field
  ordering across the CLI

## Assumptions

- This change is about imported runbook readability, not runbook semantics.
- All imported entry objects should follow this field-ordering contract,
  including future entry types that `sw import` may emit.
- The top-level runbook structure and non-import machine-readable output should
  remain unchanged unless separately specified.

## Acceptance Criteria

- [x] Given an imported runbook entry of any type that `sw import` emits,
      serialized output places `type` before the other entry fields.
- [x] Given the current imported `Heading`, `Markdown`, and `Command` entries,
      serialized output places `type` before their other entry-specific
      fields.
- [x] The generated runbook still passes `sw validate`.
- [x] Automated tests pass after the change.

## Notes

This is a readability contract for starter runbooks produced by `sw import`.
It should not quietly change JSON object ordering for unrelated commands.
