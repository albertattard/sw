---
id: TASK-095
title: Surface DisplayFile Indent In Example Output
status: completed
category: example
related_features:
  - SPEC-008
owner: @aattard
created: 2026-04-08
updated: 2026-04-08
---

## Summary

Update `sw example DisplayFile` so the printed snippet includes the supported
`indent` field, making block-level code-fence indentation discoverable without
having to read the full rendering spec first.

## Scope

- Add `indent` to the `DisplayFile` example snippet
- Keep the existing Java `collapse_method_body` transform in the example
- Add or update automated coverage for the richer `DisplayFile` example

## Assumptions

- The example remains documentation-oriented and should show common supported
  fields that users can remove if they do not need them.
- This increment changes discovery only; it does not change `DisplayFile`
  runtime behavior.

## Acceptance Criteria

- [x] Given `sw example DisplayFile`, the CLI prints a valid JSON example of a
      `DisplayFile` entry.
- [x] Given `sw example DisplayFile`, the example includes `indent`.
- [x] Given `sw example DisplayFile`, the example continues to include the
      Java `collapse_method_body` transform shape.

## Notes

This closes a discovery gap where `DisplayFile.indent` is implemented and
documented elsewhere, but omitted from the most obvious copy-paste example.
