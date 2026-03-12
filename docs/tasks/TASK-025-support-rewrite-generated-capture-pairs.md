---
id: TASK-025
title: Support Rewrite Generated Capture Pairs
status: pending
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Allow rewrite rules to generate paired captured variables for the original and
rewritten values without requiring separate top-level capture definitions.

## Scope

- Support `capture_as` on rewrite rules
- Generate `@{<capture_as>_original}` from the matched pre-rewrite value
- Generate `@{<capture_as>_rewritten}` from the rewritten value
- Make those generated variables available to later commands and Markdown
- Validate collisions with explicit capture names and other generated names

## Assumptions

- Generated names follow the same runbook-wide uniqueness rules as explicit
  `capture` names.
- In this increment, `capture_as` is limited to rewrite rules and does not
  replace the existing top-level `capture` section.

## Acceptance Criteria

- [ ] Given a rewrite rule with `capture_as`, the matched pre-rewrite value is
      stored as `@{<capture_as>_original}`.
- [ ] Given a rewrite rule with `capture_as`, the rewritten value is stored as
      `@{<capture_as>_rewritten}`.
- [ ] Given a later command or Markdown entry that uses those generated
      variables, the interpolated values match the captured original and
      rewritten forms.
- [ ] Given a rewrite rule with `capture_as` whose generated names collide with
      an existing explicit or generated capture name, validation rejects the
      runbook.

## Notes

This task reduces duplicated capture definitions for common rewrite flows where
the runbook wants to preserve both the original and normalized forms of the
same emitted value.
