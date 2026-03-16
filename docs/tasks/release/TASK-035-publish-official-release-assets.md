---
id: TASK-035
title: Publish Official Release Assets
status: done
category: release
related_features:
  - SPEC-007
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Add a release-distribution workflow that publishes official downloadable
artifacts to GitHub Releases, including both version-specific assets and a
latest-release path.

## Scope

- Add or extend pipeline behavior for tagged releases
- Publish release binaries as GitHub Release assets
- Publish a release README alongside the binary
- Keep CI artifact uploads separate from official release assets
- Align naming and metadata with `SPEC-007`

## Assumptions

- GitHub Releases are the official distribution mechanism.
- GitHub’s latest-release path is sufficient for the initial latest-build
  access pattern.
- Versioned release paths come from standard GitHub Release asset URLs.

## Acceptance Criteria

- [x] A tagged release publishes official assets to GitHub Releases.
- [x] The release includes the binary and release README.
- [x] The pipeline behavior clearly distinguishes CI artifacts from official
      release assets.
- [x] The published output supports both version-specific and latest-release
      access patterns.

## Notes

This task covers official release distribution behavior, not just CI artifact
upload for build verification.
