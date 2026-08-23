---
id: TASK-180
title: Add Runbook Execution Consent Guidance
status: done
category: explain
related_features:
  - SPEC-009
owner: albertattard
created: 2026-08-23
updated: 2026-08-23
---

## Summary

Make the generated skill distinguish explicit user authorization to execute a
runbook from cases where an agent must inspect the runbook and ask first.

## Scope

- Treat `sw run` as command execution
- Permit execution when the user explicitly requests it within a clear scope
- Require inspection and confirmation otherwise

## Acceptance Criteria

- [x] Given `sw explain --output-format=skill`, the skill permits `sw run`
      when the user has explicitly requested the runbook execution.
- [x] Given `sw explain --output-format=skill`, the skill tells agents to
      inspect and seek confirmation when execution is not explicitly requested.
- [x] Automated CLI tests cover the execution-consent guidance.
