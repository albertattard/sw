---
id: TASK-143
title: Preserve Cleanup Line Continuations
status: done
category: run
related_features:
  - SPEC-003
owner: codex
created: 2026-05-03
updated: 2026-05-03
---

# Preserve Cleanup Line Continuations

## Context

Cleanup execution wraps top-level cleanup chunks so that later cleanup commands
still run after an earlier cleanup command fails. That chunking must not split a
single shell command across lines that use trailing `\` continuations.

## Decision

When a cleanup line ends with an active trailing `\`, the next physical line is
part of the same cleanup chunk. The cleanup runner should only close the chunk
after the continued command has ended.

## Acceptance Criteria

- [x] Cleanup commands that use trailing `\` line continuations execute as one
      cleanup chunk.
- [x] The generated cleanup script no longer wraps each continued physical line
      separately.
- [x] Existing best-effort cleanup behavior remains intact for separate cleanup
      commands.
