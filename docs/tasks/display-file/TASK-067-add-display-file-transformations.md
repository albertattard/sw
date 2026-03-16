---
id: TASK-067
title: Add DisplayFile Transformations
status: pending
category: display-file
related_features:
  - SPEC-011
owner: @aattard
created: 2026-03-16
updated: 2026-03-16
---

## Summary

Add a transformation pipeline to `DisplayFile` so documentation can present
focused snippets, starting with collapsing Java method bodies without mutating
the underlying source file.

## Scope

- Add `DisplayFile.transform`
- Require `transform.language`
- Require `transform.operations`
- Support `language: java` in the first increment
- Support `collapse_method_body` in the first increment
- Preserve existing `DisplayFile` behavior when `transform` is omitted
- Reject unknown languages and unknown operation types during validation
- Fail clearly when a requested Java method cannot be transformed
- Update CLI help and examples for the new `DisplayFile` contract
- Add integration coverage for transformed display output and validation

## Assumptions

- This feature is documentation-oriented and must not write back to source
  files.
- The first increment should be narrow and deterministic rather than trying to
  support all formats at once.
- Later increments can add `json`, `xml`, and `text` operations without
  changing the top-level `DisplayFile` structure.

## Acceptance Criteria

- [ ] Given a Java source file with a method named `initialize`, a
      `DisplayFile` entry with `transform.language: "java"` and
      `collapse_method_body` renders that method as a single-line body with the
      provided replacement comment, or with `/* Closed for brevity */` when
      `replacement` is omitted.
- [ ] Given a `DisplayFile` entry without `transform`, rendering matches the
      current behavior.
- [ ] Validation rejects an unknown `transform.language`.
- [ ] Validation rejects an unknown transform operation type.
- [ ] Given a Java transform targeting a missing method, `sw run` exits with
      `2` and reports a clear rendering failure.
- [ ] Help and example coverage document the new `DisplayFile` transformation
      shape.

## Notes

This task should implement only the first slice of `SPEC-011`. Follow-up tasks
should add `json`, `xml`, and `text` transformation families once the shared
pipeline is in place.
