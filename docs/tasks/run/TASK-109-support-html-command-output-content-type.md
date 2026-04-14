---
id: TASK-109
title: Support HTML Command Output Content Type
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-15
updated: 2026-04-15
---

## Summary

Allow `Command.output.content_type` to use `html` so generated Markdown can
render captured HTML output with an `html` fenced block.

## Scope

- Accept `output.content_type: html` during validation
- Render captured command output using an `html` fenced block
- Preserve existing `text`, `json`, `xml`, and `java` behavior
- Update user-facing discovery text for the expanded supported set
- Add automated validation and rendering coverage

## Assumptions

- `html` content type is intended for syntax highlighting only.
- This change affects rendering and validation, not command execution or
  assertions.
- Additional content types should continue to land as separate tasks.

## Acceptance Criteria

- [x] Given `output.content_type: html`, validation accepts the runbook.
- [x] Given a `Command` entry with `output.content_type: html`, the generated
      Markdown uses an `html` fenced block for captured output.
- [x] Given existing `text`, `json`, `xml`, and `java` content types, behavior
      remains unchanged.
- [x] Unsupported `output.content_type` values other than the supported set
      are still rejected.

## Notes

This is a narrow extension of the command-output rendering contract for
HTML-producing examples such as HTTP responses and generated snippets.
