---
id: TASK-100
title: Add Local Verification Tool
status: done
category: repo-process
related_features: []
owner: @aattard
created: 2026-04-09
updated: 2026-04-09
---

## Summary

Add a repository verification tool so local development and interactive commits
use one ordered quality command.

## Scope

- Add a single repository tool that runs the standard Rust verification steps
  in fail-fast order
- Always run `cargo fmt` before lint, test, and release-build verification
- Move the repository verifier under `tools/`
- Update repository guidance to use the shared local tool

## Assumptions

- Reducing interactive retry latency is worth reformatting modified Rust files
  before the rest of the verification flow runs.
- A single command is easier for users and agents to remember and apply
  consistently than mode-specific variants.
- This increment is limited to the local development workflow and does not
  change the existing CI job structure.

## Acceptance Criteria

- [x] `tools/verify.sh` runs `cargo fmt`, Clippy, tests, and the release build
      in fail-fast order.
- [x] `AGENTS.md` uses the shared tool for `commit changes`.
- [x] `README.md` documents the shared tool and its verification sequence.
- [x] The shared tool passes locally.

## Notes

This is a repository-process improvement for local development. CI remains
unchanged.
