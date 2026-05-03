---
id: TASK-142
title: Improve Cleanup Failure Diagnostics
status: done
category: run
related_features:
  - SPEC-003
owner: codex
created: 2026-05-03
updated: 2026-05-03
---

# Improve Cleanup Failure Diagnostics

## Context

Cleanup failures currently report only the cleanup exit code and stderr. That
makes it hard to identify which `Command` entry registered the failing cleanup,
especially in larger runbooks with multiple cleanup blocks.

## Decision

Cleanup failure diagnostics should behave like command failure diagnostics by
including the owning command entry and the captured cleanup output. Cleanup
debug output should also be available when the owning command enables
`debug: true` or when the run uses global debug mode.

## Acceptance Criteria

- [x] Failed cleanup diagnostics identify the owning `Command` entry.
- [x] Failed cleanup diagnostics include the cleanup working directory.
- [x] Failed cleanup diagnostics include the normalized cleanup script that was
      executed.
- [x] Failed cleanup diagnostics include cleanup stdout and stderr.
- [x] Failed cleanup diagnostics include the cleanup exit-code detail.
- [x] Command-scoped `debug: true` emits cleanup diagnostics for that command's
      cleanup block.
- [x] Global `sw run --debug` emits cleanup diagnostics for cleanup blocks.
- [x] Remaining cleanup blocks continue to run after one cleanup block fails.
