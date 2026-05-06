---
id: TASK-147
title: Adopt Working Directory Runbook Field
status: done
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

Adopt `working_directory` as the preferred command-level runbook field while
keeping `working_dir` as a legacy compatibility alias. Reject entries that
declare both fields so command execution, cleanup, rendering, and file
assertions do not depend on an ambiguous working-directory source.

## Scope

- Introduce `working_directory` as the preferred command-level field.
- Keep `working_dir` as a compatibility alias for existing runbooks.
- Update the runbook spec for command-level working directory syntax.
- Update validation, rendering, examples, guides, and tests.
- Document that `sw format` and `sw convert` preserve the authored compatible
  field name instead of silently migrating `working_dir`.
- Preserve a clear validation error for ambiguous entries that declare both
  `working_dir` and `working_directory`.

## Acceptance Criteria

- [x] The runbook spec defines the preferred command-level working directory
      field name.
- [x] Existing runbooks using `working_dir` have a documented migration path.
- [x] Examples and discovery output use the preferred field name.
- [x] Validation rejects entries that declare both field names.
- [x] Automated tests cover the preferred field name and compatibility behavior.
