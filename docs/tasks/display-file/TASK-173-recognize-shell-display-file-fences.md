---
id: TASK-173
title: Recognize Shell DisplayFile Fences
status: done
category: display-file
related_features:
  - SPEC-003
owner: albertattard
created: 2026-08-07
updated: 2026-08-07
---

## Summary

Recognize shell script files in `DisplayFile` entries so generated Markdown
uses a `shell` fenced block instead of falling back to `text`.

## Scope

- Extend shared display fence detection to recognize `.sh` as `shell`
- Preserve the matching `DisplayUrl` extension behavior
- Preserve existing recognized extensions and the `text` fallback
- Update user-facing help, explain, and guide text
- Add rendering coverage for local files and URLs

## Assumptions

- This change affects rendering only; `DisplayFile` and `DisplayUrl` never
  execute the displayed script.
- `.sh` is the sole extension in scope. Other shell-related extensions and an
  explicit `content_type: shell` are separate product decisions.

## Acceptance Criteria

- [x] Given a `DisplayFile` entry that references a `.sh` file, `sw run`
      renders the snippet in a `shell` fenced block.
- [x] Given a `DisplayUrl` entry whose URL path ends in `.sh`, `sw run`
      renders the fetched body in a `shell` fenced block.
- [x] Existing recognized extensions and unknown-extension fallback behavior
      remain unchanged.
- [x] Help, explain, guide, and specification text describe `.sh` detection.
- [x] Automated tests pass after the change.

## Notes

The detector is shared by `DisplayFile` and `DisplayUrl`, and the runbook
contract explicitly keeps their recognized path mappings aligned. Treating
shell scripts as `shell` makes generated instructions easier to read without
granting execution behavior to either display entry.
