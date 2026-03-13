# SPEC-007: Release Distribution Contract

- Status: Draft
- Owner: `@aattard`
- Created: `2026-03-13`
- Updated: `2026-03-13`

## Goal

Provide an official release distribution workflow so users can download either
a version-specific build or the latest available build without depending on
ephemeral CI artifacts.

## User-facing Behavior

Official builds are published as GitHub Release assets.

The distribution model supports two access patterns:

1. Version-specific download path for pinned automation and reproducible setups.
2. Latest release path for users who always want the newest published build.

## Release Outputs

Each official release publishes:

- A release binary asset for the supported target platform(s).
- A release `README.md` file describing what the artifact contains.

In this increment, the release README includes:
- binary name
- target platform
- version or tag
- commit SHA
- commit summaries since the previous release
- build profile

The release README should describe the change history between the previous
release tag and the current release tag so a downloaded artifact can explain
what changed without requiring the user to inspect the repository history.

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

## Release Contract

- CI artifacts remain useful for workflow diagnostics but are not the official
  distribution mechanism.
- Official downloadable builds are the assets attached to GitHub Releases.
- A tagged release produces version-specific assets.
- The latest release path always points to the newest published GitHub Release.
- Asset naming should remain stable enough that users can identify the correct
  binary for a given platform.

## Acceptance Criteria

- [ ] Given a tagged release build, the pipeline publishes release assets to a
      GitHub Release.
- [ ] The published release includes the binary asset and a release README.
- [ ] The published release README includes the commit subjects between the
      previous release tag and the current release tag.
- [ ] A user can download a specific version from the versioned release path.
- [ ] A user can navigate to the latest release path and obtain the newest
      published build.
- [ ] The official release mechanism is documented separately from transient CI
      artifacts.

## Non-goals

- Defining a package manager distribution strategy in this increment.
- Supporting every OS/architecture combination immediately.
- Replacing CI artifact uploads that are useful for debugging.

## Edge Cases

- Release tag exists but asset upload fails.
- A new release supersedes the previous latest release.
- Different platforms require different asset names.
- Release README content drifts from what the pipeline actually publishes.
- There is no previous release tag to compare against.
- The commit list between releases is empty or cannot be resolved.
