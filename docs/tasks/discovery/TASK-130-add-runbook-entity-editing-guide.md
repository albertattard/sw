---
id: TASK-130
title: Add Runbook Entity Editing Guide
status: done
category: discovery
related_features:
  - SPEC-003
  - SPEC-008
owner: @aattard
created: 2026-04-25
updated: 2026-04-25
---

## Summary

Add a human-oriented editing guide that explains the supported runbook entry
types and how to use them when authoring YAML runbooks.

## Scope

- Add `docs/guides/entities.md`
- Cover the supported entry types at a practical authoring level
- Link the editing guide from `docs/guides/README.md`
- Link the editing guide from the generated README source
- Keep the guide aligned with `sw example` and `sw explain` as the source-blind
  discovery surfaces

## Assumptions

- The guide should be YAML-first because file-backed runbook authoring defaults
  to YAML.
- The guide should teach when to use each entity, not duplicate every field
  from the specs.
- Field-by-field reference documentation remains deferred.

## Acceptance Criteria

- [x] `docs/guides/entities.md` explains each supported runbook entry type.
- [x] The guide includes practical YAML examples for common entities.
- [x] The guide explains how to discover current snippets with `sw example`.
- [x] The guide explains that `sw explain` remains the current behavior
      contract for agents.
- [x] `docs/guides/README.md` links to the editing guide.
- [x] The generated README links to the editing guide.
- [x] Documentation changes are validated where practical.

## Notes

This task changes documentation only. It does not add or change runbook entry
types.
