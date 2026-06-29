---
id: TASK-147
title: Support Literal At Before Capture Interpolation
status: done
category: run
related_features:
  - SPEC-003
owner: albertattard
created: 2026-06-29
updated: 2026-06-29
---

## Summary

Allow authors to render a literal `@` immediately before a captured value
without moving the `@` into the captured variable. This supports image digest
references such as `image:tag@sha256:...` while keeping capture values focused
on the digest itself.

## Scope

- Support `\@` as a literal-at escape wherever existing captured-variable
  interpolation applies
- Render `\@@{name}` as `@` followed by the captured value
- Preserve the existing `@@{name}` behavior for literal placeholder syntax
- Keep missing or out-of-order `\@@{name}` references subject to the same
  validation and runtime errors as `@{name}`
- Update help, explain, guides, and specs for the new escape rule
- Add CLI coverage for command, Markdown, and caption interpolation

## Assumptions

- `\@` is a literal-at escape that can be placed immediately before normal
  `@{name}` interpolation.
- `@@{name}` remains the escape for rendering literal `@{name}` text.
- Captured values are still inserted unchanged after the literal `@`.

## Acceptance Criteria

- [x] Given command content containing `\@@{name}`, the executed command sees
      `@` followed by the captured value.
- [x] Given Markdown content containing `\@@{name}`, the rendered Markdown
      includes `@` followed by the captured value.
- [x] Given `Command.output.caption` containing `\@@{name}`, the caption
      includes `@` followed by the captured value.
- [x] Given a command references `\@@{name}` before the variable is captured,
      validation fails with the existing capture ordering error.
- [x] Given content containing `@@{name}`, the rendered output still includes
      literal `@{name}` text.
- [x] Discovery surfaces document the new literal-at escape.
