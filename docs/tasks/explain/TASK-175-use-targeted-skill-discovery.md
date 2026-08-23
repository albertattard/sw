---
id: TASK-175
title: Use Targeted Skill Discovery
status: done
category: explain
related_features:
  - SPEC-009
owner: albertattard
created: 2026-08-23
updated: 2026-08-23
---

## Summary

Adjust the generated Codex skill so agents choose the narrowest authoritative
discovery command for their task instead of always loading the full explain
catalogue.

## Scope

- Direct behaviour and defaults questions to `sw explain <topic>`
- Direct exact syntax questions to `sw help <command>`
- Direct unfamiliar entry shapes to `sw example <EntryType>`
- Reserve `sw explain --all` for broad discovery
- Preserve the compact, deterministic skill format

## Assumptions

- The aggregate explain output is valuable when an agent needs a broad view,
  but unnecessary context for a focused task.
- The existing command boundaries remain authoritative.

## Acceptance Criteria

- [x] Given `sw explain --output-format=skill`, the skill recommends targeted
      `explain`, `help`, and `example` discovery.
- [x] Given `sw explain --output-format=skill`, the skill presents
      `sw explain --all` as broad discovery rather than its mandatory first
      step.
- [x] Automated CLI tests cover the revised discovery guidance.
