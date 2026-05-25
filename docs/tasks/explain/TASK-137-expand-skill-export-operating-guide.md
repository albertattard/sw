---
id: TASK-137
title: Expand Skill Export Operating Guide
status: done
category: explain
related_features:
  - SPEC-009
owner: albertattard
created: 2026-04-27
updated: 2026-04-27
---

## Summary

Expand the generated `sw` skill from a thin routing document into a compact
operating guide that gives agents enough context to use `sw` effectively while
still treating `sw explain --all` as the authoritative command contract.

## Scope

- Keep the generated skill deterministic and `SKILL.md` compatible
- Keep `sw explain --all` as the first authoritative discovery step
- Add concise workflow guidance for authoring, validating, checking, running,
  formatting, and converting runbooks
- Add YAML-first authoring defaults and stdin format guidance
- Add agent rules that prevent common mistakes such as inventing fields or
  editing generated README output directly
- Avoid embedding the full per-topic command map

## Assumptions

- The skill should help agents make the first correct move without duplicating
  the full explain contract.
- `sw explain`, `sw help`, and `sw example` remain the authoritative discovery
  surfaces for detailed behavior.

## Acceptance Criteria

- [x] Given `sw explain --output-format=skill`, the generated skill includes a
      first-step instruction to run `sw explain --all`.
- [x] Given `sw explain --output-format=skill`, the generated skill includes
      common workflow guidance for major subcommands.
- [x] Given `sw explain --output-format=skill`, the generated skill includes
      YAML-first authoring defaults and stdin format guidance.
- [x] Given `sw explain --output-format=skill`, the generated skill includes
      agent rules for discovery, validation, and generated README handling.
- [x] Given `sw explain --output-format=skill`, the generated skill does not
      embed the full per-topic command map.
