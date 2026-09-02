---
id: TASK-194
title: Publish Homebrew-Synchronized v0.1.3 Release
status: in_progress
category: release
related_features:
  - SPEC-007
owner: albertattard
created: 2026-09-02
updated: 2026-09-02
---

## Summary

Publish v0.1.3 so users receive the current display-excerpt behavior and
release-process guidance through the official GitHub Release and maintained
Homebrew tap.

## Scope

- Bump the package version to 0.1.3.
- Merge a verified release-preparation pull request to `main`.
- Publish tag `v0.1.3` from the merged commit.
- Verify the tagged workflow publishes release archives, checksums, release
  README, and the Homebrew formula update.
- Verify `brew upgrade albertattard/tap/sw` installs 0.1.3.

## Assumptions

- The existing tagged-release workflow remains the only mechanism that writes
  to the Homebrew tap.
- Package version `0.1.3` and release tag `v0.1.3` identify the same release.

## Acceptance Criteria

- [ ] The merged release-preparation commit sets the package version to 0.1.3.
- [ ] GitHub Release `v0.1.3` contains the supported archives, `SHA256SUMS`,
      and generated release README.
- [ ] The tagged workflow updates `albertattard/homebrew-tap` to v0.1.3 using
      the published archive checksums.
- [ ] `brew upgrade albertattard/tap/sw` installs v0.1.3 without Rust.
- [ ] Release and Homebrew verification evidence is recorded in this task.
