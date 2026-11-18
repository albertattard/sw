---
id: TASK-210
title: Recognize JSON DisplayFile Fences
status: done
category: display-file
related_features:
  - SPEC-003
owner: albertattard
created: 2026-11-18
updated: 2026-11-18
---

## Summary

Recognize `.json` paths automatically so display entries render a `json`
fenced block without requiring `content_type: json`.

## Scope

- Add `.json` to the shared `DisplayFile` and `DisplayUrl` fence detection.
- Preserve explicit `content_type` precedence, contents, and indentation.
- Update run help, explain output, the entity guide, and their tests.
- Add integration coverage for local files and URL paths.

## Acceptance Criteria

- [x] A `DisplayFile` referencing a `.json` file without `content_type`
      renders a `json` fence and preserves its contents and `indent: 3`.
- [x] A `DisplayUrl` whose path ends in `.json` renders a `json` fence,
      including URLs with query strings.
- [x] Explicit `content_type: text` overrides `.json` detection for both
      display entry types.
- [x] Existing extension mappings and unknown-extension `text` fallback
      continue to pass their tests.
- [x] Help, explain output, and the entity guide describe JSON detection.
- [x] Automated rendering and help-focused tests pass.

## Notes

This change selects a Markdown language label only. It does not parse,
validate, or reformat displayed JSON. Uppercase extensions and other JSON
variants are outside this increment.
