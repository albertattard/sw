---
id: TASK-152
title: Make Branch Cleanup Conservative
status: done
category: repo-process
related_features:
  - AGENTS
owner: @aattard
created: 2026-05-11
updated: 2026-05-11
---

## Summary

Clarify that local branch cleanup after pull-request merges should avoid force
deletion unless the agent has verified that the branch's work is already
preserved.

## Scope

- Align the `commit changes` trigger phrase with the protected-main pull
  request flow
- Document the normal post-merge cleanup flow for local branches
- Prefer `git branch -d` for branch deletion
- Require verification before using `git branch -D` after squash merges
- Require agents to report why force deletion was appropriate when it is used

## Assumptions

- GitHub squash merges can make Git unable to prove that a local branch is
  merged even when the patch content landed in `main`.
- Force deleting a local branch is acceptable only after confirming that no
  unique work would be lost.

## Acceptance Criteria

- [x] `AGENTS.md` documents the post-merge branch cleanup flow.
- [x] `AGENTS.md` makes `commit changes` branch before verification when the
      current branch is `main`.
- [x] `AGENTS.md` makes `commit changes` avoid direct pushes to `main`.
- [x] `AGENTS.md` keeps branch deletion out of the `commit changes` trigger.
- [x] `AGENTS.md` prefers `git branch -d` for local branch deletion.
- [x] `AGENTS.md` requires verification before `git branch -D`.
- [x] `AGENTS.md` requires agents to report why force deletion was appropriate
      when it is used.
