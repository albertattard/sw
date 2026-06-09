---
id: TASK-165
title: Rebuild Local Binary After Merge Cleanup
status: done
category: repo-process
related_features:
  - AGENTS
owner: albertattard
created: 2026-06-09
updated: 2026-06-09
---

## Summary

Refresh the local release binary after the post-merge cleanup flow updates
`main`, so the `sw` binary on the local path reflects the merged code.

## Scope

- Update `tools/cleanup-merged-branch.sh` to rebuild `target/release/sw`
  after successful branch deletion
- Keep existing conservative branch deletion checks unchanged
- Document that post-merge cleanup refreshes the local release binary

## Assumptions

- The local `sw` command used during development may point to
  `target/release/sw`.
- Rebuilding after cleanup is preferable to leaving agents or users to remember
  a separate manual `cargo build --release` step.

## Acceptance Criteria

- [x] After safe deletion with `git branch -d`, the cleanup tool runs
      `cargo build --release`.
- [x] After verified force deletion with `git branch -D`, the cleanup tool runs
      `cargo build --release`.
- [x] If branch cleanup is refused, the cleanup tool does not rebuild the local
      release binary.
- [x] `AGENTS.md` documents that the cleanup tool refreshes the local release
      binary.
