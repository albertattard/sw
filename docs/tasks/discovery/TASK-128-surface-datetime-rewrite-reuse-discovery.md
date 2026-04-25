---
id: TASK-128
title: Surface Datetime Rewrite Reuse Discovery
status: done
category: discovery
related_features:
  - SPEC-008
  - SPEC-009
  - SPEC-003
owner: @aattard
created: 2026-04-25
updated: 2026-04-25
---

## Summary

Make command output rewrite reuse easier to discover from the CLI so users and
agents without source access can distinguish shared datetime timelines from
captured rewritten values.

## Scope

- Update `sw explain run` to describe `datetime_shift.id` and
  `datetime_shift.use`
- Update `sw explain run` to describe rewrite `capture_as` and the generated
  `@{<capture_as>_original}` and `@{<capture_as>_rewritten}` variables
- Update `sw example Command` in both YAML and JSON forms so a rewrite rule
  includes `capture_as`
- Add explain- and example-focused CLI coverage for the updated guidance

## Assumptions

- Runtime support already exists; this task is a discovery improvement only.
- `id` / `use` should be described as timeline reuse.
- `capture_as` should be described as value reuse.
- `sw explain run` and `sw example Command` are the right source-blind
  discovery surfaces for another agent.

## Acceptance Criteria

- [x] Given `sw explain run`, the output documents that `datetime_shift.id`
      establishes a shared shift anchor and `datetime_shift.use` reuses an
      earlier anchor.
- [x] Given `sw explain run`, the output documents that rewrite `capture_as`
      creates `@{<capture_as>_original}` and
      `@{<capture_as>_rewritten}` variables.
- [x] Given `sw explain run`, the output distinguishes shared timeline reuse
      from rewritten value reuse.
- [x] Given `sw example Command`, the YAML example includes `capture_as` on an
      output rewrite rule.
- [x] Given `sw example Command --output-format json`, the JSON example
      includes `capture_as` on an output rewrite rule.
- [x] Explain- and example-focused automated tests cover the updated guidance.

## Notes

This task does not change command execution, output rewriting, capture
semantics, or validation behavior.
