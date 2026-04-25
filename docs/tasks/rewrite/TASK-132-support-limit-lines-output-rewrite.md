---
id: TASK-132
title: Support Limit Lines Output Rewrite
status: pending
category: rewrite
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-25
updated: 2026-04-25
---

## Summary

Add a `limit_lines` command output rewrite rule so long command output, such
as recursive stack traces, can be shortened while preserving the most relevant
leading and/or trailing lines.

## Scope

- Add `limit_lines` as a supported `output.rewrite` rule type
- Support `first` for keeping the first N output lines
- Support `last` for keeping the last N output lines
- Support using `first` and `last` together without duplicating overlapping
  lines
- Support `show_trim_marker`, defaulting to `true`
- Add validation coverage for accepted and rejected rule shapes
- Add run coverage for first-only, last-only, combined first/last, overlap,
  marker suppression, and no-trim cases
- Surface the new rule through `sw example Command` and `sw explain run`

## Assumptions

- The rule is line-based, not byte-based or character-based.
- `first` and `last` must be positive integers when present.
- At least one of `first` or `last` is required.
- If no lines are removed, output remains unchanged and no trim marker is
  added.
- The trim marker text is `...`, matching existing `keep_between` marker
  conventions.
- The rule applies wherever command output rewrites already apply and composes
  in declared rewrite order.

## Acceptance Criteria

- [ ] Given `limit_lines` with `first`, only the first N lines are kept and a
      trailing trim marker is added when lines were removed.
- [ ] Given `limit_lines` with `last`, only the last N lines are kept and a
      leading trim marker is added when lines were removed.
- [ ] Given `limit_lines` with both `first` and `last`, the first N and last M
      lines are kept with a single middle trim marker when lines were removed.
- [ ] Given `limit_lines` with both `first` and `last` where the kept ranges
      overlap, lines are not duplicated.
- [ ] Given `limit_lines` where the configured limits do not remove any lines,
      the output is unchanged and no marker is added.
- [ ] Given `limit_lines` with `show_trim_marker: false`, omitted lines do not
      render a marker.
- [ ] Given `limit_lines` without `first` or `last`, validation rejects the
      runbook.
- [ ] Given `limit_lines` with non-positive or non-integer `first` or `last`,
      validation rejects the runbook.
- [ ] Given `limit_lines` with non-boolean `show_trim_marker`, validation
      rejects the runbook.
- [ ] `sw example Command` includes a `limit_lines` rewrite example.
- [ ] `sw explain run` documents the `limit_lines` rewrite rule.

## Notes

This task does not change `keep_between`. Use `keep_between` when output has
stable textual boundaries and `limit_lines` when output length is the limiting
factor.
