---
id: TASK-127
title: Support Numeric Captures And Markdown Arithmetic
status: pending
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-23
updated: 2026-04-23
---

## Summary

Allow runbooks to parse captured command output as numbers and use those
numeric values inside Markdown arithmetic expressions without making the
default parsing behavior depend on the host locale.

## Scope

- Add `capture[*].parse_as.type: number`
- Support canonical numeric parsing when no locale or explicit separators are
  declared
- Support `parse_as.locale: system`
- Support explicit locale-based parsing such as `parse_as.locale: "en"`
- Support explicit `decimal_separator` and `grouping_separator` parsing rules
- Reject runbooks that mix `locale` with explicit separators in the same
  numeric parse configuration
- Add Markdown arithmetic interpolation using `@{= expression }`
- Support `+`, `-`, `*`, `/`, and parentheses in Markdown expressions
- Keep plain `@{name}` interpolation bound to the original captured string
- Exclude locale-aware output formatting from this increment

## Assumptions

- Bare `parse_as.type: number` should remain deterministic and therefore use
  canonical parsing rather than the system locale.
- `locale: system` is intentionally explicit because it is environment
  dependent and should never be the default parsing mode.
- Numeric parsing should augment captured variables for expressions rather than
  rewriting the original captured string value.
- Arithmetic support is needed in Markdown first; extending the same syntax to
  commands, rewrites, or assertions can be evaluated later.

## Acceptance Criteria

- [ ] Given `capture.parse_as.type: number` with no other parsing options, the
      capture is parsed using decimal `.` and no grouping separator.
- [ ] Given `capture.parse_as.locale: system`, the capture is parsed using the
      locale under which `sw` is running.
- [ ] Given `capture.parse_as.locale: "en"` or another explicit locale, the
      capture is parsed using that locale.
- [ ] Given explicit `decimal_separator` and `grouping_separator`, the capture
      is parsed using those symbols.
- [ ] Given `capture.parse_as.locale` together with explicit separators,
      validation rejects the runbook.
- [ ] Given a numeric capture, plain `@{name}` interpolation still renders the
      original captured string.
- [ ] Given Markdown content with `@{= g1_8g_throughput * g1_8g_time_taken }`,
      the rendered Markdown includes the computed numeric result.
- [ ] Given Markdown arithmetic expressions, supported operators include `+`,
      `-`, `*`, `/`, and parentheses.
- [ ] Given a Markdown arithmetic expression that references a variable
      without a parsed numeric value, the run fails clearly.
- [ ] Given a numeric parse rule whose matched text does not conform to the
      declared parsing rules, the run fails clearly.

## Notes

This increment is about deterministic numeric parsing and arithmetic over
numeric captures. Locale-aware output formatting such as grouping separators or
localized decimal rendering is intentionally out of scope.
