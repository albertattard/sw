#!/usr/bin/env bash

set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "usage: $0 <output-path>" >&2
  exit 1
fi

output_path="$1"
tag_name="${TAG_NAME:-${GITHUB_REF_NAME:-}}"
commit_sha="${COMMIT_SHA:-${GITHUB_SHA:-HEAD}}"
features_file="docs/release/first-release-supported-features.md"

if [ -z "$tag_name" ]; then
  echo "TAG_NAME or GITHUB_REF_NAME must be set" >&2
  exit 1
fi

if [ ! -f "$features_file" ]; then
  echo "missing first-release feature summary: $features_file" >&2
  exit 1
fi

commit_subject="$(git log -1 --pretty=%s "$commit_sha")"
previous_tag=""

if git rev-parse "${commit_sha}^" >/dev/null 2>&1; then
  previous_tag="$(git describe --tags --abbrev=0 --match 'v*' "${commit_sha}^" 2>/dev/null || true)"
fi

mkdir -p "$(dirname "$output_path")"

cat <<EOF > "$output_path"
# Sociable Weaver Release Artifact

This release contains the official downloadable build published from a tagged GitHub Release.

## Contents

- \`sw-linux-x86_64\`: release binary built with \`cargo build --release\`
- \`sw-macos\`: release binary built with \`cargo build --release\`
- \`sw-windows-x86_64.exe\`: release binary built with \`cargo build --release\`

## Build Metadata

- Tag: \`${tag_name}\`
- Commit: \`${commit_sha}\`
- Summary: \`${commit_subject}\`
- Platforms: \`linux-x86_64\`, \`macos\`, \`windows-x86_64\`
- Profile: \`release\`

EOF

if [ -n "$previous_tag" ]; then
  {
    printf '## Changes Since Previous Release\n\n'
    printf 'Previous reachable release tag: `%s`\n\n' "$previous_tag"
    git log --pretty='- %s' "${previous_tag}..${commit_sha}"
    printf '\n'
  } >> "$output_path"
else
  cat "$features_file" >> "$output_path"
  printf '\n' >> "$output_path"
fi
