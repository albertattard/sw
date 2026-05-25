---
id: TASK-151
title: Use PR Flow For Protected Main
status: done
category: repo-process
related_features:
  - AGENTS
owner: albertattard
created: 2026-05-11
updated: 2026-05-11
---

## Summary

Update the agent commit workflow so protected `main` updates go through a
feature branch and pull request instead of attempting a direct push.

## Scope

- Teach the `commit changes` trigger to create a feature branch before
  committing when the current branch is `main`
- Push the feature branch to `origin`
- Open a pull request to `main` when the GitHub CLI is available
- Keep direct pushes available for non-protected working branches

## Assumptions

- `main` requires the GitHub Actions `Quality` status check before updates.
- Pull requests are the right path for satisfying branch protection.
- The user phrase `commit changes` remains explicit permission to commit and
  push the resulting branch.

## Acceptance Criteria

- [x] `AGENTS.md` tells agents not to push directly to `main`.
- [x] `AGENTS.md` routes `commit changes` from `main` through a feature branch.
- [x] `AGENTS.md` tells agents to open or report a pull request for non-`main`
      pushed branches.
