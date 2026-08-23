---
id: TASK-177
title: Describe Tested Documentation Purpose In Skill
status: done
category: explain
related_features:
  - SPEC-009
owner: albertattard
created: 2026-08-23
updated: 2026-08-23
---

## Summary

Make the generated skill state that `sw` authors and executes tested,
executable documentation rather than presenting it only as a generic runbook
CLI.

## Scope

- Describe authoring and execution as joint purposes
- Explain that prerequisites and documented workflows are checked and run
- State that generated Markdown includes real command output
- Preserve the compact skill format

## Acceptance Criteria

- [x] Given `sw explain --output-format=skill`, the opening guidance describes
      `sw` as an authoring and execution tool for tested, executable
      documentation runbooks.
- [x] Given `sw explain --output-format=skill`, the opening guidance explains
      that real command output is incorporated into generated Markdown.
- [x] Automated CLI tests cover the purpose statement.
