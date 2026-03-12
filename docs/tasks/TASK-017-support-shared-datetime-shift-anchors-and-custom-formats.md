---
id: TASK-017
title: Support Shared Datetime Shift Anchors And Custom Formats
status: pending
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Extend `datetime_shift` so multiple datetime styles in the same output block
can share one anchor and custom date layouts can be shifted semantically.

## Scope

- Support `datetime_shift.id` to establish a shared shift anchor
- Support `datetime_shift.use` to reuse a previously established anchor
- Support `datetime_shift.custom_format` for custom pattern-based datetime
  parsing and rendering
- Validate the allowed property combinations for `id`, `use`, `format`,
  `pattern`, `custom_format`, and `base`

## Assumptions

- Shared anchors apply only within one command output block.
- Rules that use `use` do not establish a new base timestamp.
- Built-in formats and custom formats preserve their original textual form
  after shifting.

## Acceptance Criteria

- [ ] Given a `datetime_shift` rule with `id`, later datetime rules in the same
      output block may reuse that anchor with `use`.
- [ ] Given one shared anchor and multiple datetime rules with different
      supported formats, all rewritten datetimes follow the same shift delta.
- [ ] Given a `datetime_shift` rule with `pattern` and `custom_format`,
      matching custom datetimes are shifted semantically and rendered back in
      that same custom format.
- [ ] Given a rule that uses `use`, specifying `base` is rejected.
- [ ] Given a rule that specifies both `id` and `use`, validation rejects it.
- [ ] Given a rule that specifies both `format` and `custom_format`,
      validation rejects it.

## Notes

This task extends datetime anonymisation so one output block can keep a single
coherent synthetic timeline even when it contains multiple datetime styles such
as log timestamps, HTTP dates, and filename-based datetimes.
