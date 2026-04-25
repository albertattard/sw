---
id: TASK-134
title: Support Patch Debug Diagnostics
status: done
category: run
related_features:
  - SPEC-003
  - SPEC-008
  - SPEC-009
owner: @aattard
created: 2026-04-25
updated: 2026-04-25
---

## Summary

Allow `Patch` entries to declare `debug: true` so authors can troubleshoot
patch application without enabling full-run diagnostics.

## Scope

- Accept `debug` as an optional boolean on `Patch` entries
- Reject non-boolean `Patch.debug` values during validation
- Emit Patch diagnostics when `Patch.debug: true`
- Emit Patch diagnostics for all Patch entries when global `sw run --debug` is
  used
- Include the resolved target path, patch working directory, normalized patch
  text, and patch command output in diagnostics
- Surface `debug` in `sw example Patch`
- Update explain/discovery text and tests

## Assumptions

- Debug output is for troubleshooting and is not a stable machine-readable
  contract.
- Patch debug does not change generated Markdown.
- Entry-scoped `debug: true` remains local to that entry.

## Acceptance Criteria

- [x] Given a `Patch` entry with `debug: true`, validation accepts the runbook.
- [x] Given a `Patch` entry with non-boolean `debug`, validation rejects the
      runbook.
- [x] Given a `Patch` entry with `debug: true`, `sw run` emits Patch
      diagnostics to stderr without global `--debug`.
- [x] Given global `sw run --debug`, Patch diagnostics are emitted even when
      the Patch entry does not declare `debug`.
- [x] Given a Patch debug run, generated Markdown is unchanged.
- [x] Given `sw example Patch`, the example includes `debug: true`.
- [x] Given `sw explain run`, the output documents Patch entry-scoped debug.
- [x] Existing tests pass.
