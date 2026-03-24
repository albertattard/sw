---
id: TASK-085
title: Add Selectable Verbose Progress Modes
status: pending
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-24
updated: 2026-03-24
---

## Summary

Add a selectable verbose progress mode so remote and SSH-driven workflows can
force line-based progress output instead of relying on in-place terminal
redraws.

## Scope

- Add `--verbose-mode auto|live|plain`
- Accept `--verbose-mode` before the subcommand so it works with the default
  `run` behavior
- Keep `auto` as the default mode
- Keep `live` behavior for interactive local terminals
- Add a forced `plain` mode for SSH-safe line-based progress output
- Make plain mode print both entry start and entry completion lines
- Add CLI coverage for mode selection and plain progress behavior

## Assumptions

- `--verbose-mode` has no effect unless `--verbose` is enabled.
- `auto` should preserve the current behavior for existing local users.
- `plain` mode is primarily for remote terminals, wrappers, and logs where
  carriage-return redraws are fragile.

## Acceptance Criteria

- [ ] Given `sw run --verbose --verbose-mode=auto`, verbose output keeps the
      existing automatic live-vs-plain behavior.
- [ ] Given `sw run --verbose --verbose-mode=live`, verbose output uses
      in-place timer updates when possible.
- [ ] Given `sw run --verbose --verbose-mode=plain`, verbose output uses
      line-based progress even when stderr is a TTY.
- [ ] Given plain mode, each entry prints a start line when it begins and a
      completion line when it finishes.
- [ ] Given `sw --verbose --verbose-mode=plain`, the default `run` behavior
      accepts the same mode selection without requiring the `run` subcommand.
- [ ] Help and explain output document the new verbose mode flag and when to
      prefer `plain`.

## Notes

This keeps local live timers available while giving remote execution paths a
predictable progress mode that does not depend on terminal redraw support.
