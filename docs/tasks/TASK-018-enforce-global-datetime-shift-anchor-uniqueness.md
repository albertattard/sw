---
id: TASK-018
title: Enforce Global Datetime Shift Anchor Uniqueness
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Reject duplicate `datetime_shift.id` values across the runbook and ensure that
`use` only references anchors established earlier in the same command output
block.

## Scope

- Reject duplicate `datetime_shift.id` values anywhere in one runbook
- Reject `use` when the referenced anchor was not established earlier in the
  same `output.rewrite` array
- Keep anchor reuse local to one command output block

## Assumptions

- Global uniqueness applies across all entries in the runbook, not just within
  one command.
- `use` remains local to the output block even though `id` uniqueness is
  global.

## Acceptance Criteria

- [x] Given duplicate `datetime_shift.id` values in different commands,
      validation rejects the runbook.
- [x] Given duplicate `datetime_shift.id` values in the same output block,
      validation rejects the runbook.
- [x] Given a `use` rule that references an anchor established earlier in the
      same output block, validation accepts it.
- [x] Given a `use` rule that references an anchor from a different output
      block, validation rejects the runbook.
- [x] Given a `use` rule that appears before its anchor in the same output
      block, validation rejects the runbook.

## Notes

This task makes datetime anchor identifiers stable and unambiguous across the
whole runbook while keeping actual anchor reuse scoped to the output block
where the timeline was established.
