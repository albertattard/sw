---
id: TASK-084
title: Add Version Discovery Output
status: pending
category: discovery
related_features:
  - SPEC-001
owner: @aattard
created: 2026-03-24
updated: 2026-03-24
---

## Summary

Add a version-discovery entry point so users and agents can confirm exactly
which `sw` build they are running locally or remotely.

## Scope

- Support `sw --version`
- Support `sw version`
- Print a stable human-readable version string to stdout
- Include the package version from `Cargo.toml`
- Include build identity metadata based on the source commit when available
- Append a `-dirty` marker when the binary is built from a working tree with
  uncommitted changes
- Add or update discovery-focused tests

## Assumptions

- The package version remains the product version and is not incremented on
  every build.
- Build identity metadata is intended to improve traceability between local and
  remote binaries.
- When commit metadata is unavailable, the command may fall back to the package
  version alone or another documented placeholder.

## Acceptance Criteria

- [ ] `sw --version` prints version/build identity and exits with `0`.
- [ ] `sw version` prints the same output as `sw --version` and exits with `0`.
- [ ] The printed output includes the package version from `Cargo.toml`.
- [ ] A build with source commit metadata includes that commit identifier in
      the output.
- [ ] A build from a dirty working tree appends `-dirty` to the build
      identity.
- [ ] Help and discovery coverage include the new version entry points.

## Notes

This is primarily a discovery and supportability feature. It makes it much
easier to compare local and remote binaries when behavior differs.
