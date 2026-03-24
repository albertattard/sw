---
id: TASK-086
title: Make Combined The Default Command Output Stream
status: done
category: run
related_features:
  - SPEC-003
  - SPEC-008
  - SPEC-009
owner: @aattard
created: 2026-03-24
updated: 2026-03-24
---

## Summary

Change the default rendered command output stream from stdout-only to combined
stdout-then-stderr so runbooks capture the full visible command story unless an
entry explicitly narrows the stream.

## Scope

- Change the default `output.stream` behavior from `stdout` to `combined`
- Keep explicit `stdout`, `stderr`, and `combined` values unchanged
- Update `sw example Command` so the example reflects the new default
- Update `sw explain run` so the documented default matches the renderer
- Update automated coverage for the omitted `output.stream` case

## Assumptions

- `combined` still means stdout followed by stderr in deterministic order
- Stream selection remains a rendering concern only
- Capture and assertion source contracts remain unchanged in this increment

## Acceptance Criteria

- [x] Given a `Command` entry with `output` and no `output.stream`, rendered
      command output includes captured stdout followed by captured stderr.
- [x] Given `output.stream: stdout`, rendered command output still includes
      only captured stdout.
- [x] Given `output.stream: stderr`, rendered command output still includes
      only captured stderr.
- [x] Given `output.stream: combined`, rendered command output still includes
      captured stdout followed by captured stderr.
- [x] Given `sw example Command`, the printed JSON snippet shows
      `"stream": "combined"`.
- [x] Given `sw explain run`, the CLI documents that omitted `output.stream`
      defaults to `combined`.

## Notes

This increment makes the out-of-the-box rendered output more representative for
tools that naturally write meaningful diagnostics or status lines to stderr.
