---
id: TASK-145
title: Support Regex Keep Between Boundaries
status: done
category: rewrite
related_features:
  - SPEC-003
owner: codex
created: 2026-05-06
updated: 2026-05-06
---

# Support Regex Keep Between Boundaries

## Context

`keep_between` currently requires literal full-line `start` and `end`
boundaries. That makes it awkward to keep sections that begin or end with lines
containing volatile values such as generated session IDs.

## Decision

Add explicit regex boundary fields to `keep_between`: `start_pattern` and
`end_pattern`. Existing `start` and `end` fields remain literal full-line
matches so existing runbooks keep their current behavior.

## Acceptance Criteria

- [x] `start_pattern` can be used instead of `start` to match a start boundary
      with a regex.
- [x] `end_pattern` can be used instead of `end` to match an end boundary with
      a regex.
- [x] Validation rejects rules that specify both `start` and `start_pattern`.
- [x] Validation rejects rules that specify both `end` and `end_pattern`.
- [x] Validation rejects invalid `start_pattern` and `end_pattern` regexes.
- [x] Literal `start` and `end` behavior remains unchanged.
