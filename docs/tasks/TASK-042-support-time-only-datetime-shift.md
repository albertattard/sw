---
id: TASK-042
title: Support Time-Only Datetime Shift
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Extend `datetime_shift` so custom time-only formats such as `12:56:13.902`
can participate in semantic shifting without introducing a separate rewrite
type.

## Scope

- Support time-only `pattern` plus `custom_format` combinations in
  `datetime_shift`
- Borrow the date and offset from the configured or inherited base timestamp
  when shifting time-only values
- Render the shifted value back in the same time-only textual format
- Support time-only rules that reuse an existing shared anchor via `use`
- Do not introduce any built-in time-only `format` name for this increment
- Add integration coverage for standalone time-only shifting and shared-anchor
  reuse

## Assumptions

- `datetime_shift` remains the only semantic time/date shift rewrite type; no
  separate `time_shift` entry is introduced.
- If shifting a time-only value crosses midnight, the rendered result wraps
  naturally to the new clock time because only the time portion is displayed.

## Acceptance Criteria

- [x] Given a time-only `datetime_shift` rule, the first matched time-only
      value is rewritten to the configured or default base time.
- [x] Given multiple time-only values matched by the same rule, later values
      preserve their relative distance from the first matched time-only value.
- [x] Given a time-only `datetime_shift` rule that uses `use`, the rule follows
      the previously established shared anchor.
- [x] Given a time-only `datetime_shift` that crosses midnight, the rendered
      output wraps to the correct clock time.

## Notes

This keeps shared timeline anonymisation consistent across full datetimes and
time-only values instead of forcing time-only output back into literal
replacement rules.
