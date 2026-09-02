---
id: TASK-195
title: Complete DisplayFile Example Discovery
status: done
category: example
related_features:
  - SPEC-003
  - SPEC-008
owner: albertattard
created: 2026-09-02
updated: 2026-09-02
---

## Summary

Make the copyable `DisplayFile` example demonstrate a bounded excerpt and
align the entry guide with the complete example contract.

## Scope

- Add `start_line` to the YAML and JSON `DisplayFile` example snippets
- Preserve the existing line count, content type, indentation, offset, and
  Java transform examples
- State in the entry guide that `offset` shifts copied content rather than the
  fence or trim markers
- Direct guide readers to `sw example DisplayFile` for the complete copyable
  shape
- Add automated coverage for the `start_line` field in both output formats

## Assumptions

- `show_trim_markers` remains omitted because its default is `true`; showing
  it would add noise without teaching a non-default choice.
- This changes discovery and documentation only, not DisplayFile rendering.

## Acceptance Criteria

- [x] Given `sw example DisplayFile`, the YAML output includes `start_line`
      and `line_count`.
- [x] Given `sw example DisplayFile --output-format json`, the JSON output
      includes the same `start_line` and `line_count` values.
- [x] Given the entry guide's DisplayFile section, it describes `offset` and
      points readers to the complete CLI example.
- [x] The existing content type, indentation, offset, and Java transform
      example fields remain represented in both CLI output formats.

## Notes

This task makes the current excerpt behavior discoverable. It does not add a
new runbook field or alter rendering defaults.
