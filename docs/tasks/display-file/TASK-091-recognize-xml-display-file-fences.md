---
id: TASK-091
title: Recognize XML DisplayFile Fences
status: pending
category: display-file
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-07
updated: 2026-04-07
---

## Summary

Recognize `.xml` files in `DisplayFile` entries so rendered snippets use an
`xml` fenced block instead of falling back to plain text.

## Scope

- Extend `DisplayFile` fence-language detection to recognize `.xml`
- Preserve existing `DisplayFile` behavior for `.java`, `.sql`, and unknown
  extensions
- Update user-facing help or discovery text for the implemented behavior
- Add integration coverage for XML `DisplayFile` rendering

## Assumptions

- This change is about Markdown rendering only; it does not add XML-specific
  transforms, validation, or execution behavior.
- Unknown file extensions should continue to fall back to `text`.
- This increment should stay narrow and recognize `.xml` only, which covers
  files such as `pom.xml` without expanding to other XML-adjacent extensions.

## Acceptance Criteria

- [ ] Given a `DisplayFile` entry that references a `.xml` file, `sw run`
      renders that snippet in an `xml` fenced block.
- [ ] Given a `DisplayFile` entry that references a `.java` file, rendering
      still uses a `java` fenced block.
- [ ] Given a `DisplayFile` entry that references a `.sql` file, rendering
      still uses a `sql` fenced block.
- [ ] Given an unrecognized `DisplayFile` extension, rendering still falls
      back to a `text` fenced block.
- [ ] Help or discovery text reflects the recognized XML fence behavior.
- [ ] Automated tests pass after the change.

## Notes

This is a small rendering-contract extension for documentation quality. It is
not intended to expand `DisplayFile.transform` or introduce XML-aware
rewriting.
