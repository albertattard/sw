---
id: TASK-066
title: Add Patch Example Snippet
status: open
category: example
related_features:
  - SPEC-008
  - SPEC-003
owner: @aattard
created: 2026-03-16
updated: 2026-03-16
---

## Summary

Add a `Patch` topic to the `example` command so users can discover the new
entry shape directly from the CLI instead of guessing from the spec.

## Scope

- Add `Patch` as a supported `sw example` entity type
- Return a valid JSON example for a `Patch` entry
- Reflect the current automatic restore behavior in the example shape
- Add CLI coverage for the new example topic
- Keep entity-type matching case-insensitive for the new topic

## Assumptions

- The example should show the common patch shape without forcing users to
  declare `restore`, since automatic restore is the default behavior.
- A practical patch example is more useful than a minimal shell because users
  can trim fields they do not need.

## Acceptance Criteria

- [ ] Given `sw example Patch`, the CLI prints a valid JSON example of a
      `Patch` entry.
- [ ] Given `sw example patch`, the CLI behaves the same as
      `sw example Patch`.
- [ ] The returned example reflects the documented `Patch` contract, including
      patch lines and a target path.

## Notes

This keeps `sw example` aligned with the current set of first-class runbook
entry types.
