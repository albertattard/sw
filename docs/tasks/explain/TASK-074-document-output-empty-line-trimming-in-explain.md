---
id: TASK-074
title: Document Output Empty Line Trimming In Explain
status: done
category: explain
related_features:
  - SPEC-009
owner: @aattard
created: 2026-03-22
updated: 2026-03-22
---

## Summary

Extend `sw explain` so users and agents can discover the `trim_empty_lines`
output contract from the CLI instead of needing repository spec access.

## Scope

- Add `trim_empty_lines` guidance to `sw explain run`
- Clarify the supported values and intent of `trim_empty_lines`
- Update `sw explain example` so it notes that the `Command` example includes
  current output fields such as `trim_empty_lines`
- Ensure skill export inherits the new explain guidance
- Add explain-focused CLI coverage for the new discovery content

## Assumptions

- `explain` is the contract surface for runbook-authored behavior
- Skill output should remain a derived view of the explain knowledge model
- Users should be able to discover output cleanup behavior without reading the
  raw specs

## Acceptance Criteria

- [x] Given `sw explain run`, the CLI documents `output.trim_empty_lines` and
      its supported values.
- [x] Given `sw explain example`, the CLI notes that the `Command` example
      includes current nested output fields such as `trim_empty_lines`.
- [x] Given `sw explain --output-format=skill`, the generated skill content
      preserves the new empty-line trimming guidance.
- [x] Explain-focused automated tests cover the updated guidance.

## Notes

This increment improves CLI discoverability for a schema feature that is
already implemented in validation and rendering.
