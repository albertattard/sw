---
id: TASK-055
title: Refine Verbose Run Time Formatting
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-15
updated: 2026-03-15
---

## Summary

Refine verbose run progress output so elapsed time is easier to scan and
running commands show their expected timeout window.

## Scope

- Format elapsed time as seconds with one decimal place under one minute
- Format elapsed time as minutes and seconds from one minute onward
- Show the expected timeout window for `Command` entries
- Use the default command timeout when a `Command` entry does not declare one
- Remove the trailing `...` from the live timer display
- Add CLI coverage for the refined verbose time output

## Assumptions

- Non-command entries continue to show elapsed time only.
- Command timeout expectations are shown only in verbose progress output.
- The timeout display is informational and does not change actual timeout
  behavior.

## Acceptance Criteria

- [x] Given `sw run --verbose`, elapsed time under one minute is rendered like
      `12.4s`.
- [x] Given `sw run --verbose`, elapsed time from one minute onward is rendered
      like `1m 8s`.
- [x] Given a running `Command` entry in verbose mode, progress shows elapsed
      time together with the expected timeout window.
- [x] Given a `Command` entry without an explicit timeout, verbose mode shows
      the default timeout window.
- [x] Given a running entry in verbose mode, the timer does not include a
      trailing `...`.

## Notes

This keeps verbose progress readable during long-running commands and gives
users a clearer sense of how current execution time compares to the expected
timeout.
