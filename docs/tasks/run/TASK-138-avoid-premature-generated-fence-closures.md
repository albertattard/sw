---
id: TASK-138
title: Avoid Premature Generated Fence Closures
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-04-27
updated: 2026-04-27
---

## Summary

Prevent generated Markdown code fences from closing prematurely when rendered
content itself contains Markdown fence delimiters such as triple backticks.

## Scope

- Detect fence delimiters inside generated fenced block content
- Use tilde fences when generated block content contains triple backticks
- Choose a longer safe delimiter when content contains both backtick and tilde
  fence runs
- Preserve language labels such as `shell`, `json`, `diff`, and detected file
  content types
- Apply the behavior to generated fences for commands, command output,
  displayed files, and patches
- Add regression coverage for command blocks and command output

## Assumptions

- Backtick fences remain the default for generated blocks that do not contain
  conflicting fence delimiters.
- Markdown entries remain authored Markdown and are not wrapped by the renderer.

## Acceptance Criteria

- [x] Given a command block whose command text includes triple backticks, the
      generated command block uses a safe fence delimiter.
- [x] Given command output whose rendered text includes triple backticks, the
      generated output block uses a safe fence delimiter and preserves the
      content type label.
- [x] Existing generated fenced blocks keep their current backtick shape when
      their contents do not contain conflicting delimiters.
- [x] Existing tests pass.
