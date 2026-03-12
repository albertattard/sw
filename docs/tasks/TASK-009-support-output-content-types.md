---
id: TASK-009
title: Support Output Content Types
status: done
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Support `output.content_type` for rendered command output in `sw run`.

## Scope

- Allow `output.content_type` on `Command` entries
- Default rendered output blocks to `text` when `content_type` is omitted
- Support `text`, `json`, and `xml`
- Render the fenced Markdown block using the declared content type
- Reject unsupported content type values during validation

## Assumptions

- `content_type` affects rendering only, not command execution or assertions.
- Additional content types may be added later in separate tasks.

## Acceptance Criteria

- [x] Given `output.content_type: json`, captured output is rendered in a `json` fenced block.
- [x] Given `output.content_type: xml`, captured output is rendered in an `xml` fenced block.
- [x] Given `output` without `content_type`, captured output is rendered in a `text` fenced block.
- [x] Unsupported `output.content_type` values are rejected by validation.

## Notes

This task improves readability of generated output for structured command
responses without changing execution behavior.
