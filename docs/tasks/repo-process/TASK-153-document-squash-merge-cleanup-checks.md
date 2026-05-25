---
id: TASK-153
title: Add Reliable Post Merge Cleanup Tool
status: done
category: repo-process
related_features:
  - AGENTS
owner: albertattard
created: 2026-05-11
updated: 2026-05-11
---

## Summary

Make post-merge local branch cleanup executable instead of relying on agents to
manually interpret squash or rebase merge edge cases.

## Scope

- Add a cleanup tool for merged local feature branches
- Fetch and fast-forward `main` before deleting a local feature branch
- Try safe deletion with `git branch -d` first
- Verify content equality before any force deletion
- Verify patch identity with `git cherry -v` before any force deletion
- Keep `AGENTS.md` pointed at the executable tool instead of open-ended Git
  instructions

## Assumptions

- GitHub squash and rebase merges can put the branch's patch on `main` using a
  commit hash that differs from the local branch tip.
- A branch can be safe to force delete after `git branch -d` refuses only when
  the local repository proves that its patch is already represented on current
  `main`.
- Build and cleanup reliability should come from shared tools before detailed
  prose instructions.

## Acceptance Criteria

- [x] `tools/cleanup-merged-branch.sh` deletes a fully merged branch with
      `git branch -d`.
- [x] `tools/cleanup-merged-branch.sh` refuses to delete `main`.
- [x] `tools/cleanup-merged-branch.sh` refuses to force delete when the branch
      still has a content diff from `main`.
- [x] `tools/cleanup-merged-branch.sh` refuses to force delete when
      `git cherry -v main <branch>` reports unapplied commits.
- [x] `tools/cleanup-merged-branch.sh` uses `git branch -D` only after content
      and patch-identity checks pass.
- [x] `AGENTS.md` tells agents to use the cleanup tool after pull-request
      merges.
