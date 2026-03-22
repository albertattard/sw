---
id: TASK-079
title: Support Command Output Stream Selection
status: pending
category: run
related_features:
  - SPEC-001
  - SPEC-003
  - SPEC-008
  - SPEC-009
owner: @aattard
created: 2026-03-22
updated: 2026-03-22
---

## Summary

Allow `Command` entries to choose whether rendered output comes from stdout,
stderr, or a combined stdout-then-stderr stream so documentation can include
tools that report meaningful output on stderr without shell-level redirection.

## Scope

- Accept `output.stream` on `Command` entries
- Support `stdout`, `stderr`, and `combined`
- Keep `stdout` as the default when `output.stream` is omitted
- Apply stream selection before output rewrites and output trimming
- Keep `capture.source` unchanged in this increment so capture remains limited
  to stdout
- Keep assertion-check sources unchanged in this increment
- Validate unsupported `output.stream` values
- Update `sw example Command` so the output shape includes `stream`
- Update `sw explain run` and `sw explain example` so agents can discover the
  new output field and its limits
- Add integration coverage for stdout, stderr, and combined rendering

## Assumptions

- Output stream selection is a rendering concern, not a process-execution
  concern
- `combined` means stdout followed by stderr in a deterministic order
- Avoid broadening capture and assertion source models in the same increment
  because that would expand the contract beyond the immediate rendering need

## Acceptance Criteria

- [ ] Given `output.stream: stdout`, rendered command output includes only
      captured stdout.
- [ ] Given `output.stream: stderr`, rendered command output includes only
      captured stderr.
- [ ] Given `output.stream: combined`, rendered command output includes
      captured stdout followed by captured stderr.
- [ ] Given no `output.stream`, rendered command output defaults to stdout.
- [ ] Given `output.stream: stderr` together with `output.rewrite`,
      rewrites apply to the selected stderr stream before rendering.
- [ ] Given `output.stream: combined` together with output trimming, trimming
      applies to the selected combined stream before rendering.
- [ ] Given an invalid `output.stream` value, validation rejects the runbook
      with a clear error.
- [ ] Given `output.stream: stderr` or `output.stream: combined`,
      `capture.source` and assertion-check sources keep their current
      contracts and are not implicitly widened.
- [ ] Given `sw example Command`, the printed JSON snippet includes `stream`
      as part of the current output contract.
- [ ] Given `sw explain run`, the CLI documents `output.stream` and the
      supported values `stdout`, `stderr`, and `combined`.
- [ ] Given `sw explain example`, the CLI notes that the `Command` example
      includes current nested output fields such as `trim_empty_lines` and
      `stream`.

## Notes

This increment solves the rendering problem directly without forcing users to
rewrite commands just to move stderr into stdout.
