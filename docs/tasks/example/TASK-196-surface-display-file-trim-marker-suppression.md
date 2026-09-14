---
id: TASK-196
title: Surface DisplayFile Trim Marker Suppression
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

Make the non-default `DisplayFile` trim-marker suppression option visible in
the complete CLI example and its entry-guide description.

## Scope

- Add `show_trim_markers: false` to the YAML and JSON DisplayFile examples
- Preserve all existing complete-example fields
- Explain in the entry guide that the field suppresses the default omission
  markers
- Add automated coverage for both output formats

## Assumptions

- The field demonstrates an explicit non-default choice; omitting it continues
  to enable trim markers by default.
- This is a documentation and discovery change only.

## Acceptance Criteria

- [x] Given `sw example DisplayFile`, its YAML output includes
      `show_trim_markers: false`.
- [x] Given `sw example DisplayFile --output-format json`, its JSON output
      includes `"show_trim_markers": false`.
- [x] Given the entry guide's DisplayFile section, it identifies the field as
      the explicit alternative to default omission markers.
- [x] The complete example retains line slicing, content type, indentation,
      offset, and Java transform fields.
