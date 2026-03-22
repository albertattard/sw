---
id: TASK-077
title: Document DisplayFile Transform Discovery In Explain
status: done
category: explain
related_features:
  - SPEC-009
  - SPEC-011
owner: @aattard
created: 2026-03-22
updated: 2026-03-22
---

## Summary

Extend `sw explain example` so users and agents can discover that the
`DisplayFile` example includes the Java `collapse_method_body` transform for
collapsing method bodies.

## Scope

- Add DisplayFile transform guidance to `sw explain example`
- Ensure skill export inherits that guidance
- Add explain-focused CLI coverage for the new discovery content

## Assumptions

- `explain` is the right place to describe what an example topic is good for
- Skill output should remain a derived view of the explain knowledge model

## Acceptance Criteria

- [x] Given `sw explain example`, the CLI notes that `sw example DisplayFile`
      includes the Java `collapse_method_body` transform.
- [x] Given `sw explain --output-format=skill`, the generated skill content
      preserves that guidance.
- [x] Explain-focused automated tests cover the updated guidance.

## Notes

This improves CLI discoverability without changing the underlying DisplayFile
transform contract.
