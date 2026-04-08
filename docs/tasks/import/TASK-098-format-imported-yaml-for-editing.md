---
id: TASK-098
title: Format Imported YAML For Editing
status: completed
category: import
related_features:
  - SPEC-006
owner: @aattard
created: 2026-04-08
updated: 2026-04-08
---

## Summary

Make `sw import` emit more editable YAML by separating imported entries with a
blank line and rendering imported multi-line prose with YAML literal block
scalars.

## Scope

- Add a blank line between adjacent items in the top-level imported `entries`
  list when `sw import` writes YAML
- Emit imported multi-line prose fields such as `Markdown.contents` using `|`
  block scalars instead of explicit line arrays
- Preserve the current imported entry order and imported content
- Keep the change limited to `sw import` YAML output rather than changing YAML
  formatting across unrelated commands
- Add coverage for the emitted YAML text as well as parsed validity

## Assumptions

- This increment is about authoring ergonomics for imported YAML, not runbook
  semantics.
- JSON import output remains available and keeps its own readability contract.
- Execution-oriented arrays such as `Command.commands` stay as arrays in YAML.

## Acceptance Criteria

- [x] Given `sw import` YAML output with multiple imported entries, the
      serialized `entries` list includes a blank line between adjacent entry
      items.
- [x] Given imported multi-line Markdown prose in YAML output, the serialized
      `contents` field uses a literal block scalar introduced with `|`.
- [x] The generated YAML still passes `sw validate`.
- [x] Automated tests pass after the change.

## Notes

This keeps YAML as a hand-editable output format instead of exposing the raw
serializer defaults.
