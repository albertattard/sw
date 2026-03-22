---
id: TASK-076
title: Surface DisplayFile Transform Discovery In Help
status: done
category: discovery
related_features:
  - SPEC-001
  - SPEC-011
owner: @aattard
created: 2026-03-22
updated: 2026-03-22
---

## Summary

Refine `sw help example` so users and agents can discover that the
`DisplayFile` example includes the Java `collapse_method_body` transform for
collapsing method bodies.

## Scope

- Keep `example` help focused on command usage
- Add targeted guidance that points users to `sw example DisplayFile` for the
  Java `collapse_method_body` transform
- Add help-focused CLI coverage for that guidance

## Assumptions

- `help` remains syntax-first rather than dumping full schema details
- Short cross-references are enough to improve transform discovery

## Acceptance Criteria

- [x] Given `sw help example`, the CLI points users to `sw example DisplayFile`
      for the Java `collapse_method_body` transform.
- [x] Help-focused automated tests cover the updated guidance.

## Notes

This keeps help concise while making an implemented DisplayFile feature easier
to find.
