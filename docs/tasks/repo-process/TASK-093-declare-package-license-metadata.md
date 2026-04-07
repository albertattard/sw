---
id: TASK-093
title: Declare Package License Metadata
status: done
category: repo-process
related_features:
  - SPEC-005
owner: @aattard
created: 2026-04-07
updated: 2026-04-07
---

## Summary

Declare the package license explicitly in `Cargo.toml` so tooling can read the
project's MIT license without inferring it from the repository layout.

## Scope

- Add the SPDX license expression for this package in `Cargo.toml`
- Preserve the existing `LICENSE` file
- Ensure dependency hygiene tooling no longer warns that the package manifest
  is missing a license field

## Assumptions

- This change affects package metadata only; it does not change the project's
  actual license terms.
- The project's intended package license is MIT.
- This increment is limited to the root crate metadata and does not change
  third-party dependency policy.

## Acceptance Criteria

- [x] The root package manifest declares `license = "MIT"`.
- [x] `cargo deny check` no longer warns that the package is missing a license
      field.
- [x] Dependency hygiene checks remain green after the change.

## Notes

This is a metadata cleanup slice for tooling and publishing readiness. The
existing `LICENSE` file remains the authoritative license text in the
repository.
