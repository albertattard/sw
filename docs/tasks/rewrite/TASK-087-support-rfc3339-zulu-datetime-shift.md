---
id: TASK-087
title: Support RFC3339 Zulu Datetime Shift
status: done
category: rewrite
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-24
updated: 2026-03-24
---

## Summary

Extend built-in `datetime_shift` support for `format: rfc3339` so it also
matches and rewrites UTC timestamps that use the `Z` suffix.

## Scope

- Update built-in `rfc3339` matching to accept `Z` in addition to numeric
  offsets
- Preserve `Z` output when the original matched timestamp used `Z`
- Keep existing numeric-offset `rfc3339` behavior unchanged
- Add CLI coverage for `Z` timestamps with and without fractional seconds

## Assumptions

- Users should not need a custom `pattern` and `custom_format` pair for common
  RFC3339 timestamps emitted with a `Z` suffix
- Preserving the original textual style means `Z`-suffixed timestamps remain
  `Z`-suffixed after rewriting

## Acceptance Criteria

- [x] Given `datetime_shift` with `format: rfc3339` and a matched timestamp
      like `2026-03-24T15:43:04Z`, the timestamp is rewritten successfully.
- [x] Given `datetime_shift` with `format: rfc3339` and a matched timestamp
      like `2026-03-24T15:43:04.734783532Z`, the timestamp is rewritten
      successfully.
- [x] Given `datetime_shift` with `format: rfc3339` and an original matched
      timestamp that uses `Z`, the rewritten timestamp remains in RFC3339 form
      with a `Z` suffix.
- [x] Existing numeric-offset `rfc3339` rewrite behavior remains supported.

## Notes

This removes the need for a custom rewrite pattern when tools emit RFC3339
timestamps in UTC using the standard `Z` suffix.
