---
id: TASK-164
title: Start Runbook At Entry
status: done
category: run
related_features:
  - SPEC-003
owner: albertattard
created: 2026-06-09
updated: 2026-06-09
---

## Summary

Add a `sw run --start-at <entry-number>` debugging flag that starts a partial
run at a 1-based runbook entry number.

## Scope

- Add the `--start-at <entry-number>` run flag
- Support the flag on explicit and implicit `run`
- Validate that the selected entry number is within the runbook entry range
- Skip rendering and execution before the selected entry
- Run prerequisite checks only from the selected entry range
- Preserve original verbose progress numbering and total entry count
- Produce partial debug output starting at the selected entry

## Assumptions

- Numeric entry selection is intentionally simple and may be replaced or
  complemented by stable entry IDs later.
- A start-at run assumes the workspace already contains any state created by
  skipped entries.

## Acceptance Criteria

- [x] Given `--start-at 2`, entry 1 is not rendered or executed and entry 2 is
      the first processed entry.
- [x] Given `--start-at 0`, the run fails with an operational error.
- [x] Given `--start-at` greater than the number of entries, the run fails with
      an operational error.
- [x] Given `--start-at` and `--verbose`, progress preserves original entry
      numbers and total count.
- [x] Given prerequisites before the selected entry, those prerequisite checks
      are skipped.
- [x] Help and spec output document the debugging flag.
