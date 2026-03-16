---
id: TASK-010
title: Support DisplayFile Entries
status: done
category: display-file
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Support `DisplayFile` entries in `sw run` so a runbook can copy file contents
into generated Markdown.

## Scope

- Allow `DisplayFile` as a supported runbook entry type
- Require `DisplayFile.path`
- Read the referenced file during rendering
- Resolve `DisplayFile.path` relative to the runbook location
- Render the file contents in a fenced code block
- Detect the fenced block content type from known file extensions
- In this increment, support `.java` as `java`
- Fall back to `text` when the file extension is not recognized

## Assumptions

- `DisplayFile` is a rendering feature only and does not execute the referenced
  file.
- Additional extension mappings may be added in later tasks.

## Acceptance Criteria

- [x] Given a valid `DisplayFile` entry, the referenced file contents are
      copied into the generated Markdown.
- [x] Given a `DisplayFile` entry that references a `.java` file, the
      generated Markdown uses a `java` fenced block.
- [x] Given a `DisplayFile` entry with an unrecognized extension, the
      generated Markdown uses a `text` fenced block.
- [x] Given a `DisplayFile` entry with a missing file, the run exits with `1`
      and reports a clear error.
- [x] Given a `DisplayFile` entry without `path`, validation fails.

## Notes

This task extends the runbook format so generated documentation can include
real source files without duplicating their contents in the runbook itself.
