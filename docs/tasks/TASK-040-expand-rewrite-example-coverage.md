---
id: TASK-040
title: Expand Rewrite Example Coverage
status: pending
related_features:
  - SPEC-008
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Extend the `example` command so users can request built-in rewrite-rule
examples for `replace`, `datetime_shift`, and rewrite scenarios that reference
captured variables.

## Scope

- Add `rewrite.replace` example output
- Add `rewrite.datetime_shift` example output
- Add a rewrite example that demonstrates captured-variable interpolation
- Update help coverage if topic examples are documented there
- Add integration coverage for the new example topics

## Assumptions

- `rewrite.keep_between` remains as an existing supported example topic.
- The richer rewrite examples are still JSON snippets, not full runbooks.

## Acceptance Criteria

- [ ] Given `sw example rewrite.replace`, the CLI prints a valid JSON fragment.
- [ ] Given `sw example rewrite.datetime_shift`, the CLI prints a valid JSON
      fragment.
- [ ] Given the capture-oriented rewrite example topic, the CLI prints a valid
      JSON fragment that demonstrates captured-variable usage in rewrite rules.
- [ ] Unknown rewrite-example topics still exit with `1`.

## Notes

This keeps the `example` command useful for the most configuration-heavy part
of the runbook format, where small copyable snippets are more valuable than a
full starter file.
