---
id: TASK-012
title: Apply Command Indentation
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-12
updated: 2026-03-12
---

## Summary

Apply `Command.indent` to rendered Markdown so command sections can remain
correctly nested inside surrounding Markdown structures such as list items.

## Scope

- Use `Command.indent` as the number of leading spaces for each rendered line
  of the command section
- Apply indentation to the shell block
- Apply indentation to output captions
- Apply indentation to rendered command output blocks
- Leave command rendering unchanged when `indent` is absent

## Assumptions

- `indent` affects rendering only and does not change execution behavior.
- `indent` is interpreted as a count of spaces, not tabs.

## Acceptance Criteria

- [x] Given a `Command` entry with `indent`, each rendered line in that command
      section is prefixed with the configured number of spaces.
- [x] Given a `Command` entry with `indent` and rendered output, the shell
      block, caption, and output block all use the same indentation.
- [x] Given a `Command` entry without `indent`, rendering remains unchanged.

## Notes

This task improves generated Markdown in contexts where command sections need
to stay nested under list items or other indented content.
