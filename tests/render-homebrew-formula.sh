#!/usr/bin/env bash

set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
workspace="$(mktemp -d)"
trap 'rm -rf "$workspace"' EXIT

checksums="$workspace/SHA256SUMS"
formula="$workspace/Formula/sw.rb"

cat > "$checksums" <<'EOF'
0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef  sw-v0.1.1-aarch64-apple-darwin.tar.gz
fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210  sw-v0.1.1-x86_64-unknown-linux-musl.tar.gz
EOF

"$repository_root/tools/render-homebrew-formula.sh" v0.1.1 "$checksums" "$formula"

grep -Fq 'releases/download/v0.1.1/sw-v0.1.1-aarch64-apple-darwin.tar.gz' "$formula"
grep -Fq 'releases/download/v0.1.1/sw-v0.1.1-x86_64-unknown-linux-musl.tar.gz' "$formula"
grep -Fq 'sha256 "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"' "$formula"
grep -Fq 'sha256 "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210"' "$formula"
grep -Fq 'bin.install "sw"' "$formula"
! grep -Fq 'cargo' "$formula"

if "$repository_root/tools/render-homebrew-formula.sh" v0.1.1 /dev/null "$formula" 2>/dev/null; then
  echo "formula generation unexpectedly accepted missing checksums" >&2
  exit 1
fi

cat > "$checksums" <<'EOF'
not-a-checksum  sw-v0.1.1-aarch64-apple-darwin.tar.gz
fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210  sw-v0.1.1-x86_64-unknown-linux-musl.tar.gz
EOF

if "$repository_root/tools/render-homebrew-formula.sh" v0.1.1 "$checksums" "$formula" 2>/dev/null; then
  echo "formula generation unexpectedly accepted a malformed checksum" >&2
  exit 1
fi

printf 'render-homebrew-formula tests passed\n'
