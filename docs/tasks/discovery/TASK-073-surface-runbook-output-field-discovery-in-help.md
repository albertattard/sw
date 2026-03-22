---
id: TASK-073
title: Surface Runbook Output Field Discovery In Help
status: done
category: discovery
related_features:
  - SPEC-001
owner: @aattard
created: 2026-03-22
updated: 2026-03-22
---

## Summary

Refine `sw help` so users looking at `run` help can discover where runbook
output fields such as `trim_empty_lines` are documented, without turning help
into a full schema dump.

## Scope

- Keep `run` help focused on CLI flags and invocation syntax
- Add targeted guidance in `run` help that points users to `sw example Command`
  and `sw explain run` for runbook-authored output fields
- Add help-focused CLI coverage for that discovery path

## Assumptions

- `help` remains the syntax-first discovery surface
- Runbook schema details still belong primarily to `example` and `explain`
- Short cross-references in help are sufficient for this increment

## Acceptance Criteria

- [x] Given `sw help run`, the CLI still documents the implemented `run` flags.
- [x] Given `sw help run`, the CLI points users to `sw example Command` and
      `sw explain run` for runbook-authored output fields such as
      `trim_empty_lines`.
- [x] Help-focused automated tests cover the updated guidance.

## Notes

This keeps the help surface concise while still making new runbook fields
discoverable from the CLI.
