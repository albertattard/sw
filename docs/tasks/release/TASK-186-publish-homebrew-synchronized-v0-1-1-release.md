---
id: TASK-186
title: Publish Homebrew-Synchronized v0.1.1 Release
status: in_progress
category: release
related_features:
  - SPEC-007
owner: albertattard
created: 2026-08-31
updated: 2026-08-31
---

## Summary

Publish v0.1.1 so users can install the safe command-output fence-label
support through the maintained Homebrew tap rather than relying on a local
development build.

## Scope

- Bump the package version to 0.1.1.
- Publish a `v0.1.1` tag from merged `main`.
- Verify the tagged-release workflow publishes the two supported archives,
  checksums, release README, and tap formula update.
- Verify Homebrew upgrades to v0.1.1 on Apple Silicon macOS.

## Acceptance Criteria

- [ ] `sw version` reports 0.1.1 from the tagged release archive.
- [ ] GitHub Release `v0.1.1` contains the declared archives, `SHA256SUMS`,
      and generated release README.
- [ ] The tagged workflow updates `albertattard/homebrew-tap` to v0.1.1 using
      the published archive checksums.
- [ ] `brew upgrade albertattard/tap/sw` installs v0.1.1 without Rust.

## Verification

- Run `./tools/verify.sh` before creating the version-bump commit.
- Inspect the tagged release workflow and tap update after the tag is pushed.
- Run `brew update`, `brew upgrade albertattard/tap/sw`, and `sw version`.
