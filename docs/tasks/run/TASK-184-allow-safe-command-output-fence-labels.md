---
id: TASK-184
title: Allow Safe Command Output Fence Labels
status: done
category: run
related_features:
  - SPEC-003
owner: albertattard
created: 2026-08-31
updated: 2026-08-31
---

## Summary

Allow runbooks to choose safe Markdown fence labels for `Command` output,
including `diff`, without adding one product task for every syntax-highlighting
label.

## Scope

- Accept `diff` and other safe `output.content_type` labels during validation.
- Render a declared label verbatim, except that `text` continues to use an
  unlabeled fence.
- Reject empty labels and labels containing whitespace or fence-breaking
  characters.
- Update the `run` specification, discovery text, and automated coverage.

## Non-goals

- Parsing, validating, or transforming the captured output according to its
  label.
- Expanding the explicit content-type contract for `DisplayFile` or
  `DisplayUrl` entries.

## Acceptance Criteria

- [x] `output.content_type: diff` validates successfully and renders a `diff`
      fenced block.
- [x] A conventional safe label such as `toml` validates without a code change.
- [x] `text` continues to render an unlabeled fenced block.
- [x] Empty, whitespace-containing, and fence-breaking labels are rejected.

## Verification

- `./tools/verify.sh`
