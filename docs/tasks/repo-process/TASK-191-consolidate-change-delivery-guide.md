---
id: TASK-191
title: Consolidate Change Delivery Guide
status: done
category: repo-process
related_features:
  - release-distribution
owner: albertattard
created: 2026-09-02
updated: 2026-09-02
---

## Summary

Provide one maintainer-facing guide for delivering a change from a local
working tree through pull request, merge, cleanup, and an optional release.

## Scope

- Consolidate existing commit, pull-request, merge, cleanup, tag, and
  Homebrew-release guidance in `docs/release/README.md`
- Preserve the distinction between merging source changes and publishing a
  versioned distribution
- Link the documented steps to their existing repository tools and safeguards

## Acceptance Criteria

- [x] The release guide explains commit, push, pull request, merge, cleanup,
      and optional release in their execution order.
- [x] The guide states that a normal merged change is not automatically a
      published release.
- [x] The guide explains the required post-tag verification and Homebrew
      upgrade.
- [x] Documentation checks pass.
