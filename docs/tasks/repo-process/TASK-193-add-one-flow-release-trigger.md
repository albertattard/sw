---
id: TASK-193
title: Add One Flow Release Trigger
status: done
category: repo-process
related_features:
  - release-distribution
owner: albertattard
created: 2026-09-02
updated: 2026-09-02
---

## Summary

Allow an explicit agent request to deliver current changes through merge,
tagged release, and Homebrew tap synchronization as one guarded workflow.

## Scope

- Define the `release changes as v<version>` authorization phrase
- Require an explicit target version and a matching `Cargo.toml` version
- Document the required PR, merge, tag, release, tap, and Homebrew checks
- Stop the flow on failed verification, CI, release, tap synchronization, or
  install verification

## Acceptance Criteria

- [x] `AGENTS.md` states the explicit authorization boundary for the new
      trigger.
- [x] The release guide documents the full trigger flow and its stop
      conditions.
- [x] The guide distinguishes the release version from its `v`-prefixed tag.
- [x] Documentation checks pass.
