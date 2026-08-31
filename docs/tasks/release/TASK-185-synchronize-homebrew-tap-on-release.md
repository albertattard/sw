---
id: TASK-185
title: Synchronize Homebrew Tap On Release
status: done
category: release
related_features:
  - SPEC-007
owner: albertattard
created: 2026-08-31
updated: 2026-08-31
---

## Summary

Keep the maintained Homebrew tap aligned with every official release so
`brew upgrade albertattard/tap/sw` installs the newest verified release rather
than remaining on an older formula revision.

## Scope

- Generate the tap formula from the release tag and the published archive
  checksums.
- Run formula synchronization only after a tagged release publishes its assets.
- Use a dedicated `HOMEBREW_TAP_TOKEN` repository secret to write to
  `albertattard/homebrew-tap`.
- Keep stable formula installs binary-only; do not add a Rust build dependency.
- Add local coverage for formula generation and document maintainer setup.

## Acceptance Criteria

- [x] A tag release generates archive URLs and SHA-256 values for both
      supported platforms in `Formula/sw.rb`.
- [x] The formula generator rejects missing or malformed checksum input.
- [x] The release workflow updates the tap only after release assets are
      published and fails when synchronization cannot complete.
- [x] Stable `brew install albertattard/tap/sw` continues to avoid Cargo and a
      Rust toolchain.

## Verification

- Run `./tools/verify.sh`.
- Configure the `HOMEBREW_TAP_TOKEN` secret, publish a test release tag, and
  verify the tap formula URLs, checksums, `brew audit`, and installation on
  each supported platform.

## Completion Notes

- The v0.1.1 tagged-release workflow generated and published the supported
  archives and `SHA256SUMS`, then synchronized `Formula/sw.rb` in
  `albertattard/homebrew-tap` successfully.
- The updated formula referenced the v0.1.1 versioned release URLs and matching
  checksums. On Apple Silicon macOS, Homebrew upgraded the installed formula
  from 0.1.0 to 0.1.1 without installing Rust.
