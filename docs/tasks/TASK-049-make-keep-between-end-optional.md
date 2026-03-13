---
id: TASK-049
title: Make Keep Between End Optional
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Allow `keep_between` rewrite rules to keep output from a matched `start`
boundary to the end of the output without requiring an `end` boundary.

## Scope

- Make `end` optional for `keep_between`
- Keep the existing bounded-slice behavior when both `start` and `end` are
  present
- When `end` is omitted, keep from the adjusted `start` boundary to the end of
  the output
- Ignore `end_offset` when `end` is omitted
- Add validation and integration coverage for the unbounded form

## Assumptions

- `start` remains required for `keep_between`.
- The unbounded form is a special case of the same slicing rule, not a separate
  rewrite rule type.

## Acceptance Criteria

- [x] Given a `keep_between` rewrite rule with `start` and no `end`, output is
      kept from the adjusted `start` boundary to the end.
- [x] Given a `keep_between` rewrite rule with `start_offset` and no `end`,
      `start_offset` still applies.
- [x] Given a `keep_between` rewrite rule with no `end`, `end_offset` is
      ignored.
- [x] Existing bounded `keep_between` behavior remains unchanged.

## Notes

This keeps the rewrite model smaller and more consistent than introducing a new
start-to-end rule for a behavior that is really just an unbounded slice.
