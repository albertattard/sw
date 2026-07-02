---
id: TASK-170
title: Preserve Custom Datetime Shift Offsets
status: done
category: rewrite
related_features:
  - SPEC-003
owner: albertattard
created: 2026-07-02
updated: 2026-07-02
---

## Summary

Prevent `datetime_shift` from panicking when a custom datetime format includes
a numeric UTC offset, and preserve that offset in the rewritten output.

## Scope

- Format custom datetime values with their fixed offset intact
- Add a CLI regression test for an Apache-style timestamp using `%z`
- Preserve existing custom date-time and time-only rewrite behavior

## Assumptions

- `custom_format` follows Chrono format semantics.
- A custom format containing `%z` is a supported offset-aware date-time
  format, not a validation error.

## Acceptance Criteria

- [x] Given a custom `datetime_shift` format containing `%z`, `sw run`
      completes without a panic.
- [x] The rewritten timestamp renders the numeric UTC offset requested by the
      custom format.
- [x] Existing custom date-time and time-only rewrite tests continue to pass.

## Notes

The previous implementation converted the parsed `DateTime<FixedOffset>` to a
naive local datetime before formatting. That removed the offset required by
`%z`, causing Chrono's formatter to fail and Rust's string conversion to
panic.
