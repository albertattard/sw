---
id: TASK-016
title: Extend Datetime Shift Formats And Default Base
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Extend `datetime_shift` so built-in date formats can be selected explicitly and
the shared default base timestamp is used when `base` is omitted.

## Scope

- Support `datetime_shift.format`
- Treat `format` and `pattern` as mutually exclusive
- Support built-in `rfc3339`
- Support built-in `rfc1123`
- Default `base` to `2077-04-27T12:34:56.789+01:00` when omitted

## Assumptions

- Built-in formats preserve their original textual form after shifting.
- `pattern` remains available for custom matching, but custom semantic parsing
  may require a later increment.

## Acceptance Criteria

- [x] Given a `datetime_shift` rule without `base`, the default base timestamp
      `2077-04-27T12:34:56.789+01:00` is used.
- [x] Given `format: rfc3339`, matched timestamps are rewritten and kept in RFC
      3339 form.
- [x] Given `format: rfc1123`, matched timestamps are rewritten and kept in RFC
      1123 form.
- [x] `format` and `pattern` are treated as mutually exclusive.

## Notes

This task makes datetime shifting easier to configure for common formats while
keeping the output stable and predictable across multiple date styles.
