---
id: TASK-192
title: Link Agent Git Guidance To Delivery Guide
status: done
category: repo-process
related_features:
  - release-distribution
owner: albertattard
created: 2026-09-02
updated: 2026-09-02
---

## Summary

Make the release guide the canonical change-delivery procedure and replace
duplicated Git workflow instructions in `AGENTS.md` with a reference to it.

## Scope

- Keep agent-only authorization, branch-protection, and result-reporting rules
  in `AGENTS.md`
- Move procedural delivery guidance to `docs/release/README.md`
- Preserve completed task records as historical evidence

## Acceptance Criteria

- [x] `AGENTS.md` points to the release guide for the delivery procedure.
- [x] `AGENTS.md` retains explicit commit and push authorization boundaries.
- [x] The release guide identifies itself as the canonical maintainer workflow.
- [x] Documentation checks pass.
