---
id: TASK-071
title: Prefer Datetime Shift Guidance In Explain
status: done
category: explain
related_features:
  - SPEC-009
owner: @aattard
created: 2026-03-20
updated: 2026-03-20
---

## Summary

Extend the `sw explain` contract so agent-facing guidance steers models toward
`datetime_shift` when rewriting real dates and times, reducing fragile literal
replacements in generated runbooks and skills.

## Scope

- Add explicit guidance to `sw explain run` to prefer `output.rewrite` with
  `type: datetime_shift` for semantic dates and times
- Keep `replace` available for non-semantic text and unsupported formats
- Ensure the generated `SKILL.md` output inherits the same guidance
- Add CLI test coverage for both text explain output and skill export output

## Assumptions

- `sw explain run` is the primary authoring guidance surface for runbook
  rewrite behavior
- Skill export should remain a derived view of the explain knowledge model
- Agents benefit from explicit prioritisation when multiple rewrite strategies
  could appear to fit a date-like value

## Acceptance Criteria

- [x] Given `sw explain run`, the CLI tells agents to prefer
      `datetime_shift` over `replace` for semantic dates and times.
- [x] Given `sw explain run`, the CLI makes it clear that `replace` still
      applies to non-semantic text or unsupported date and time formats.
- [x] Given `sw explain --output-format=skill`, the generated skill content
      preserves the same datetime rewrite guidance.
- [x] Automated CLI tests cover both the explain topic output and the skill
      export output for this guidance.

## Notes

This increment clarifies existing rewrite capabilities for agents without
changing the underlying runbook schema.
