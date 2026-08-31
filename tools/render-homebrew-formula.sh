#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 3 ]]; then
  echo "usage: $0 <tag> <sha256sums-path> <output-path>" >&2
  exit 1
fi

tag_name="$1"
checksums_path="$2"
output_path="$3"

if [[ ! "$tag_name" =~ ^v[0-9][A-Za-z0-9._-]*$ ]]; then
  echo "tag must start with v followed by a version identifier" >&2
  exit 1
fi

if [[ ! -f "$checksums_path" ]]; then
  echo "missing checksum file: $checksums_path" >&2
  exit 1
fi

checksum_for() {
  local asset_name="$1"
  local matches
  local checksum

  matches="$(awk -v asset_name="$asset_name" '$2 == asset_name { print $1 }' "$checksums_path")"
  if [[ "$(printf '%s\n' "$matches" | sed '/^$/d' | wc -l | tr -d ' ')" != "1" ]]; then
    echo "expected exactly one checksum for $asset_name" >&2
    exit 1
  fi

  checksum="$matches"
  if [[ ! "$checksum" =~ ^[[:xdigit:]]{64}$ ]]; then
    echo "invalid SHA-256 checksum for $asset_name" >&2
    exit 1
  fi

  printf '%s\n' "$checksum"
}

macos_asset="sw-${tag_name}-aarch64-apple-darwin.tar.gz"
linux_asset="sw-${tag_name}-x86_64-unknown-linux-musl.tar.gz"
macos_checksum="$(checksum_for "$macos_asset")"
linux_checksum="$(checksum_for "$linux_asset")"

mkdir -p "$(dirname "$output_path")"

cat > "$output_path" <<EOF
class Sw < Formula
  desc "Executable documentation runbook CLI"
  homepage "https://github.com/albertattard/sw"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/albertattard/sw/releases/download/${tag_name}/${macos_asset}"
      sha256 "${macos_checksum}"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/albertattard/sw/releases/download/${tag_name}/${linux_asset}"
      sha256 "${linux_checksum}"
    end
  end

  def install
    bin.install "sw"
  end

  test do
    system bin/"sw", "version"
  end
end
EOF
