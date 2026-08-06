---
id: TASK-171
title: Support ChangeDirectory Entry
status: done
category: run
related_features:
  - SPEC-003
owner: albertattard
created: 2026-08-06
updated: 2026-08-06
---

## Goal

Allow a runbook to establish a persistent directory for subsequent normal
command entries without placing shell `cd` plumbing in generated documentation
or repeating a command-level working directory on every entry.

## Scope

- Add a `ChangeDirectory` entry with a required relative `path` and optional
  Markdown `contents`.
- Render the optional prose and an explicit working-directory note.
- Use the active directory for later commands, cleanup blocks, and file
  assertions unless a command declares its existing explicit override.
- Reconstruct the active directory for partial runs started with `--start-at`.
- Keep the existing execution-root behavior for non-command entries and
  prerequisites.

## Acceptance Criteria

- [x] A valid `ChangeDirectory` entry changes the directory for following
      commands without rendering a raw `cd` shell command.
- [x] Its optional scalar or array `contents` renders with capture
      interpolation, followed by the visible directory note.
- [x] Missing, absolute, escaping, non-existent, and non-directory paths fail
      with clear errors.
- [x] Command cleanup and relative file assertions use the active directory.
- [x] `Command.working_directory` and legacy `working_dir` retain their
      execution-root-relative override behavior.
- [x] `--start-at` reconstructs earlier directory changes without rendering or
      executing skipped entries.
- [x] Validation, formatting, conversion, examples, explain/help discovery,
      and integration tests recognize the new entry type.

## Notes

Prerequisite checks are intentionally out of scope because they run before
normal command processing and do not share that execution sequence.
