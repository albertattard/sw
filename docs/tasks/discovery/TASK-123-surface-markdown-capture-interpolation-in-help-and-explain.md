---
id: TASK-123
title: Surface Markdown Capture Interpolation In Help And Explain
status: done
category: discovery
related_features:
  - SPEC-001
  - SPEC-009
owner: @aattard
created: 2026-04-23
updated: 2026-04-23
---

## Summary

Make captured-variable interpolation in `Markdown` entries discoverable through
the normal CLI guidance surfaces so users and other models can find the syntax
without reading the raw specs.

## Scope

- Update `sw help run` to mention `@{name}` and `@@{name}`
- Update `sw explain run` to describe Markdown interpolation and escaping
- Cover the new guidance with help and explain tests

## Assumptions

- `sw help run` should stay concise and point to `sw explain run` for fuller
  authoring guidance.
- `sw explain run` is the right place to document the interpolation contract in
  more detail.
- This task is discovery-only and does not change runtime interpolation
  behavior.

## Acceptance Criteria

- [x] Given `sw help run`, the output mentions `@{name}` for captured-variable
      interpolation in `Markdown` and `@@{name}` for the literal form.
- [x] Given `sw help run`, the output still points users to `sw explain run`
      for the broader contract.
- [x] Given `sw explain run`, the output documents `@{name}` interpolation and
      `@@{name}` escaping for Markdown entries.
- [x] Given `sw explain run`, the output makes it clear that Markdown entries
      may interpolate values captured earlier or later in the runbook.

## Notes

This closes a real discovery gap: the behavior already exists, but the current
CLI guidance does not make it easy to find.
