---
id: TASK-139
title: Support Breakpoint Entry
status: done
category: run
related_features:
  - SPEC-003
  - SPEC-008
  - SPEC-009
owner: @aattard
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Add a `Breakpoint` runbook entry that lets authors intentionally stop runbook
processing while debugging or incrementally developing a workflow.

## Scope

- Accept `Breakpoint` as a valid runbook entry type
- Support an optional string `message`
- Stop `sw run` successfully when the breakpoint is reached
- Render a visible breakpoint note in generated Markdown
- Skip entries after the breakpoint
- Preserve unresolved Markdown placeholders that belong to skipped later entries
- Continue running cleanup and patch restoration registered before the breakpoint
- Stop prerequisite checking at the first breakpoint for both `sw run` and
  `sw check`
- Add `sw example Breakpoint` in YAML and JSON
- Surface the behavior through `sw explain run`, `sw explain check`, and help

## Assumptions

- A breakpoint is a debugging control point, not a workflow failure.
- `Breakpoint` should stay intentionally small; broader control-flow features
  should be separate tasks.

## Acceptance Criteria

- [x] Given a runbook with `Breakpoint`, `sw validate` accepts it.
- [x] Given a `Breakpoint` with an unsupported field, `sw validate` rejects it.
- [x] Given a runbook with `Breakpoint`, `sw run` exits with `0`, writes output
      up to the breakpoint, and skips later entries.
- [x] Given a `Breakpoint` with `message`, the generated Markdown includes that
      message in the breakpoint note.
- [x] Given a `Breakpoint` before a later `Command`, the later command is not
      executed.
- [x] Given a `Breakpoint` before a later `Prerequisite`, `sw check` does not
      evaluate the later prerequisite.
- [x] Given `sw example Breakpoint`, the CLI prints a valid YAML snippet.
- [x] Given `sw example Breakpoint --output-format json`, the CLI prints a
      valid JSON snippet.
- [x] Explain and help output make the breakpoint behavior discoverable.
