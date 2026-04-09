---
id: TASK-102
title: Review actions checkout v6 Upgrade
status: done
category: repo-process
related_features: []
owner: @aattard
created: 2026-04-09
updated: 2026-04-09
---

## Summary

Apply the pending `actions/checkout` major-version upgrade on top of current
`main` after reviewing the official action guidance for compatibility with the
repository's current workflow usage.

## Scope

- Review the `actions/checkout` `v4` to `v6` upgrade for the repository's
  current GitHub Actions usage
- Reapply the minimal workflow changes on top of current `main`
- Verify the updated workflows remain consistent with the repository's pinned
  Rust toolchain and release process

## Assumptions

- A major GitHub Actions dependency upgrade should be reviewed separately from
  product code and Rust crate updates.
- Recreating the change on top of current `main` is easier to review than
  merging an older Dependabot branch with unrelated drift.
- This increment is repository-process work and should not change the local
  development workflow unless explicitly intended.

## Acceptance Criteria

- [x] The repository has a documented decision on whether to adopt
      `actions/checkout@v6`.
- [x] If adopted, the relevant workflow files are updated on top of current
      `main`.
- [x] The change is verified in a way appropriate for workflow-only updates.

## Notes

This task is intentionally scoped to `actions/checkout`. The
`actions/upload-artifact` major-version bump remains a separate slice. The
review concluded that this repository's workflows use standard checkout steps on
GitHub-hosted runners, so the documented `v6` runner requirement for Docker
container action credential persistence does not add an extra repository change
here.
