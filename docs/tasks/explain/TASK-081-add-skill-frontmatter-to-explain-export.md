---
id: TASK-081
title: Add Skill Frontmatter To Explain Export
status: done
category: explain
related_features:
  - SPEC-009
owner: @aattard
created: 2026-03-24
updated: 2026-03-24
---

## Summary

Make `sw explain --output-format=skill` emit a valid Codex `SKILL.md` file by
adding the required YAML frontmatter.

## Scope

- Add YAML frontmatter to the generated skill export
- Include at least `name` and `description` in the frontmatter
- Keep the existing skill body guidance and command map structure
- Update skill-export tests to validate the frontmatter contract

## Assumptions

- Codex skill loaders require YAML frontmatter delimited by `---`.
- The exported skill should remain deterministic and repository-derived.

## Acceptance Criteria

- [x] Given `sw explain --output-format=skill`, stdout begins with YAML
      frontmatter delimited by `---`.
- [x] Given `sw explain --output-format=skill`, the YAML frontmatter includes
      `name: sw`.
- [x] Given `sw explain --output-format=skill`, the YAML frontmatter includes a
      non-empty `description`.
- [x] Given `sw explain --output-format=skill --output-file`, the written file
      is accepted as a Codex `SKILL.md`-compatible document.

## Notes

This is a compatibility fix for downstream Codex skill loaders rather than a
change to the underlying explain knowledge model.
