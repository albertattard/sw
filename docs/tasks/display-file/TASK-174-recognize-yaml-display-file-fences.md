---
id: TASK-174
title: Recognize YAML DisplayFile Fences
status: done
category: display-file
related_features:
  - SPEC-003
owner: albertattard
created: 2026-08-22
updated: 2026-08-22
---

## Summary

Recognize YAML files and explicit YAML content types in `DisplayFile` entries
so generated Markdown uses a `yaml` fenced block instead of `text`.

## Scope

- Extend shared display fence detection to recognize `.yaml` and `.yml` as
  `yaml`
- Accept `content_type: yaml` for `DisplayFile` and `DisplayUrl`
- Preserve matching `DisplayUrl` extension behavior
- Preserve existing recognized extensions and the `text` fallback
- Update user-facing help, explain, guide, and specification text
- Add rendering coverage for local files and URLs

## Assumptions

- This change affects Markdown rendering only; display entries never parse or
  execute the displayed YAML.
- `.yaml` and `.yml` are the only extensions in scope.

## Acceptance Criteria

- [x] Given a `DisplayFile` entry that references a `.yaml` file, `sw run`
      renders the snippet in a `yaml` fenced block.
- [x] Given a `DisplayFile` entry that references a `.yml` file, `sw run`
      renders the snippet in a `yaml` fenced block.
- [x] Given a `DisplayFile` entry with `content_type: yaml`, `sw validate`
      accepts the runbook and `sw run` uses a `yaml` fenced block.
- [x] Given a `DisplayUrl` entry whose URL path ends in `.yaml` or `.yml`,
      `sw run` renders the fetched body in a `yaml` fenced block.
- [x] Given a `DisplayUrl` entry with `content_type: yaml`, `sw validate`
      accepts the runbook and `sw run` uses a `yaml` fenced block.
- [x] Existing recognized extensions and unknown-extension fallback behavior
      remain unchanged.
- [x] Help, explain, guide, and specification text describe YAML detection.
- [x] Automated tests pass after the change.

## Notes

`DisplayFile` and `DisplayUrl` share fence detection, and the runbook contract
keeps their recognized path mappings aligned. A `yaml` label improves syntax
highlighting without changing displayed content semantics.
