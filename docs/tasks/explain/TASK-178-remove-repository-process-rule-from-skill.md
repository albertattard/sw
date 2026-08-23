---
id: TASK-178
title: Remove Repository Process Rule From Skill
status: done
category: explain
related_features:
  - SPEC-009
owner: albertattard
created: 2026-08-23
updated: 2026-08-23
---

## Summary

Remove repository-contributor process guidance from the globally generated
user skill so it remains focused on authoring and executing runbooks.

## Scope

- Remove the instruction to update this repository's spec and task before
  user-visible changes
- Retain agent rules that help users author valid runbooks

## Acceptance Criteria

- [x] Given `sw explain --output-format=skill`, the output does not include
      repository-specific spec-and-task process guidance.
- [x] Automated CLI tests prevent that guidance from returning.
