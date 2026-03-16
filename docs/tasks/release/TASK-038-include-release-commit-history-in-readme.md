---
id: TASK-038
title: Include Release Commit History In README
status: pending
category: release
related_features:
  - SPEC-007
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Extend the release pipeline so the generated release `README.md` includes the
commit subject lines between the previous release tag and the current release
tag.

## Scope

- Determine the previous release tag during the release workflow
- Collect commit subject lines between the previous release and the current tag
- Include those commit summaries in the generated release README
- Define sensible behavior when the current release is the first release

## Assumptions

- Release tags use the existing `v*` pattern.
- Commit subjects are sufficient for the initial release-notes summary.
- The release README remains a generated build artifact.

## Acceptance Criteria

- [ ] The release README includes the commit subjects since the previous
      release.
- [ ] The release README handles the first tagged release without failing.
- [ ] The release pipeline still publishes the binary and release README
      successfully.

## Notes

This makes official release artifacts more useful by summarizing the changes in
that release rather than only restating one commit headline.
