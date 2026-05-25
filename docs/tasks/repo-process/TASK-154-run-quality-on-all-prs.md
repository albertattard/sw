---
id: TASK-154
title: Run Quality On All Pull Requests
status: done
category: repo-process
related_features:
  - CI
owner: albertattard
created: 2026-05-11
updated: 2026-05-11
---

## Summary

Make the protected-main build flow reliable by ensuring the required `Quality`
check is produced for every branch push and pull request.

## Scope

- Remove CI path ignores that skip `docs/**`
- Keep one baseline quality job for normal push and pull-request feedback
- Keep full release-platform builds limited to version tags

## Assumptions

- `main` requires the `Quality` status check before updates.
- Required checks are easier to reason about when they are always produced for
  pull requests, including documentation-only changes.
- The cost of running baseline Rust checks for documentation-only changes is
  acceptable for this repository.

## Acceptance Criteria

- [x] `.github/workflows/ci.yml` runs on all branch pushes.
- [x] `.github/workflows/ci.yml` runs on all pull requests.
- [x] `.github/workflows/ci.yml` no longer skips `docs/**`.
- [x] Tagged releases still run the release-platform build matrix after the
      baseline `Quality` job.
