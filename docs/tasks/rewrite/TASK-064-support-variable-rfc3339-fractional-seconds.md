---
id: TASK-064
title: Support Variable RFC3339 Fractional Seconds
status: open
category: rewrite
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-16
updated: 2026-03-16
---

## Summary

Extend built-in `datetime_shift` support for `format: rfc3339` so it matches
timestamps that use variable fractional-second precision, including JFR-style
nanosecond values.

## Scope

- Update built-in `rfc3339` matching to allow optional fractional seconds
- Support 1 to 9 fractional-second digits when that fraction is present
- Preserve the original textual RFC 3339 shape after rewriting
- Add CLI coverage for `format: rfc3339` with higher-than-millisecond
  precision timestamps

## Assumptions

- Users should not need a custom `pattern` and `custom_format` pair for common
  RFC 3339 timestamps that differ only in fractional precision.
- Existing millisecond-precision `rfc3339` behavior must remain supported.

## Acceptance Criteria

- [ ] Given `datetime_shift` with `format: rfc3339` and a matched timestamp
      without fractional seconds, the timestamp is rewritten successfully.
- [ ] Given `datetime_shift` with `format: rfc3339` and a matched timestamp
      with 3 fractional-second digits, the timestamp is rewritten
      successfully.
- [ ] Given `datetime_shift` with `format: rfc3339` and a matched timestamp
      with 9 fractional-second digits, the timestamp is rewritten
      successfully.
- [ ] The rewritten timestamp remains in RFC 3339 form and preserves the
      original fractional precision width.

## Notes

This is intended to remove the need for custom patterns when working with JFR
and similar event streams that emit RFC 3339 timestamps with nanosecond
precision.
