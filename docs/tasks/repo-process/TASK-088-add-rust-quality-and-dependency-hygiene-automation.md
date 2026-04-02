---
id: TASK-088
title: Add Rust Quality And Dependency Hygiene Automation
status: done
category: repo-process
related_features: []
owner: @aattard
created: 2026-04-02
updated: 2026-04-02
---

## Summary

Pin the Rust toolchain and add automated dependency hygiene checks so local
development and CI enforce the same baseline quality expectations.

## Scope

- Pin the repository Rust toolchain and declare the supported compiler version
- Keep CI aligned with the pinned toolchain instead of a floating `stable`
- Add automated dependency advisory checks for Cargo dependencies
- Add scheduled dependency update automation for Cargo crates and GitHub Actions
- Document the new local and CI dependency hygiene workflow

## Assumptions

- A pinned toolchain is preferable to a floating stable channel for a CLI that
  depends on consistent formatter and Clippy behavior.
- Weekly dependency update review is frequent enough for this repository’s
  current size and release cadence.
- Advisory and duplicate-version checks are the highest-value first dependency
  hygiene gate for the current dependency graph.

## Acceptance Criteria

- [x] The repository declares a pinned Rust toolchain for local development.
- [x] `Cargo.toml` declares the supported Rust compiler version.
- [x] CI installs and uses the pinned Rust toolchain.
- [x] A CI workflow runs `cargo-deny` against dependency advisories and bans.
- [x] Dependabot is configured to review Cargo crates and GitHub Actions weekly.
- [x] The README documents the pinned toolchain and local dependency hygiene
      command.

## Notes

This task improves repository process and supply chain visibility. It does not
yet replace deprecated dependencies or raise the Clippy policy beyond the
existing `-D warnings` baseline.
