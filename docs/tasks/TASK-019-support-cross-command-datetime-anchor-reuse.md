---
id: TASK-019
title: Support Cross-Command Datetime Anchor Reuse
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Allow `datetime_shift.use` to reference an anchor established earlier in the
runbook, so multiple command outputs can share one synthetic timeline.

## Scope

- Allow `use` to reference an anchor established in an earlier command output
  block
- Keep `datetime_shift.id` globally unique across the runbook
- Reject forward references where `use` appears before the anchor is
  established
- Update validation and rendering so later commands can reuse an earlier
  timeline anchor

## Assumptions

- Anchor lookup is runbook-wide and follows declaration order.
- A reused anchor preserves its original base timestamp and original first
  matched datetime.

## Acceptance Criteria

- [x] Given a later command output block that uses an anchor established
      earlier in the runbook, the runbook validates successfully.
- [x] Given cross-command reuse of one anchor across different supported
      datetime formats, rewritten datetimes follow one shared timeline.
- [x] Given a `use` rule before its anchor is established in the runbook,
      validation rejects the runbook.
- [x] Given duplicate `datetime_shift.id` values anywhere in the runbook,
      validation still rejects the runbook.

## Notes

This task allows a runbook to keep one coherent synthetic timeline across
multiple command outputs, which is especially useful when a walkthrough shows
several related steps that should appear to belong to the same execution
session.
