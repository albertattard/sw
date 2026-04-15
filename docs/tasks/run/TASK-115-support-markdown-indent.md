---
id: TASK-115
title: Support Markdown Indent
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-15
updated: 2026-04-15
---

## Summary

Allow `Markdown` entries to declare `indent` so authors can nest rendered prose
inside surrounding Markdown structures without encoding layout spaces directly
inside `contents`.

## Scope

- Add `Markdown.indent` to the runbook rendering contract
- Validate `Markdown.indent` as a non-negative integer
- Apply the indent to each non-empty rendered Markdown line
- Keep empty lines empty so Markdown paragraph boundaries remain intact
- Update help and explain discovery text for the run contract
- Add automated coverage for rendering, validation, and help/explain output

## Assumptions

- `indent` is a rendering concern, not a content transformation
- Interpolation should happen before indentation so captured values inherit the
  same rendered layout as literal content
- `Markdown.indent` should behave consistently with the existing render-time
  `indent` support on `DisplayFile`, `Patch`, and `Command`

## Acceptance Criteria

- [x] Given a `Markdown` entry with `indent`, each non-empty rendered Markdown
      line is prefixed with that many spaces.
- [x] Given a `Markdown` entry with `indent` and blank lines, blank lines
      remain empty in the rendered output.
- [x] Given a negative `Markdown.indent`, validation fails with a clear error
      on `entries[N].indent`.
- [x] Run help and explain output mention `Markdown.indent` alongside the
      existing render-time indent guidance.

## Notes

This change adds a first-class layout field. It is preferable to baking
leading spaces directly into `Markdown.contents`, which couples prose content to
rendering layout and makes later editing more brittle.
