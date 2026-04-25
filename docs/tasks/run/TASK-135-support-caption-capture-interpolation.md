---
id: TASK-135
title: Support Caption Capture Interpolation
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-25
updated: 2026-04-25
---

## Summary

Allow `Command.output.caption` to interpolate captured variables, including
captures produced by the same command entry, so captions can describe the
rendered output using values extracted from that output.

## Scope

- Interpolate `@{name}` in `Command.output.caption`
- Preserve literal capture syntax with `@@{name}`
- Allow captions to reference captures available at output-render time,
  including captures from the same `Command` entry
- Keep captions from deferring to captures produced by later entries
- Surface the behavior through `sw explain run`
- Add run coverage for same-command caption interpolation and escaping

## Assumptions

- This applies to command output captions only.
- Timeout partial output keeps existing behavior; same-command captures may not
  be available on timeout.
- Plain `@{name}` interpolation keeps using the captured string value.

## Acceptance Criteria

- [x] Given `Command.output.caption` references a same-command capture, the
      rendered Markdown includes the captured value.
- [x] Given `Command.output.caption` references an earlier capture, the
      rendered Markdown includes the captured value.
- [x] Given `@@{name}` in `Command.output.caption`, the rendered Markdown
      includes literal `@{name}` text.
- [x] Given `Command.output.caption` references an unknown capture, the run
      fails with a clear interpolation error.
- [x] Given `sw explain run`, the output documents caption interpolation.
- [x] Existing tests pass.
