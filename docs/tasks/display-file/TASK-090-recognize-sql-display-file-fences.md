---
id: TASK-090
title: Recognize SQL DisplayFile Fences
status: done
category: display-file
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-06
updated: 2026-04-06
---

## Summary

Recognize `.sql` files in `DisplayFile` entries so rendered snippets use a
`sql` fenced block instead of falling back to plain text.

## Scope

- Extend `DisplayFile` fence-language detection to recognize `.sql`
- Preserve existing `DisplayFile` behavior for `.java` and unknown extensions
- Update user-facing help or discovery text for the implemented behavior
- Add integration coverage for SQL `DisplayFile` rendering

## Assumptions

- This change is about Markdown rendering only; it does not add SQL-specific
  transforms or execution behavior.
- Unknown file extensions should continue to fall back to `text`.
- The first SQL increment should stay narrow and only add fence-language
  detection.

## Acceptance Criteria

- [x] Given a `DisplayFile` entry that references a `.sql` file, `sw run`
      renders that snippet in a `sql` fenced block.
- [x] Given a `DisplayFile` entry that references a `.java` file, rendering
      still uses a `java` fenced block.
- [x] Given an unrecognized `DisplayFile` extension, rendering still falls
      back to a `text` fenced block.
- [x] Help or discovery text reflects the recognized SQL fence behavior.
- [x] Automated tests pass after the change.

## Notes

This is a small rendering-contract extension for documentation quality. It is
not intended to expand `DisplayFile.transform` or introduce SQL-aware
rewriting.
