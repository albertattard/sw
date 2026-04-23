---
id: TASK-126
title: Surface Command Debug Discovery In Explain And Example
status: done
category: discovery
related_features:
  - SPEC-008
  - SPEC-009
owner: @aattard
created: 2026-04-23
updated: 2026-04-23
---

## Summary

Make command-scoped debugging easier to discover from `sw explain` and
`sw example` so users and agents can find `debug: true` without reading the
full run spec.

## Scope

- Update `sw explain run` to describe `Command.debug` alongside global
  `--debug`
- Update `sw explain example` so it explicitly notes that the `Command`
  example includes `debug`
- Update `sw example Command` in both YAML and JSON forms so it includes
  `debug: true`
- Add explain- and example-focused CLI coverage for the new guidance

## Assumptions

- The problem here is discoverability, not runtime support.
- `sw explain run` is the right place to describe the boundary between
  command-scoped and global debug diagnostics.
- The `Command` example should remain a fuller starting point that users trim
  down, rather than a minimal shell.

## Acceptance Criteria

- [x] Given `sw explain run`, the output documents `Command.debug` and explains
      that `debug: true` enables diagnostics for only that command unless
      global `--debug` is enabled.
- [x] Given `sw explain example`, the output notes that the `Command` example
      includes `debug`.
- [x] Given `sw example Command`, the YAML example includes `debug: true`.
- [x] Given `sw example Command --output-format json`, the JSON example
      includes `"debug": true`.
- [x] Explain- and example-focused automated tests cover the updated guidance.

## Notes

This is a discovery improvement only. It does not change command execution,
debug output content, or validation behavior.
