---
id: TASK-149
title: Recognize Markdown DisplayFile Fences
status: done
category: display-file
related_features:
  - SPEC-003
owner: @aattard
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Recognize Markdown files in `DisplayFile` entries so rendered snippets use a
`markdown` fenced block instead of falling back to plain text.

## Scope

- Extend `DisplayFile` fence-language detection to recognize `.md`
- Extend `DisplayFile` fence-language detection to recognize `.markdown`
- Preserve existing `DisplayFile` behavior for `.java`, `.sql`, `.xml`, and
  unknown extensions
- Update user-facing help, explain, and guide text for the implemented behavior
- Add integration coverage for Markdown `DisplayFile` rendering

## Assumptions

- This change is about Markdown rendering only; it does not add
  Markdown-specific transforms, validation, or execution behavior.
- Unknown file extensions should continue to fall back to `text`.
- Supporting `.markdown` alongside `.md` keeps the behavior aligned with common
  Markdown file naming without adding a broader MIME detection system.

## Acceptance Criteria

- [x] Given a `DisplayFile` entry that references a `.md` file, `sw run`
      renders that snippet in a `markdown` fenced block.
- [x] Given a `DisplayFile` entry that references a `.markdown` file, the
      renderer recognizes the same `markdown` fence label.
- [x] Given a `DisplayFile` entry that references `.java`, `.sql`, or `.xml`,
      existing fence labels are preserved.
- [x] Given an unrecognized `DisplayFile` extension, rendering still falls
      back to a `text` fenced block.
- [x] Help, explain, and guide text reflect the recognized Markdown fence
      behavior.
- [x] Automated tests pass after the change.

## Notes

This addresses Markdown-oriented files such as `SKILL.md`, where rendering as
plain `text` loses useful syntax highlighting in generated documentation.
