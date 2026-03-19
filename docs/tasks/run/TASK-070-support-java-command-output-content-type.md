---
id: TASK-070
title: Support Java Command Output Content Type
status: pending
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-18
updated: 2026-03-18
---

## Summary

Allow `Command.output.content_type` to use `java` so generated Markdown can
render captured Java output in a `java` fenced block.

## Scope

- Accept `output.content_type: java` during validation
- Render captured command output with a `java` fenced block when requested
- Preserve existing unlabeled rendering for omitted `content_type` and for
  `text`
- Keep `content_type` as a rendering concern only
- Add integration coverage for validation and Markdown rendering

## Assumptions

- `java` content type is intended for syntax highlighting only
- This change does not affect command execution, assertions, or output capture
- Additional content types should continue to land as separate tasks

## Acceptance Criteria

- [ ] Given `output.content_type: java`, validation accepts the runbook.
- [ ] Given a `Command` entry with `output.content_type: java`, the generated
      Markdown uses a `java` fenced block for captured output.
- [ ] Given existing `text`, `json`, and `xml` content types, behavior remains
      unchanged.
- [ ] Unsupported `output.content_type` values other than the supported set
      are still rejected by validation.

## Notes

This task extends the existing output content type contract rather than
changing the structure of `output`.
