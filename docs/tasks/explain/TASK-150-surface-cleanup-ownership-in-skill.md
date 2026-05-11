---
id: TASK-150
title: Surface Cleanup Ownership In Skill
status: done
category: explain
related_features:
  - SPEC-009
owner: @aattard
created: 2026-05-10
updated: 2026-05-10
---

## Summary

Make the generated `sw` skill harder for agents to misread when authoring
`Command.cleanup`.

## Scope

- Add concise generated-skill guidance that explicit `Command.cleanup`
  replaces automatic process cleanup for that command entry
- Make the guidance action-oriented for agents authoring runbooks with
  long-lived processes
- Keep `sw explain --all` as the authoritative detailed contract
- Cover the generated skill output with CLI tests

## Assumptions

- The generated skill should prevent common authoring mistakes without
  duplicating the full run explanation.
- The existing `sw explain run` contract remains the detailed source for
  cleanup behavior.

## Acceptance Criteria

- [x] Given `sw explain --output-format=skill`, the generated skill tells
      agents that explicit `Command.cleanup` disables automatic process cleanup
      for that entry.
- [x] Given `sw explain --output-format=skill`, the generated skill tells
      agents that explicit cleanup must stop processes that should not keep
      running.
- [x] The human run guide also makes the automatic-cleanup versus
      explicit-cleanup ownership rule discoverable.
