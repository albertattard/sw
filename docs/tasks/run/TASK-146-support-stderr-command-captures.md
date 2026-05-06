---
id: TASK-146
title: Support Stderr Command Captures
status: done
category: run
related_features:
  - SPEC-003
owner: codex
created: 2026-05-06
updated: 2026-05-06
---

# Support Stderr Command Captures

## Context

Some command-line tools write operational metadata to stderr even when the
command succeeds. `codex exec`, for example, prints the session id on stderr,
which makes the existing stdout-only capture contract insufficient for
reusing that value in later entries.

## Decision

Allow `Command.capture.source` to be either `stdout` or `stderr`. The selected
source remains explicit and independent from `output.stream`, so authors can
choose the exact stream that contains the value they need without making
captures search both streams implicitly.

`stage: raw` reads the selected original stream. `stage: rewritten` applies the
command's `output.rewrite` rules to the selected stream before matching.

## Acceptance Criteria

- [x] `source: stderr` with `stage: raw` captures a value from raw stderr.
- [x] `source: stderr` with `stage: rewritten` captures a value from rewritten
      stderr.
- [x] `source: stdout` capture behavior remains unchanged.
- [x] Validation accepts `source: stderr`.
- [x] Validation rejects unsupported capture sources with a clear error.
- [x] `sw explain run` documents the supported capture sources.
