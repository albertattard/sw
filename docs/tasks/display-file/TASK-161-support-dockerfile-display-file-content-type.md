---
id: TASK-161
title: Support Dockerfile DisplayFile Content Type
status: done
category: display-file
related_features:
  - SPEC-003
owner: albertattard
created: 2026-06-08
updated: 2026-06-08
---

## Summary

Allow `DisplayFile` entries to render Dockerfiles with a Dockerfile fenced-code
label, including extensionless Dockerfile variants such as `Dockerfile-Java8`.

## Scope

- Accept `DisplayFile.content_type: Dockerfile` during validation
- Render `DisplayFile.content_type: Dockerfile` as a Dockerfile fenced block
- Detect `Dockerfile` and `Dockerfile-*` file names when `content_type` is
  omitted
- Reuse the same content type support for `DisplayUrl`
- Update user-facing help, explain, spec, and guide text
- Add CLI coverage for validation and rendering behavior

## Assumptions

- `Dockerfile` remains a rendering label only; it does not change file reading,
  slicing, transforms, indentation, offset behavior, or command execution.
- Dockerfile detection is based on the final path component rather than a file
  extension because common Dockerfile names are extensionless.

## Acceptance Criteria

- [x] Given a `DisplayFile` entry with `content_type: Dockerfile`, validation
      accepts the runbook.
- [x] Given a `DisplayFile` entry with `content_type: Dockerfile`, `sw run`
      renders a `Dockerfile` fenced block.
- [x] Given a `DisplayFile` entry that references `Dockerfile-Java8` without a
      `content_type`, `sw run` renders a `Dockerfile` fenced block.
- [x] Existing recognized extension behavior for `.java`, `.md`, `.markdown`,
      `.sql`, and `.xml` is preserved.
- [x] Help, explain, spec, and guide text describe Dockerfile support.
- [x] Automated tests pass after the change.
