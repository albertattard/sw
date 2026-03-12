---
id: TASK-014
title: Use Unlabeled Fences For Plain Output
status: pending
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Render plain captured command output in unlabeled fenced blocks instead of
using the explicit `text` language tag.

## Scope

- Use unlabeled fenced blocks when `output.content_type` is omitted
- Use unlabeled fenced blocks when `output.content_type` is `text`
- Preserve existing labeled fenced blocks for `json` and `xml`

## Assumptions

- This change affects generated Markdown only and does not change command
- execution, validation, or output trimming behavior.

## Acceptance Criteria

- [ ] Given a `Command` entry with `output` and no `content_type`, the
      generated Markdown uses an unlabeled fenced block for captured output.
- [ ] Given a `Command` entry with `output.content_type: text`, the generated
      Markdown uses an unlabeled fenced block for captured output.
- [ ] Given a `Command` entry with `output.content_type: json`, the generated
      Markdown continues to use a `json` fenced block.
- [ ] Given a `Command` entry with `output.content_type: xml`, the generated
      Markdown continues to use an `xml` fenced block.

## Notes

This task makes plain output read more naturally in Markdown while preserving
explicit syntax highlighting for structured output types.
