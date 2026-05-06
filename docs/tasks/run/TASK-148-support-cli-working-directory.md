---
id: TASK-148
title: Support CLI Working Directory
status: done
category: run
related_features:
  - SPEC-002
  - SPEC-003
  - SPEC-005
owner: codex
created: 2026-05-06
updated: 2026-05-06
---

# Support CLI Working Directory

## Context

Runbooks can live in one directory while the project or exercise they operate
on lives in another. Today, runbook-relative paths resolve from the runbook
file's directory, which forces authors to either place the runbook in the
execution target or manually `cd` inside command bodies.

## Decision

Add `--working-directory` to runbook-consuming commands so callers can select
an execution root independently from the runbook file location. CLI file paths
such as `--input-file` and `--output-file` remain relative to the shell current
directory. Runbook-relative paths resolve from the selected execution root.

## Scope

- Add `--working-directory <path>` to `run`, implicit `sw`, `check`, and
  `validate`.
- Resolve relative `--working-directory` values from the shell current
  directory.
- Fail when `--working-directory` does not exist or is not a directory.
- Keep `--input-file` and `--output-file` path resolution relative to the shell
  current directory.
- Resolve runbook-relative paths from the effective execution root, including
  `DisplayFile.path`, `Patch.path`, command execution, command cleanup, command
  file assertions, and command-level `working_dir`.
- Keep command-level `working_dir` relative and constrained inside the
  effective execution root.

## Acceptance Criteria

- [x] `sw run --working-directory <dir>` executes commands from the selected
      directory when entries omit `working_dir`.
- [x] `sw run --working-directory <dir>` resolves `DisplayFile.path` and
      `Patch.path` from the selected directory.
- [x] `sw run --working-directory <dir>` resolves command-level
      `working_dir` relative to the selected directory.
- [x] `sw check --working-directory <dir>` uses the selected directory for
      command-based prerequisite checks.
- [x] `sw validate --working-directory <dir>` validates command-level
      `working_dir` relative to the selected directory.
- [x] Missing or non-directory `--working-directory` values fail clearly.
- [x] CLI help and `sw explain` document the option.
