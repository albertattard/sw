---
id: TASK-001
title: Implement SPEC-001 Help Placeholder
status: in_progress
related_features:
  - SPEC-001
owner: @aattard
created: 2026-03-05
updated: 2026-03-05
---

## Summary

Implement minimal help/discovery behavior for `sw`.

## Scope

- `sw --help`
- `sw help`
- Generic pattern guidance: `sw [command] --help`
- Friendly placeholder messaging indicating work in progress

## Assumptions

- Help output is human-readable in this first increment.
- Command-specific help details can be expanded in later features.

## Acceptance Criteria

- [x] `sw --help` prints top-level usage and exits with `0`.
- [x] `sw help` prints top-level usage and exits with `0`.
- [x] Output includes a short in-progress placeholder line.

## Notes

Implemented in `src/main.rs`. Can be marked `done` once the task board is
reviewed and confirmed.
