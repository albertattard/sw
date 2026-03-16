---
id: TASK-029
title: Support Keep Between Rewrite Rule
status: done
category: rewrite
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Add a `keep_between` rewrite rule so runbooks can keep only a selected slice
of command output based on line boundaries instead of relying on large regex
replacements.

## Scope

- Support `type: "keep_between"` under `output.rewrite`
- Support literal `start` and `end` boundaries
- Support line-based `start_offset` and `end_offset`
- Default to `start_offset: 1` and `end_offset: -1`
- Leave output unchanged when either boundary is not found

## Assumptions

- This first increment uses literal string matching, not regex boundaries.
- `keep_between` follows the existing ordered rewrite pipeline.
- Offsets are line-based, not character-based.

## Acceptance Criteria

- [x] Given a `keep_between` rewrite rule, only the lines between the matched
      `start` and `end` boundaries are kept.
- [x] Given a `keep_between` rewrite rule without explicit offsets,
      `start_offset: 1` and `end_offset: -1` are used.
- [x] Given a `keep_between` rewrite rule with explicit offsets, the resulting
      output slice reflects those line-based offsets.
- [x] Given a `keep_between` rewrite rule whose `start` or `end` boundary is
      not found, the rule leaves the output unchanged.

## Notes

This provides a more maintainable way to trim verbose logs down to the relevant
section without forcing users to express line ranges through large regex
replacements.
