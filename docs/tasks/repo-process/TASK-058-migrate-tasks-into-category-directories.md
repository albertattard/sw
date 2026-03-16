---
id: TASK-058
title: Migrate Tasks Into Category Directories
status: done
category: repo-process
related_features: []
owner: @aattard
created: 2026-03-15
updated: 2026-03-15
---

## Summary

Move existing task files into category-specific directories while keeping the
root task README as the master entry point.

## Scope

- Create the category directories under `docs/tasks/`
- Move existing task files into the correct category directories
- Keep task filenames unchanged during the move
- Update task links in `docs/tasks/README.md`
- Update cross-references from specs and task files to the new task paths

## Assumptions

- Category metadata remains useful even after the directory structure reflects
  the same grouping.
- The migration should preserve stable task ids and task filenames.

## Acceptance Criteria

- [x] Category directories exist for the controlled task categories.
- [x] Existing task files are moved into the correct category directories
      without renaming the files.
- [x] `docs/tasks/README.md` links resolve to the new task paths.
- [x] Existing task/spec cross-references to moved task files are updated.

## Notes

This is a repository-organization change intended to keep the task area
navigable as the number of tracked delivery slices grows.
