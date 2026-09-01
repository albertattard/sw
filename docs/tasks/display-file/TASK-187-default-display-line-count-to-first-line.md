---
id: TASK-187
title: Default Display Line Count To First Line
status: done
category: display-file
related_features:
  - SPEC-003
owner: albertattard
created: 2026-09-01
updated: 2026-09-01
---

## Summary

Allow a display entry to specify `line_count` without redundant
`start_line: 1`, so concise runbooks can render an opening snippet directly.

## Scope

- Accept `line_count` without `start_line` for `DisplayFile` and `DisplayUrl`
- Treat an omitted `start_line` as line 1 when `line_count` is present
- Preserve positive-integer validation for both fields
- Update the runbook specification, guidance, validation coverage, and
  rendering coverage

## Assumptions

- `DisplayFile` and `DisplayUrl` retain aligned line-slicing behavior because
  both use the same author-facing fields and rendering semantics.
- Existing entries that explicitly declare `start_line: 1` remain valid and
  render unchanged.

## Acceptance Criteria

- [x] Given `DisplayFile.line_count` without `start_line`, `sw validate`
      accepts the runbook and `sw run` renders the first requested lines.
- [x] Given `DisplayUrl.line_count` without `start_line`, `sw validate`
      accepts the runbook and `sw run` renders the first requested lines.
- [x] Given `start_line` or `line_count` less than `1`, validation still
      rejects the entry.
- [x] User-facing specification and guidance describe the default.
- [x] Automated tests pass after the change.
