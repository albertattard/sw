---
id: TASK-176
title: Link Runbook Entry Guide From Skill
status: done
category: explain
related_features:
  - SPEC-009
owner: albertattard
created: 2026-08-23
updated: 2026-08-23
---

## Summary

Give agents an optional repository reference for comparing runbook entry types
without replacing the installed CLI as the source of truth.

## Scope

- Link the generated skill to the runbook entry guide on GitHub
- Present the guide as an overview for cross-entry questions
- Preserve `sw explain` and `sw example` as the current-binary discovery
  surfaces

## Assumptions

- A repository link can be useful for a broad conceptual overview but can
  diverge from an installed binary's version.

## Acceptance Criteria

- [x] Given `sw explain --output-format=skill`, the output links to the
      runbook entry guide.
- [x] Given `sw explain --output-format=skill`, the output says to prefer
      `sw explain` and `sw example` for the installed binary's current
      contract.
- [x] Automated CLI tests cover the reference guidance.
