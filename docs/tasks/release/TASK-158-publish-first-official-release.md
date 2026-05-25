---
id: TASK-158
title: Publish And Validate Official Release Distribution
status: pending
category: release
related_features:
  - SPEC-007
owner: albertattard
created: 2026-05-25
updated: 2026-05-25
---

## Summary

Publish and verify the first official GitHub Release, and audit the release
workflow against the remaining `SPEC-007` acceptance criteria so the release
distribution contract is satisfied for users, not only represented by workflow
configuration.

## Scope

- Create a `v*` release tag from the intended release commit.
- Let the tagged release workflow publish the official release assets.
- Verify that the release includes supported platform binaries and the release
  README.
- Verify that normal push and pull-request CI still runs baseline quality
  checks without building the full release-platform matrix.
- Verify that non-initial release README generation is ready to include commit
  subjects between the nearest previous reachable `v*` tag and the current
  release tag.
- Verify that the versioned release asset URLs work.
- Verify that GitHub's latest release path points users to the newest published
  release.
- Keep transient CI artifacts distinct from official release assets.

## Assumptions

- The release workflow already defines the supported platform matrix and
  generated release README behavior.
- This task is about cutting and validating the first public release, not
  redesigning the release workflow.
- If the release workflow fails, the fix should be captured in a narrower
  follow-up task tied to the failing release behavior.

## Acceptance Criteria

- [ ] A `v*` tag publishes an official GitHub Release.
- [ ] The published release includes the supported platform binary assets.
- [ ] The published release includes the generated release `README.md`.
- [ ] Normal push and pull-request CI runs baseline quality checks without
      building the full release-platform matrix.
- [ ] Non-initial release README generation is verified to use commit subjects
      between the nearest previous reachable `v*` tag and the current release
      tag.
- [ ] The generated first-release README includes the curated supported-features
      section because no previous reachable `v*` tag exists.
- [ ] A user can download a specific version from a versioned release asset URL.
- [ ] GitHub's latest release path points to the newest published release.
- [ ] The official release mechanism remains documented separately from
      transient CI artifacts.

## Notes

The release workflow and release README generation tasks are already tracked
separately. This task keeps the remaining acceptance gap focused on publishing
and validating the release distribution contract end to end.
