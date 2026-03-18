---
id: TASK-068
title: Add Skill Output Format To Explain Command
status: done
category: explain
related_features:
  - SPEC-009
owner: @aattard
created: 2026-03-18
updated: 2026-03-18
---

## Summary

Extend `sw explain` so it can emit a `SKILL.md`-compatible Markdown document
that helps Codex-style agents use `sw` correctly without depending on direct
repository access.

## Scope

- Add `--output-format=skill` to `sw explain`
- Keep the existing human-readable text output as the default explain format
- Allow `--output-file` to write the generated skill document to the default
  Codex skill location for `sw`
- Allow `--output-file=<path>` to write the generated skill document to a file
- Fail when the output file already exists unless `--force` is provided
- Keep stdout output as the default when `--output-file` is not provided
- Make the skill content aggregate current `sw` guidance rather than dumping
  raw spec files
- Make implemented versus planned commands explicit in the generated skill
- Update help output and help-focused tests for the new flags
- Add integration coverage for stdout export, file export, overwrite refusal,
  and forced overwrite

## Assumptions

- `explain` remains the agent-facing contract surface for `sw`.
- The skill document is a derived artifact, not a second source of truth.
- The generated skill should guide agents toward `sw help`, `sw example`, and
  `sw explain` rather than duplicate every contract detail inline.
- The canonical option spelling in docs and examples should use the `=` form,
  such as `--output-format=skill` and `--output-file=<path>`, while still
  supporting the bare `--output-file` form for the default location.

## Acceptance Criteria

- [x] Given `sw explain --output-format=skill`, the CLI exits with `0` and
      prints a deterministic `SKILL.md`-compatible document to stdout.
- [x] Given `sw explain --output-format=skill --output-file`, the CLI exits
      with `0` and writes the document to the default Codex skill location
      for `sw`.
- [x] Given `sw explain --output-format=skill --output-file=<path>`, the CLI
      exits with `0` and writes the document to the provided path.
- [x] Given `sw explain --output-format=skill --output-file` when the default
      target file already exists, the CLI exits with `1` and leaves the file
      unchanged.
- [x] Given `sw explain --output-format=skill --output-file=<path>` when the
      target file already exists, the CLI exits with `1` and leaves the file
      unchanged.
- [x] Given `sw explain --output-format=skill --output-file --force` or
      `sw explain --output-format=skill --output-file=<path> --force`, the CLI
      overwrites the existing file and exits with `0`.
- [x] Given `sw explain --output-format=skill`, the generated content makes it
      clear which `sw` commands are implemented and which are still planned.
- [x] `sw explain --help` documents `--output-format=<format>`,
      `--output-file[=<path>]`, and `--force`.

## Notes

This increment should reuse the existing explain knowledge model so the human
and skill outputs stay aligned as the product evolves.
