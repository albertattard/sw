---
id: TASK-038
title: Include Release Commit History In README
status: done
category: release
related_features:
  - SPEC-007
owner: @aattard
created: 2026-03-13
updated: 2026-04-15
---

## Summary

Extend the release pipeline so the generated release `README.md` includes the
commit subject lines between the nearest previous reachable `v*` tag and the
current release tag.

## Scope

- Determine the nearest previous reachable `v*` tag during the release workflow
- Collect commit subject lines between the previous release and the current tag
- Include those commit summaries in the generated release README

## Assumptions

- Release tags use the existing `v*` pattern.
- "Previous release" means the nearest earlier reachable `v*` tag in the
  current commit history, not the most recently created tag in the repository.
- Commit subjects are sufficient for the initial release-notes summary.
- The release README remains a generated build artifact.
- First-release README behavior is handled separately from this task.

## Acceptance Criteria

- [x] The release README includes the commit subjects since the previous
      release.
- [x] If multiple `v*` tags exist in the repository, the release README uses
      the nearest previous reachable `v*` tag rather than an unrelated newer
      tag elsewhere in the graph.
- [x] The release pipeline still publishes the binary and release README
      successfully.

## Notes

This makes official release artifacts more useful by summarizing the changes in
that release rather than only restating one commit headline.
