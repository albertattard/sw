#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 0 ]]; then
  echo "Usage: ./tools/verify.sh" >&2
  exit 1
fi

run_step() {
  printf '+ %s\n' "$*"
  "$@"
}

run_step cargo fmt
run_step cargo clippy --all-targets --all-features -- -D warnings
run_step cargo test
run_step cargo build --release
