---
id: SPEC-007
title: Release Distribution Contract
status: in_progress
priority: high
owner: albertattard
last_updated: 2026-09-02
---

## Goal

Provide an official release distribution workflow so users can download either
a version-specific build or the latest available build without depending on
ephemeral CI artifacts.

## User-facing Behavior

Official builds are published as GitHub Release assets.

The distribution model supports two access patterns:

1. Version-specific download path for pinned automation and reproducible setups.
2. Latest release path for users who always want the newest published build.

Normal push and pull-request CI is not itself a release. It validates the code
on a baseline platform and may keep transient build artifacts for diagnostics,
but it does not perform the full official release build matrix.

## Release Outputs

Each official release publishes:

- A compressed release archive for each supported target platform.
- A `SHA256SUMS` asset covering every published release archive.
- A release `README.md` file describing what the artifact contains.

The supported release targets are:

- `aarch64-apple-darwin` (Apple Silicon macOS)
- `x86_64-unknown-linux-musl` (portable 64-bit Linux)

Windows and Intel macOS are not supported release targets in this increment.

The full supported-platform build matrix runs only for tagged releases. Normal
CI runs should not pay the cost of building official assets for every release
platform.

In this increment, the release README includes:
- binary name
- target platform
- version or tag
- commit SHA
- release-history context
- build profile

For releases after the first published release, the release README should
describe the change history between the previous release tag and the current
release tag so a downloaded artifact can explain what changed without requiring
the user to inspect the repository history.

For release-history generation, the "previous release tag" means the nearest
earlier reachable tag matching `v*` in the current commit history. It is not
defined by tag creation time or any global repository ordering that ignores the
released commit's ancestry.

For the first published release, there is no previous reachable release tag.
In that case the release README should include a curated "Supported Features"
section sourced from a maintained repository file instead of a raw commit
history dump.

## Download Paths

### Version-specific

Each release asset is available through a versioned release path.

Example pattern:

```text
https://github.com/<owner>/<repo>/releases/download/<tag>/<asset-name>
```

This path is intended for pinned usage where the caller wants a specific build.

### Latest

The latest published release is available through GitHub’s stable latest
release path.

Example pattern:

```text
https://github.com/<owner>/<repo>/releases/latest
```

The latest release page must expose the current release assets, including the
release binary and release README.

## Homebrew Distribution

Sociable Weaver is available from the maintained upstream tap using:

```text
brew install albertattard/tap/sw
```

The tap formula installs the immutable, checksummed GitHub Release archive for
the user's supported platform. It does not compile Rust on the user's machine.

The supported Homebrew platforms match the official release targets:

- Apple Silicon macOS
- x86_64 Linux, including tested Oracle Linux environments

Unsupported architectures must fail during formula selection rather than
downloading an archive for a different target. The formula lives in the
separate `albertattard/homebrew-tap` repository and is updated for every
published release after its target archives have been verified.

The tagged-release workflow synchronizes `Formula/sw.rb` after it publishes
the GitHub Release assets. The synchronization uses the current tag and the
generated `SHA256SUMS` file; it must not rely on GitHub's mutable latest-release
download URL. A repository secret named `HOMEBREW_TAP_TOKEN` grants the
workflow write access only to the maintained tap. If formula synchronization
fails, the release workflow fails so maintainers can correct the mismatch and
rerun it.

## Release Contract

- CI artifacts remain useful for workflow diagnostics but are not the official
  distribution mechanism.
- Normal push and pull-request CI should run on a baseline platform rather than
  the full release-platform matrix.
- Official downloadable builds are the assets attached to GitHub Releases.
- A tagged release produces version-specific assets for all supported release
  platforms.
- Each platform archive includes the `sw` executable, `LICENSE`, and the
  repository `README.md`.
- Asset names identify the release tag and Rust target triple.
- `SHA256SUMS` lets users verify downloaded archives before extracting them.
- The maintained Homebrew formula installs only the matching official release
  archive and preserves the same supported-platform boundary.
- The tagged-release workflow derives the tap formula's versioned archive URLs
  and SHA-256 values from the same release tag and generated checksum file
  that it publishes.
- The latest release path always points to the newest published GitHub Release.
- Asset naming should remain stable enough that users can identify the correct
  binary for a given platform.

### Release Versioning

- A release-preparation pull request updates the package version in
  `Cargo.toml` before the release tag is created.
- The release tag must use the same semantic version with a `v` prefix; for
  example, package version `0.1.2` is released as tag `v0.1.2`.
- The release tag is created only from the merged, verified release-preparation
  commit on `main`.
- The tagged workflow publishes the archives, checksums, release README, and
  Homebrew formula update as one release transaction.

## Acceptance Criteria

- [ ] Given a tagged release build, the pipeline publishes release assets to a
      GitHub Release.
- [ ] Given a push or pull request, the pipeline runs baseline quality checks
      without building the full release-platform matrix.
- [ ] The published release includes the binary asset and a release README.
- [ ] Given a tagged release build, the pipeline builds official assets for all
      supported release platforms.
- [ ] A tagged release publishes a SHA-256 checksum file for its platform
      archives.
- [ ] The release contains only the declared Apple Silicon macOS and portable
      Linux targets.
- [ ] For non-initial releases, the published release README includes the
      commit subjects between the nearest previous reachable `v*` tag and the
      current release tag.
- [ ] For the first published release, the release README includes a curated
      supported-features summary from a maintained repository file instead of a
      raw full-history commit list.
- [ ] A user can download a specific version from the versioned release path.
- [ ] A user can navigate to the latest release path and obtain the newest
      published build.
- [ ] A user on a supported platform can install the current release with
      `brew install albertattard/tap/sw` without installing Rust.
- [ ] Given a tagged release and configured tap credential, the pipeline
      updates the tap formula to the release's versioned archives and
      checksums.
- [x] Given a release-preparation pull request that sets package version
      `0.1.3`, tag `v0.1.3` is created from its merged commit on `main`.
- [ ] The official release mechanism is documented separately from transient CI
      artifacts.

## Non-goals

- Defining a package manager distribution strategy in this increment.
- Supporting every OS/architecture combination immediately.
- Supporting Windows, Intel macOS, RPM, or other native Linux package formats.
- Adding Sociable Weaver to `homebrew/core` in this increment.
- Replacing CI artifact uploads that are useful for debugging.

## Edge Cases

- Release tag exists but asset upload fails.
- A new release supersedes the previous latest release.
- Different platforms require different asset names.
- Release README content drifts from what the pipeline actually publishes.
- There is no previous release tag to compare against.
- A newer tag exists elsewhere in the repository but is not reachable from the
  released commit.
- The maintained first-release supported-features summary becomes stale.
- The commit list between releases is empty or cannot be resolved.
