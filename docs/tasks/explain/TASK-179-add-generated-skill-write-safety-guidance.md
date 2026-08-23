---
id: TASK-179
title: Add Generated Skill Write Safety Guidance
status: done
category: explain
related_features:
  - SPEC-009
owner: albertattard
created: 2026-08-23
updated: 2026-08-23
---

## Summary

Warn agents about generated-document and runbook rewrites before they invoke
commands that write files using defaults.

## Scope

- State `sw run`'s default `./README.md` output target
- Direct agents to use `--output-file` when they do not intend that target
- State that `sw format` rewrites its input file in place
- Correct the default-runbook ambiguity wording for JSON, YAML, and YML

## Acceptance Criteria

- [x] Given `sw explain --output-format=skill`, the skill explains the
      `sw run` default output target and `--output-file` safeguard.
- [x] Given `sw explain --output-format=skill`, the skill says `sw format`
      rewrites its input in place.
- [x] Given `sw explain --output-format=skill`, the skill says "more than
      one" default runbook file rather than "both".
- [x] Automated CLI tests cover the safety guidance.
