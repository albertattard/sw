---
id: TASK-054
title: Add Verbose Run Progress Output
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-15
updated: 2026-03-15
---

## Summary

Add a `--verbose` option to `sw run` so long-running runbooks report which
entry is currently being processed.

## Scope

- Add a `--verbose` flag to the `run` command
- Accept `--verbose` before the subcommand so `sw --verbose` maps to the
  default `run` behavior
- Emit one short progress line before each runbook entry begins
- Keep verbose progress on stderr so stdout remains stable
- Pad entry numbers so summaries align to the same starting column
- Summarize `Markdown` entries using the first non-empty content line
- Summarize `Command` entries using the first non-empty command line
- Show a live elapsed timer for the current entry when stderr is a TTY
- Fall back to non-live line-based progress when stderr is not a TTY
- Add CLI coverage for verbose progress output

## Assumptions

- The default `run` output remains unchanged when `--verbose` is not provided.
- Verbose progress output is concise and does not dump full long-form content.
- Entry summaries may be truncated for readability.
- Live timer updates should not make non-TTY logs noisy or hard to read.

## Acceptance Criteria

- [x] Given `sw run --verbose`, the CLI prints one progress line per entry to
      stderr.
- [x] Given `sw --verbose` with no subcommand, the CLI behaves the same as
      `sw run --verbose`.
- [x] Given `sw run --verbose`, entry numbers are padded to align summaries.
- [x] Given `sw run --verbose`, `Markdown` entries use the first non-empty
      content line as the progress summary.
- [x] Given `sw run --verbose`, `Command` entries use the first non-empty
      command line as the progress summary.
- [x] Given `sw run --verbose` on a TTY, the current entry line shows a
      live-updating elapsed timer until the entry completes.
- [x] Given `sw run --verbose` without a TTY stderr stream, progress output
      falls back to non-live line-based output.
- [x] Given `sw run` without `--verbose`, the current stdout and stderr
      behavior remains unchanged.

## Notes

This helps users and agents understand where time is being spent during long
runbook execution without turning `run` into a full debug log.
