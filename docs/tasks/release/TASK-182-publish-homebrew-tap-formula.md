---
id: TASK-182
title: Publish Homebrew Tap Formula
status: done
category: release
related_features:
  - SPEC-007
owner: albertattard
created: 2026-08-24
updated: 2026-11-18
---

## Summary

Make the verified Sociable Weaver release available through a maintained
Homebrew tap without requiring users to install the Rust toolchain.

## Scope

- Create the public `albertattard/homebrew-tap` repository.
- Add `Formula/sw.rb` with immutable GitHub Release URLs and SHA-256 checksums.
- Select the Apple Silicon macOS archive on Apple Silicon macOS.
- Select the portable x86_64 Linux archive on x86_64 Linux.
- Install only the `sw` executable into Homebrew's `bin` directory.
- Test the tap installation on Apple Silicon macOS and Oracle Linux.

## Acceptance Criteria

- [x] `brew install albertattard/tap/sw` installs `sw` on Apple Silicon macOS.
- [x] `brew install albertattard/tap/sw` installs `sw` on x86_64 Oracle Linux.
- [x] `sw version` runs after installation on both supported platforms.
- [x] The formula URLs and checksums match the verified GitHub Release assets.
- [x] The formula does not invoke Cargo or require a Rust toolchain.
- [x] Unsupported target combinations do not select a mismatched archive.

## Non-goals

- Publishing to `homebrew/core`.
- Building or publishing Homebrew bottles.
- Supporting Intel macOS, Linux ARM64, Windows, RPM, or DEB.

## Verification

- Run `brew audit --strict --formula` for the tap formula.
- Install the formula from the tap on each supported platform.
- Run `sw version` through the installed Homebrew path.

## Progress Notes

- The public `albertattard/homebrew-tap` repository contains `Formula/sw.rb`.
- On Apple Silicon macOS, `brew install albertattard/tap/sw`,
  `brew audit --strict --formula albertattard/tap/sw`, and
  `brew test albertattard/tap/sw` succeeded.
- On 2026-11-18, Albert confirmed successful Homebrew installation and operation
  on x86_64 Oracle Linux and Apple Silicon macOS, completing verification on
  both supported platforms.
