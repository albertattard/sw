---
id: TASK-112
title: Surface Cleanup Discovery In Help And Explain
status: done
category: discovery
related_features:
  - SPEC-001
  - SPEC-009
owner: @aattard
created: 2026-04-14
updated: 2026-04-14
---

## Summary

Make `cleanup` discoverable from the CLI's short-form guidance surfaces so
users and agents can find the feature without reading the full run spec.

## Scope

- Update `sw help run` to point users toward `cleanup` alongside other
  runbook-authored `Command` fields
- Update `sw explain run` to describe `cleanup` behavior and the boundary with
  automatic process cleanup
- Update `sw explain example` so it explicitly notes that the `Command`
  example includes `cleanup`
- Add help- and explain-focused CLI coverage

## Assumptions

- Another agent should be able to discover `cleanup` from the CLI without
  repository access.
- `help` should stay syntax-first and only point at `cleanup`, not fully
  restate the complete cleanup contract.
- `explain` is the right place for concise behavioral guidance such as the
  manual-cleanup versus automatic-cleanup distinction.

## Acceptance Criteria

- [x] Given `sw help run`, the CLI points users to `sw example Command` and
      `sw explain run` for runbook-authored fields such as `cleanup`.
- [x] Given `sw explain run`, the CLI documents `cleanup` as a `Command`
      field for manual teardown and explains that explicit `cleanup`
      replaces automatic process cleanup for that entry.
- [x] Given `sw explain example`, the CLI notes that the `Command` example
      includes `cleanup`.
- [x] Help- and explain-focused automated tests cover the updated guidance.

## Notes

This is a discovery improvement only. It does not change `cleanup` runtime or
validation behavior.
