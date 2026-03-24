---
id: TASK-082
title: Minimize Skill Export To Discovery Entrypoint
status: done
category: explain
related_features:
  - SPEC-009
owner: @aattard
created: 2026-03-24
updated: 2026-03-24
---

## Summary

Reduce the generated `SKILL.md` output to a compact routing document that
points agents at `sw explain --all` instead of embedding a long command map.

## Scope

- Keep YAML frontmatter in the generated skill export
- Replace the embedded command map with concise discovery guidance
- Direct agents to start with `sw explain --all`
- Keep the guidance deterministic and minimal
- Update skill-export tests to validate the new compact contract

## Assumptions

- `sw explain --all` is the primary agent-facing discovery surface.
- The exported skill should route agents to authoritative CLI output rather
  than duplicate large parts of the explain contract.

## Acceptance Criteria

- [x] Given `sw explain --output-format=skill`, the generated skill remains a
      valid Codex `SKILL.md` file.
- [x] Given `sw explain --output-format=skill`, the generated skill tells
      agents to start with `sw explain --all`.
- [x] Given `sw explain --output-format=skill`, the generated skill tells
      agents to treat `sw` output as authoritative over cached assumptions.
- [x] Given `sw explain --output-format=skill`, the generated skill does not
      embed the full per-topic command map.

## Notes

This keeps the skill small and makes `sw explain --all` the single discovery
entry point for agents.
