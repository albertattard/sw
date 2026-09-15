---
id: TASK-197
title: Support Properties DisplayFile Content Type
status: done
category: display-file
related_features:
  - SPEC-003
owner: albertattard
created: 2026-09-15
updated: 2026-09-15
---

## Summary

Allow `DisplayFile` entries to declare `content_type: properties` so runbooks
can render Java properties and manifest-style configuration with a properties
fenced-code label.

## Scope

- Accept `DisplayFile.content_type: properties` during validation
- Render a `properties` fenced block when that value is declared
- Reuse the shared display content-type contract for `DisplayUrl`
- Document the explicit label without adding `.properties` extension detection
- Add validation and rendering coverage

## Assumptions

- `properties` is a rendering label only; it does not parse, validate, or
  otherwise change the displayed file contents.
- Extension inference remains unchanged: an omitted `content_type` on a
  `.properties` file continues to fall back to `text`.

## Acceptance Criteria

- [x] Given `DisplayFile.content_type: properties`, validation accepts the
      runbook.
- [x] Given `DisplayFile.content_type: properties`, `sw run` renders a
      `properties` fenced block.
- [x] Existing display content types and extension inference remain unchanged.
- [x] The spec, authoring guide, CLI discovery text, and automated tests cover
      the explicit label.
