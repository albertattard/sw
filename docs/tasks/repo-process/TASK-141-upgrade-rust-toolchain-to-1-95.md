---
id: TASK-141
title: Upgrade Rust Toolchain To 1.95
status: done
category: repo-process
related_features: []
owner: @aattard
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Raise the repository's pinned Rust toolchain from `1.94.1` to `1.95.0` and
keep local development, CI, dependency hygiene, and documentation aligned.

## Scope

- Update the pinned Rust toolchain used by local development.
- Update CI and dependency hygiene workflows to install the same Rust version.
- Refresh compatible Cargo lockfile dependencies while performing the toolchain
  maintenance.
- Update documentation that names the pinned Rust toolchain.
- Verify the repository with the standard local quality workflow.

## Assumptions

- Keeping a pinned patch version remains preferable to a floating stable
  channel because it keeps formatter and Clippy behavior reproducible.
- This is repository-process maintenance and should not change CLI behavior.
- Compatible transitive dependency updates are appropriate while refreshing the
  toolchain, provided the standard verification workflow remains green.

## Acceptance Criteria

- [x] `rust-toolchain.toml` pins Rust `1.95.0`.
- [x] `Cargo.toml` declares Rust `1.95` as the supported compiler version.
- [x] CI and dependency hygiene workflows install Rust `1.95.0`.
- [x] Cargo lockfile dependencies are refreshed to current compatible versions.
- [x] Documentation that names the pinned Rust toolchain refers to `1.95.0`.
- [x] `./tools/verify.sh` passes after the upgrade.

## Notes

This task does not change the public CLI contract. It keeps repository
maintenance automation current with the supported Rust baseline.
