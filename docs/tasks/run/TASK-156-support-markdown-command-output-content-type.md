---
id: TASK-156
title: Support Markdown Command Output Content Type
status: done
category: run
related_features:
  - SPEC-003
owner: albertattard
created: 2026-05-13
updated: 2026-05-13
---

## Summary

Allow `Command.output.content_type` to use `markdown` so generated Markdown can
render captured Markdown output with a `markdown` fenced block.

## Scope

- Accept `output.content_type: markdown` during validation
- Render captured command output using a `markdown` fenced block
- Preserve existing `text`, `json`, `xml`, `html`, and `java` behavior
- Keep `content_type` as a fenced-code rendering label only
- Update user-facing discovery text for the expanded supported set
- Add automated validation and rendering coverage

## Assumptions

- `markdown` content type is intended for syntax highlighting only.
- Command output remains fenced and is not interpreted as generated document
  structure.
- This change affects rendering and validation, not command execution,
  assertions, captures, or rewrites.

## Acceptance Criteria

- [x] Given `output.content_type: markdown`, validation accepts the runbook.
- [x] Given a `Command` entry with `output.content_type: markdown`, the
      generated Markdown uses a `markdown` fenced block for captured output.
- [x] Given existing `text`, `json`, `xml`, `html`, and `java` content types,
      behavior remains unchanged.
- [x] Unsupported `output.content_type` values other than the supported set
      are still rejected.

## Notes

This is a narrow extension of the command-output rendering contract. It does
not add an interpreted Markdown output mode.
