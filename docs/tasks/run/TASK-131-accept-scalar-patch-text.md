---
id: TASK-131
title: Accept Scalar Patch Text
status: done
category: run
related_features:
  - SPEC-003
  - SPEC-008
  - SPEC-012
owner: @aattard
created: 2026-04-25
updated: 2026-04-25
---

## Summary

Allow `Patch.patch` to be authored as a YAML literal scalar so patch entries
can use `|` instead of a sequence of quoted patch lines.

## Scope

- Update the run contract so `Patch.patch` accepts a string or an array of
  strings
- Normalize scalar patch text using the same terminal-line-break behavior as
  other scalar-capable multiline fields
- Update `sw example Patch` to prefer YAML scalar patch text
- Normalize JSON patch arrays to YAML block scalars during conversion
- Add validation, run, example, and conversion coverage

## Assumptions

- JSON runbooks should continue to support the existing array form.
- YAML scalar patch text should preserve intentional blank lines and drop only
  the literal-scalar terminator line break.
- This is an authoring convenience and should not change patch application or
  restore semantics.

## Acceptance Criteria

- [x] Given a YAML `Patch` entry whose `patch` field is a scalar string,
      validation succeeds.
- [x] Given a YAML `Patch` entry whose `patch` field is a scalar string,
      `sw run` applies and renders the patch correctly.
- [x] Given a JSON `Patch` entry whose `patch` field is an array of strings,
      existing validation and run behavior continues to work.
- [x] Given `sw example Patch`, the YAML example uses `patch: |`.
- [x] Given `sw example Patch --output-format json`, the JSON example keeps the
      array form.
- [x] Given JSON-to-YAML conversion of a `Patch` entry, patch arrays are
      normalized to YAML block scalars.
- [x] Automated tests cover the updated behavior.

## Notes

This task does not change the patch command invocation, automatic restore
behavior, or patch failure handling.
