---
id: TASK-125
title: Normalize Scalar-Capable Arrays During JSON To YAML Convert
status: pending
category: format
related_features:
  - SPEC-012
owner: @aattard
created: 2026-04-23
updated: 2026-04-23
---

## Summary

Make `sw convert` emit more editable YAML when converting from JSON by
serializing scalar-capable line arrays as YAML literal block scalars.

## Scope

- Update JSON-to-YAML conversion so documented scalar-capable fields may emit
  YAML literal block scalars with `|` instead of YAML sequences
- Limit that normalization to fields whose runbook contract already accepts
  either a single string or an array of strings
- Cover `Markdown.contents`, `Command.commands`, `Command.cleanup`,
  `Prerequisite.checks[*].contents`, and `Prerequisite.checks[*].commands`
- Preserve existing JSON output behavior
- Preserve YAML sequences for fields that are not documented as scalar-capable
- Add automated conversion coverage for both normalized and non-normalized
  fields

## Assumptions

- This is a conversion ergonomics improvement, not a schema expansion.
- Converting a scalar-capable line array into a YAML block scalar is acceptable
  because those fields already support both shapes in the documented contract.
- This increment should not change `sw format` for existing YAML files, because
  rewriting in-place user-authored YAML arrays into scalars is a separate
  behavior change.

## Acceptance Criteria

- [ ] Given JSON input whose `Markdown.contents` is an array of strings,
      `sw convert` to YAML emits `contents: |` with the equivalent multiline
      content.
- [ ] Given JSON input whose `Command.commands` is an array of strings,
      `sw convert` to YAML emits `commands: |` with the equivalent multiline
      script.
- [ ] Given JSON input whose `Command.cleanup` is an array of strings,
      `sw convert` to YAML emits `cleanup: |` with the equivalent multiline
      script.
- [ ] Given JSON input whose `Prerequisite.checks[*].contents` or
      `Prerequisite.checks[*].commands` is an array of strings, `sw convert`
      to YAML emits that field as `|` with equivalent multiline content.
- [ ] Given JSON input with a string array in a field that is not documented as
      scalar-capable, `sw convert` preserves that field as a YAML sequence.
- [ ] The converted YAML still passes `sw validate`.

## Notes

This intentionally stops at `convert`. If we later want `sw format` to rewrite
existing YAML arrays into block scalars for the same fields, that should be
tracked as a separate increment because it changes in-place formatting
behavior.
