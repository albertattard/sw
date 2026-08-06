---
id: TASK-172
title: Avoid Duplicate Feature Branch CI
status: done
category: repo-process
related_features:
  - repository automation
owner: albertattard
created: 2026-08-06
updated: 2026-08-06
---

## Goal

Avoid running equivalent quality pipelines for both a feature-branch push and
its pull request while retaining validation for pull requests, `main`, release
tags, and scheduled dependency checks.

## Scope

- Limit branch-push CI and dependency-hygiene runs to `main`.
- Limit pull-request runs to pull requests targeting `main`.
- Retain CI runs for version tags.
- Keep scheduled and manually dispatched dependency-hygiene runs.
- Give the CI workflow read-only content access by default and grant write
  access only to the release-publishing job.

## Acceptance Criteria

- [x] Feature-branch pushes do not start the CI or dependency-hygiene workflows.
- [x] Pull requests targeting `main` run the applicable checks once.
- [x] Pushes to `main` and version tags retain CI coverage.
- [x] Release publishing retains the permission required to create releases.
- [x] Workflow files remain structurally valid and repository verification
      passes.
