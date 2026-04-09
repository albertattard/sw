---
id: TASK-103
title: Review upload artifact v7 Upgrade
status: done
category: repo-process
related_features: []
owner: @aattard
created: 2026-04-09
updated: 2026-04-09
---

## Summary

Apply the pending `actions/upload-artifact` major-version upgrade on top of
current `main` after reviewing the official action guidance for compatibility
with the repository's current artifact usage.

## Scope

- Review the `actions/upload-artifact` `v4` to `v7` upgrade for the
  repository's current GitHub Actions usage
- Reapply the minimal workflow change on top of current `main`
- Verify the updated workflow remains consistent with the repository's release
  artifact behavior

## Assumptions

- A major GitHub Actions dependency upgrade should be reviewed separately from
  product code and Rust crate updates.
- Recreating the change on top of current `main` is easier to review than
  merging an older Dependabot branch with unrelated drift.
- This increment is repository-process work and should not change the local
  development workflow unless explicitly intended.

## Acceptance Criteria

- [x] The repository has a documented decision on whether to adopt
      `actions/upload-artifact@v7`.
- [x] If adopted, the relevant workflow file is updated on top of current
      `main`.
- [x] The change is verified in a way appropriate for workflow-only updates.

## Notes

The review concluded that this repository uploads one uniquely named `dist/`
artifact from a single job, so the documented newer-version restrictions around
re-uploading the same artifact name do not require an extra repository change
here. Hidden-file exclusion is also not material for this `dist/` upload.
