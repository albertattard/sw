---
id: TASK-039
title: Implement Example Command
status: done
category: example
related_features:
  - SPEC-008
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Add an `example` subcommand that prints focused JSON snippets for supported
entry types and nested features so users can request a small example without
generating a full starter runbook.

## Scope

- Add the `example` subcommand to the CLI
- Accept a required topic argument
- Print topic-specific JSON snippets to stdout
- Return an error for unknown topics
- Update help output and help-focused tests
- Add integration coverage for at least one entry-type example and one nested
  feature example

## Assumptions

- `sw init` remains the full-runbook scaffolding command.
- Example snippets are documentation-oriented and may require user editing
  before they are fully usable in a real runbook.

## Acceptance Criteria

- [x] Given `sw example Command`, the CLI prints a valid JSON example of a
      `Command` entry.
- [x] Given `sw example DisplayFile`, the CLI prints a valid JSON example of a
      `DisplayFile` entry.
- [x] Given `sw example rewrite.keep_between`, the CLI prints a valid JSON
      example of that rewrite rule fragment.
- [x] Given an unsupported topic, the CLI exits with `1`.
- [x] The `example` command appears in help output.

## Notes

This gives users a lightweight discovery tool for one feature at a time, which
is especially useful when the full starter runbook from `init` contains much
more than they currently need.
