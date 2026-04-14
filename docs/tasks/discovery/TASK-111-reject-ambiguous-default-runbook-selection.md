---
id: TASK-111
title: Reject Ambiguous Default Runbook Selection
status: done
category: discovery
related_features:
  - SPEC-002
  - SPEC-003
  - SPEC-005
owner: @aattard
created: 2026-04-14
updated: 2026-04-14
---

## Summary

Fail fast when more than one default runbook file is present and the user does
not provide `--input-file`, instead of silently preferring one format over
another.

## Scope

- Reject ambiguous implicit input selection for `run`, `validate`, and `check`
- Keep stdin and explicit `--input-file` behavior unchanged
- Report a clear operational error that tells the user to provide
  `--input-file`
- Update explain output and automated coverage

## Assumptions

- Silent precedence between JSON and YAML is harder to reason about than an
  explicit failure when both exist.
- The ambiguity belongs to shared input discovery, so the behavior should stay
  consistent across commands that use the default runbook lookup.
- Users who keep multiple runbook formats in the same directory can choose
  deterministically by passing `--input-file`.

## Acceptance Criteria

- [x] Given no `--input-file` and exactly one default runbook file present,
      existing implicit file loading still works.
- [x] Given no `--input-file` and more than one default runbook file present,
      `sw run` exits with `1` and reports a clear ambiguity error.
- [x] Given no `--input-file` and more than one default runbook file present,
      `sw validate` exits with `1` and reports a clear ambiguity error.
- [x] Given no `--input-file` and more than one default runbook file present,
      `sw check` exits with `1` and reports a clear ambiguity error.
- [x] Explain output documents the ambiguity rule for default input discovery.

## Notes

This changes only the implicit file-discovery path. Explicit file paths and
stdin-based input remain unchanged.
