---
id: TASK-155
title: Support DisplayFile Content Type
status: done
category: display-file
related_features:
  - SPEC-003
owner: @aattard
created: 2026-05-12
updated: 2026-05-12
---

## Summary

Allow `DisplayFile` entries to declare `content_type` so runbooks can control
the generated Markdown fence label when a file extension is missing or
misleading.

## Scope

- Accept optional `DisplayFile.content_type` during validation
- Render `DisplayFile.content_type` ahead of extension-based fence detection
- Continue using extension-based fence detection when `content_type` is absent
- Reject unsupported `DisplayFile.content_type` values
- Update user-facing help, explain, example, and guide text
- Add CLI coverage for validation and rendering behavior

## Assumptions

- `content_type` affects rendering only; it does not change file reading,
  slicing, transforms, indentation, or offset behavior.
- The supported values should stay explicit instead of accepting arbitrary
  fence labels.
- `text` means a fenced block labelled `text`, matching the existing
  unknown-extension fallback for `DisplayFile`.

## Acceptance Criteria

- [x] Given a `DisplayFile` entry with `content_type: java` and a path without
      a recognized extension, validation accepts the runbook.
- [x] Given a `DisplayFile` entry with `content_type: java` and a path without
      a recognized extension, `sw run` renders a `java` fenced block.
- [x] Given a `DisplayFile` entry with `content_type: text` and a `.java` path,
      `sw run` renders a `text` fenced block instead of a `java` fenced block.
- [x] Given a `DisplayFile` entry without `content_type`, extension-based
      detection continues to choose the existing fence label.
- [x] Given an unsupported `DisplayFile.content_type`, validation fails.
- [x] Help, explain, example, and guide text describe the precedence rule.
- [x] Automated tests pass after the change.

## Notes

This keeps `DisplayFile` usable for extensionless files and generated source
snapshots while preserving the existing extension fallback for ordinary files.
