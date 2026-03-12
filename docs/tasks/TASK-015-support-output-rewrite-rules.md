---
id: TASK-015
title: Support Output Rewrite Rules
status: pending
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Support ordered output rewrite rules so rendered command output can be cleaned
or anonymised before it is written to Markdown.

## Scope

- Support `output.rewrite`
- Apply rewrite rules in the declared order
- Support `replace` rewrite rules
- Support `datetime_shift` rewrite rules
- Keep rewrite behavior limited to rendered output

## Assumptions

- Rewrite rules affect rendering only and do not alter execution or assertion
  behavior.
- `datetime_shift` operates independently within each command output block.
- Additional rewrite rule types may be added in later tasks.

## Acceptance Criteria

- [ ] Given a `Command` entry with `output.rewrite`, rewrite rules are applied
      in the declared order before output is rendered.
- [ ] Given a `replace` rewrite rule, matching text is replaced in rendered
      output.
- [ ] Given a `datetime_shift` rewrite rule, the first matched timestamp is
      rewritten to the configured base timestamp.
- [ ] Given multiple timestamps matched by the same `datetime_shift` rule,
      later timestamps preserve their relative distance from the first matched
      timestamp.

## Notes

This task prepares the runbook format for controlled output cleanup and
anonymisation without requiring users to edit generated Markdown by hand.
