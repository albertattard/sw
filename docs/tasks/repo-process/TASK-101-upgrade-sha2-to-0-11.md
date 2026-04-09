---
id: TASK-101
title: Upgrade sha2 To 0.11
status: done
category: repo-process
related_features: []
owner: @aattard
created: 2026-04-09
updated: 2026-04-09
---

## Summary

Replace the stale Dependabot `sha2` upgrade branch with a fresh dependency
update on current `main` so the repository uses the latest proposed hashing
crate version without pulling in unrelated old branch history.

## Scope

- Bump the direct `sha2` dependency from `0.10` to `0.11`
- Refresh `Cargo.lock` to the resolved dependency graph for that upgrade
- Verify the repository still passes the local Rust quality workflow after the
  upgrade

## Assumptions

- The project's current `sha2` usage is limited to the stable digest API and is
  expected to remain compatible with the `0.11` release.
- Reapplying the dependency bump on top of current `main` is safer than merging
  the stale Dependabot branch directly.
- This increment is dependency maintenance only and does not change user-facing
  behavior.

## Acceptance Criteria

- [x] `Cargo.toml` declares `sha2 = "0.11"`.
- [x] `Cargo.lock` is updated for the `sha2 0.11` dependency graph.
- [x] `./tools/verify.sh` passes after the upgrade.

## Notes

This task replaces the stale remote Dependabot proposal with a verified update
applied against the current branch head.
