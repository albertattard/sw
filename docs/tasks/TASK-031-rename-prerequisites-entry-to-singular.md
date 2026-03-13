---
id: TASK-031
title: Rename Prerequisites Entry To Singular
status: pending
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Rename the runbook entry type from `Prerequisites` to `Prerequisite` so it
matches the singular naming style used by the other entry types.

## Scope

- Update the accepted entry type name from `Prerequisites` to `Prerequisite`
- Align validation, rendering, and tests with the singular form
- Decide whether the old plural spelling remains temporarily supported or is
  rejected immediately

## Assumptions

- Entry type names should be consistent across the runbook format.
- This task focuses on the naming change and not on new prerequisite behavior.

## Acceptance Criteria

- [ ] Given a runbook with `type: "Prerequisite"`, validation accepts it.
- [ ] Given a runbook with `type: "Prerequisite"`, rendering and execution
      behave the same as the previous prerequisite entry behavior.
- [ ] The documented entry type in the spec uses the singular `Prerequisite`
      form consistently.

## Notes

This keeps the runbook vocabulary aligned with the singular entry naming style
used elsewhere, such as `Heading`, `Markdown`, `Command`, and `DisplayFile`.
