---
id: TASK-209
title: Upgrade Getrandom Dependency
status: done
category: repo-process
related_features:
  - SPEC-013
owner: albertattard
created: 2026-11-17
updated: 2026-11-17
---

## Summary

Upgrade the direct getrandom dependency to 0.4.3 and migrate the generated
secrets implementation to its current API. PR #85 fails to compile because
it updates the dependency without updating the caller.

## Acceptance Criteria

- [x] Use getrandom 0.4.3 and its `fill` API for secure secret generation.
- [x] Preserve generated secret behavior and random-source error propagation.
- [x] `./tools/verify.sh` passes.

## Verification

- `CARGO_HOME=/private/tmp/sw-pr85-cargo ./tools/verify.sh` passed.
