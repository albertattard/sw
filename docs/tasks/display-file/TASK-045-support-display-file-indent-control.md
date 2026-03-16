---
id: TASK-045
title: Support DisplayFile Indent Control
status: done
category: display-file
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Allow `DisplayFile` entries to adjust rendered indentation so extracted code can
either nest more deeply or remove surrounding indentation from a sliced block.

## Scope

- Support optional `indent` on `DisplayFile`
- Accept positive and negative integer values
- Add spaces to each non-empty rendered line when `indent` is positive
- Remove up to the requested number of leading spaces from each non-empty
  rendered line when `indent` is negative
- Preserve blank lines when `indent` is applied
- Add validation and integration coverage for the new behavior

## Assumptions

- `DisplayFile.indent` adjusts the copied file contents inside the fenced block,
  not the Markdown fence itself.
- Negative indentation removes spaces only; it does not strip tabs or other
  leading characters.

## Acceptance Criteria

- [x] Given a `DisplayFile` entry with positive `indent`, the rendered snippet
      adds that many leading spaces to each non-empty line.
- [x] Given a `DisplayFile` entry with negative `indent`, the rendered snippet
      removes up to that many leading spaces from each non-empty line.
- [x] Given a `DisplayFile` entry with blank lines and `indent`, blank lines
      remain unchanged.
- [x] Given a `DisplayFile` entry with non-integer `indent`, validation fails.

## Notes

This keeps `DisplayFile` useful for both full-file excerpts and extracted
methods, where the original surrounding indentation can make the rendered
snippet harder to read.
