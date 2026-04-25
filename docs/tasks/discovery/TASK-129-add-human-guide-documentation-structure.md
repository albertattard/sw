---
id: TASK-129
title: Add Human Guide Documentation Structure
status: done
category: discovery
related_features:
  - SPEC-001
  - SPEC-009
owner: @aattard
created: 2026-04-25
updated: 2026-04-25
---

## Summary

Add a human-oriented guide structure under `docs/guides/` while keeping
`sw explain` as the primary discovery surface for AI agents.

## Scope

- Add a navigable guide index under `docs/guides/`
- Add one guide page per supported subcommand
- Update the generated README source to include a 5-minute workflow and links
  to the guide index
- Keep the `sw` skill output minimal so it remains a routing layer to
  `sw explain`

## Assumptions

- Human docs should teach common workflows and link to specs for the full
  contract.
- AI agents should use `sw explain` and `sw example` before relying on static
  guide pages.
- A separate field-by-field runbook reference is not needed until the guide
  pages become overloaded.
- `sw explain` subtopic filtering is deferred until the existing topic output
  becomes too large or agents repeatedly miss details.

## Acceptance Criteria

- [x] `docs/guides/README.md` provides navigation for human documentation.
- [x] Each supported subcommand has a guide page under `docs/guides/`.
- [x] The README includes a 5-minute workflow.
- [x] The README links to the guide index.
- [x] The guide docs clearly direct AI agents to `sw explain --all` and
      `sw example <topic>` instead of treating guides as the source of truth.
- [x] Documentation changes are validated where practical.

## Notes

This task changes documentation only. It does not add CLI behavior, change
runbook validation, or expand `sw explain`.
