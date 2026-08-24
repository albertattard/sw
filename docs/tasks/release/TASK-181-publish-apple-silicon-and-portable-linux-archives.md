---
id: TASK-181
title: Publish Apple Silicon And Portable Linux Release Archives
status: done
category: release
related_features:
  - SPEC-007
owner: albertattard
created: 2026-08-24
updated: 2026-08-24
---

## Summary

Publish clearly named, verifiable GitHub Release archives for the two platforms
that Sociable Weaver currently supports: Apple Silicon macOS and portable
64-bit Linux.

## Scope

- Define the supported release targets in `SPEC-007`.
- Build Apple Silicon macOS on the GitHub-hosted `macos-15` runner.
- Build a portable Linux executable for `x86_64-unknown-linux-musl`.
- Package each executable with the project licence and README.
- Publish versioned `.tar.gz` archives and a `SHA256SUMS` file on tagged GitHub
  Releases.
- Remove Windows from the official release matrix and release documentation.

## Acceptance Criteria

- [x] A `v*` tag publishes `sw-<tag>-aarch64-apple-darwin.tar.gz`.
- [x] A `v*` tag publishes `sw-<tag>-x86_64-unknown-linux-musl.tar.gz`.
- [x] Each archive contains `sw`, `LICENSE`, and `README.md`.
- [x] The release publishes `SHA256SUMS` covering both archives.
- [x] The tagged-release matrix does not build or publish a Windows asset.
- [x] The generated release README describes the two supported targets and the
      checksum asset.
- [x] The two archives install and run on their intended platforms.

## Non-goals

- Homebrew tap publication.
- RPM, DEB, or distribution-repository publication.
- Intel macOS or Windows support.

## Verification

- Validate the workflow syntax before merging.
- Push a test `v*` tag only when release publication is intentionally
  authorised, then verify the GitHub Release assets and checksums on the two
  supported platforms.

## Completion Notes

- `v0.1.0` published both target archives, the generated release README, and
  `SHA256SUMS`.
- The Apple Silicon archive passed checksum verification and ran `sw version`
  on an ARM64 macOS host.
- The portable Linux archive passed checksum verification and ran `sw version`
  on Oracle Linux.
