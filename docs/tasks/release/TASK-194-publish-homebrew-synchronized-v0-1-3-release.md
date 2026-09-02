---
id: TASK-194
title: Publish Homebrew-Synchronized v0.1.3 Release
status: done
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

- [x] The merged release-preparation commit sets the package version to 0.1.3.
- [x] GitHub Release `v0.1.3` contains the supported archives, `SHA256SUMS`,
      and generated release README.
- [x] The tagged workflow updates `albertattard/homebrew-tap` to v0.1.3 using
      the published archive checksums.
- [x] `brew upgrade albertattard/tap/sw` installs v0.1.3 without Rust.
- [x] Release and Homebrew verification evidence is recorded in this task.

## Completion Notes

- Merged release-preparation PR #66 at commit `edb051c`, with package version
  0.1.3, then created and pushed tag `v0.1.3` from that same commit.
- Tagged workflow [33598433176](https://github.com/albertattard/sw/actions/runs/33598433176)
  completed successfully: Quality, both platform archive builds, GitHub Release
  publication, and Homebrew tap synchronization all passed.
- GitHub Release `v0.1.3` published the Apple Silicon archive, portable Linux
  archive, `SHA256SUMS`, and generated release README.
- The synchronized formula commit `4646d4f` references the v0.1.3 archive URLs
  and their published SHA-256 values.
- On Apple Silicon macOS, `brew update` followed by
  `brew upgrade albertattard/tap/sw` upgraded `sw` from 0.1.2 to 0.1.3;
  `sw version` reported `sw 0.1.3 (edb051c)`.
