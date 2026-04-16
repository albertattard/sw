---
id: TASK-117
title: Standardize Indented YAML Sequences
status: done
category: format
related_features:
  - SPEC-004
  - SPEC-006
  - SPEC-010
owner: @aattard
created: 2026-04-16
updated: 2026-04-16
---

## Summary

Standardize the repository's YAML authoring style on indented sequence markers
under mapping keys, using `entries:\n  - ...` instead of `entries:\n- ...`.

## Scope

- Update the canonical YAML formatting contract used by `sw format`
- Align YAML output from `sw init` with the same indentation style
- Align YAML output from `sw import` with the same indentation style while
  preserving type-first field order and block-scalar prose output
- Preserve the existing blank line between adjacent top-level `entries`
- Keep JSON output unchanged
- Add automated CLI coverage for the new YAML house style

## Assumptions

- This is a presentation and authoring ergonomics change, not a schema change.
- The new house style should apply to YAML sequences nested under mapping keys,
  not only to the top-level `entries` list.
- `sw example` already follows this convention for nested YAML sequences in its
  single-entry snippets, so it does not need a parallel behavior change in this
  increment.

## Acceptance Criteria

- [x] Given YAML output from `sw format`, the top-level `entries` list renders
      as `entries:\n  - ...` and still separates adjacent entries with a single
      blank line.
- [x] Given YAML output from `sw format`, nested sequences under mapping keys
      are indented by two spaces beneath the owning key.
- [x] Given YAML output from `sw init`, the generated starter runbook uses the
      same indented-sequence style.
- [x] Given YAML output from `sw import`, the generated runbook uses the same
      indented-sequence style while preserving the existing type-first and
      block-scalar contracts.
- [x] JSON output behavior remains unchanged.

## Notes

This adopts the more conventional human-authored YAML style across the
repository instead of treating indented sequence markers as a command-specific
exception.
