---
id: TASK-106
title: Move Release Matrix To Tagged Releases
status: done
category: release
related_features:
  - SPEC-007
owner: @aattard
created: 2026-04-14
updated: 2026-04-14
---

## Summary

Restructure the GitHub Actions pipeline so normal CI runs baseline quality
checks on one platform, while tagged releases build and publish the full
supported-platform release matrix.

## Scope

- Keep push and pull-request quality checks on a baseline platform
- Remove full release-platform packaging work from normal CI runs
- Build official release assets for each supported release platform only when a
  `v*` tag is built
- Keep the generated release `README.md` attached to the tagged GitHub Release
- Preserve the distinction between transient CI artifacts and official release
  assets

## Assumptions

- Baseline CI on one platform is sufficient for normal repository feedback.
- Cross-platform release builds are still required before publishing an
  official release.
- This increment changes workflow structure, not the CLI's runtime behavior.

## Acceptance Criteria

- [x] Push and pull-request runs execute the quality workflow on a baseline
      platform without building the full release-platform matrix.
- [x] Tagged `v*` releases build official binaries for every supported release
      platform.
- [x] Tagged releases still publish the official release `README.md`.
- [x] CI artifacts remain transient workflow diagnostics rather than the
      official download mechanism.

## Notes

This task is about placing the matrix at the correct stage of the pipeline.
It does not expand the documented release README contents beyond the existing
release tasks.
