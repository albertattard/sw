---
id: TASK-167
title: Surface DisplayFile Offset In Example Output
status: done
category: example
related_features:
  - SPEC-008
owner: albertattard
created: 2026-06-15
updated: 2026-06-15
---

## Summary

Update `sw example DisplayFile` so the printed snippet includes the supported
`offset` field, making copied-content de-indentation discoverable from the
copyable example.

## Scope

- Add `offset` to the `DisplayFile` example snippet
- Keep the existing `indent`, `content_type`, and Java transform example fields
- Add automated coverage for `offset` in YAML and JSON example output

## Assumptions

- The example remains documentation-oriented and may show fields that users can
  remove when they do not need them.
- This increment changes discovery only; it does not change `DisplayFile`
  runtime behavior.
- The example should make the distinction between block indentation and
  copied-content shifting discoverable without introducing negative `indent`.

## Acceptance Criteria

- [x] Given `sw example DisplayFile`, the CLI prints a valid YAML example of a
      `DisplayFile` entry.
- [x] Given `sw example DisplayFile`, the example includes `offset`.
- [x] Given `sw example DisplayFile --output-format json`, the CLI prints a
      valid JSON example of a `DisplayFile` entry.
- [x] Given `sw example DisplayFile --output-format json`, the example includes
      `offset`.

## Notes

This closes a discovery gap where `DisplayFile.offset` is implemented and
documented elsewhere, but omitted from the most obvious copy-paste example.
