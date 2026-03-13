---
id: TASK-036
title: Include Commit Subject In Release README
status: done
related_features:
  - SPEC-007
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Extend the release pipeline so the generated release `README.md` includes the
latest commit subject line as part of the published build metadata.

## Scope

- Capture the latest commit subject line during the release workflow
- Include that subject line in the generated release README
- Keep the rest of the release asset behavior unchanged

## Assumptions

- The latest commit subject line is the first line of `git log -1`.
- The release README remains a generated build artifact.

## Acceptance Criteria

- [x] The release README includes the latest commit subject line.
- [x] The release pipeline still publishes the binary and release README
      successfully.

## Notes

This keeps the downloaded artifact more self-describing by showing the human
summary of the commit that produced the build.
