---
id: TASK-033
title: Implement Import Command
status: done
category: import
related_features:
  - SPEC-006
owner: @aattard
created: 2026-03-13
updated: 2026-04-02
---

## Summary

Add `sw import` so users can bootstrap a runbook from an existing README and
then refine the generated JSON with runbook-specific details.

## Scope

- Add an `import` CLI subcommand
- Support `--input-file`, `--output-file`, and `--force`
- Default to `./README.md` input and `./sw-runbook.json` output
- Import headings, Markdown prose, and fenced shell code blocks
- Generate a valid runbook JSON file

## Assumptions

- Import is intentionally lossy.
- This increment focuses on common Markdown structures, not every Markdown
  feature.
- The generated output should be easy to edit manually afterward.

## Acceptance Criteria

- [x] Given a README with headings, prose, and fenced shell blocks, `sw import`
      writes a valid runbook JSON file.
- [x] Given an existing target file without `--force`, `sw import` exits with
      `1` and does not overwrite it.
- [x] Given `--force`, `sw import` overwrites the target file.
- [x] The generated runbook passes `sw validate`.

## Notes

This command is intended for converting existing documentation into a useful
starting runbook, not for reconstructing every runbook-only behavior from the
rendered Markdown.
