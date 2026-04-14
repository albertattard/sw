---
id: TASK-107
title: Add Supported Features To First Release README
status: done
category: release
related_features:
  - SPEC-007
owner: @aattard
created: 2026-04-15
updated: 2026-04-15
---

## Summary

Add a curated supported-features section to the generated release `README.md`
when a tagged release has no previous reachable `v*` tag.

## Scope

- Detect when the current release is the first published release
- Source first-release feature content from a maintained repository file
- Include that curated feature summary in the generated release `README.md`
- Avoid dumping the full raw git history for the first release

## Assumptions

- A curated feature summary is more useful than a full commit list for the
  first release.
- The maintained source file lives in the repository and can be reviewed like
  other release-facing content.
- Later releases continue to use release-to-release commit history instead.

## Acceptance Criteria

- [x] If no previous reachable `v*` tag exists, the release README includes a
      curated supported-features section.
- [x] The first-release feature summary is sourced from a maintained repository
      file rather than being generated ad hoc in the workflow.
- [x] The release pipeline still publishes the binary assets and release README
      successfully for the first release.

## Notes

This keeps the first public release focused on what the product supports rather
than on the internal implementation sequence that led to that release.
