---
id: TASK-133
title: Surface Command Capture Stage Discovery
status: done
category: discovery
related_features:
  - SPEC-003
  - SPEC-009
owner: @aattard
created: 2026-04-25
updated: 2026-04-25
---

## Summary

Make command capture authoring clearer for humans and AI agents by documenting
the exact `capture` stage names, capture source boundary, and regex extraction
behavior in the discovery surfaces.

## Scope

- Update `sw explain run` to document `capture.source: stdout`
- Update `sw explain run` to document `capture.stage: raw|rewritten`
- Explain that `raw` captures before `output.rewrite`
- Explain that `rewritten` captures after `output.rewrite`
- Explain that the first regex capture group is stored when present, otherwise
  the full regex match is stored
- Update the runbook entity editing guide with a concrete capture example
- Add or update tests for the new explain output

## Assumptions

- This task does not change runbook execution or validation behavior.
- `stage: original` remains invalid; the documented values are `raw` and
  `rewritten`.
- `capture.source` remains limited to `stdout`.

## Acceptance Criteria

- [x] Given `sw explain run`, the output documents `capture.source: stdout`.
- [x] Given `sw explain run`, the output documents `capture.stage: raw` and
      `capture.stage: rewritten`.
- [x] Given `sw explain run`, the output explains the difference between
      `raw` and `rewritten`.
- [x] Given `sw explain run`, the output explains first-capture-group
      extraction.
- [x] The entity editing guide shows how to capture a value such as `109` from
      `Computed in 109 ms`.
- [x] Existing tests pass.
