---
id: TASK-114
title: Clarify Format Defaults By Workflow
status: done
category: discovery
related_features:
  - SPEC-001
  - SPEC-002
  - SPEC-003
  - SPEC-004
  - SPEC-005
  - SPEC-006
  - SPEC-008
  - SPEC-009
owner: @aattard
created: 2026-04-15
updated: 2026-04-15
---

## Summary

Make the default-format contract explicit across discovery surfaces: file-based
runbook workflows default to YAML, while stdin-backed runbook input via
`--input-file=-` defaults to JSON unless `--input-format=yaml` is provided.

## Scope

- Update the relevant specs so the format split is stated deliberately rather
  than implied
- Align README wording with the same contract
- Update CLI help and `sw explain` output to surface the distinction where it
  affects users and agents
- Add or update automated coverage for the updated discovery text

## Assumptions

- YAML is the better default for files users edit directly.
- JSON remains the better default for stdin because it is the stricter and more
  predictable agent-to-tool boundary.
- The contract should be explained by workflow, not by forcing one format into
  every interface.

## Acceptance Criteria

- [x] README states that file-based runbook workflows default to YAML.
- [x] README states that stdin-backed runbook input defaults to JSON when
      `--input-file=-` is used unless `--input-format=yaml` is provided.
- [x] The relevant specs describe the same split without contradicting current
      command behavior.
- [x] `sw help` surfaces the split where users choose between file-backed and
      stdin-backed runbook input.
- [x] `sw explain` surfaces the split for the relevant topics and no longer
      describes `example` as JSON-only.

## Notes

This task clarifies the contract only. It does not change parsing behavior or
file-generation behavior.
