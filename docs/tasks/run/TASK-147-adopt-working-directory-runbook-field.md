---
id: TASK-147
title: Adopt Working Directory Runbook Field
status: pending
category: run
related_features:
  - SPEC-003
owner: codex
created: 2026-05-06
updated: 2026-05-06
---

# Adopt Working Directory Runbook Field

## Context

The CLI-level execution root option should use the fully descriptive
`--working-directory` name. The existing runbook field is `working_dir`, which
is shorter and no longer matches the preferred CLI terminology.

## Decision

Track a later compatibility-focused change to move runbook entry syntax toward
`working_directory`. This should be handled separately from the initial
CLI-level `--working-directory` feature so the CLI behavior can land without
forcing a runbook schema migration at the same time.

## Scope

- Define whether `working_directory` replaces `working_dir` immediately or is
  introduced as a compatibility alias first.
- Update the runbook spec for command-level working directory syntax.
- Update validation, rendering, examples, explain output, guides, and tests.
- Decide how `sw format`, `sw convert`, and generated examples should serialize
  the field during any compatibility period.
- Preserve a clear validation error for ambiguous entries that declare both
  `working_dir` and `working_directory`.

## Acceptance Criteria

- [ ] The runbook spec defines the preferred command-level working directory
      field name.
- [ ] Existing runbooks using `working_dir` have a documented migration path.
- [ ] Examples and discovery output use the preferred field name.
- [ ] Validation rejects entries that declare both field names.
- [ ] Automated tests cover the preferred field name and compatibility behavior.
