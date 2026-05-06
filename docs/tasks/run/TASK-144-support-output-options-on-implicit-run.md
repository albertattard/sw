---
id: TASK-144
title: Support Output Options On Implicit Run
status: done
category: run
related_features:
  - SPEC-003
owner: codex
created: 2026-05-05
updated: 2026-05-05
---

# Support Output Options On Implicit Run

## Context

The CLI contract says that invoking `sw` without a subcommand is equivalent to
`sw run`. In practice, top-level `sw` accepted shared run input, verbose, and
debug options, but rejected run output options such as `--output-file`.

## Decision

The implicit run path should accept the same run output options as the explicit
`sw run` path. This keeps the default command behavior predictable and avoids
forcing users to switch to `sw run` only when they need a custom output path.

## Acceptance Criteria

- [x] `sw --output-file <path>` writes generated Markdown to the requested
      path.
- [x] `sw --output-format markdown` is accepted by the implicit run path.
- [x] Top-level help shows run output options that are valid for implicit run.
- [x] Explicit `sw run --output-file <path>` behavior is unchanged.
