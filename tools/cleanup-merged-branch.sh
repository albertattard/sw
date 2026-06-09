#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: ./tools/cleanup-merged-branch.sh <branch>" >&2
  exit 1
fi

branch="$1"

if [[ "$branch" == "main" ]]; then
  echo "Refusing to delete main." >&2
  exit 1
fi

if ! git rev-parse --verify --quiet "$branch" >/dev/null; then
  echo "Branch does not exist locally: $branch" >&2
  exit 1
fi

current_branch="$(git branch --show-current)"

run_step() {
  printf '+ %s\n' "$*"
  "$@"
}

rebuild_local_binary() {
  run_step cargo build --release
  echo "Rebuilt local release binary from current main."
}

run_step git fetch --prune origin
run_step git switch main
run_step git pull --ff-only origin main

if git branch -d "$branch"; then
  echo "Deleted local branch $branch with git branch -d."
  rebuild_local_binary
  exit 0
fi

echo "git branch -d refused to delete $branch; checking whether the patch is already on main." >&2

if ! git diff --quiet main "$branch"; then
  if [[ -n "$current_branch" && "$current_branch" != "main" ]]; then
    git switch "$current_branch" >/dev/null
  fi
  echo "Refusing to force delete $branch because it still differs from main." >&2
  exit 1
fi

cherry_output="$(git cherry -v main "$branch")"

if [[ -n "$cherry_output" ]] && grep -q '^+' <<<"$cherry_output"; then
  if [[ -n "$current_branch" && "$current_branch" != "main" ]]; then
    git switch "$current_branch" >/dev/null
  fi
  echo "Refusing to force delete $branch because git cherry reports unapplied commits:" >&2
  echo "$cherry_output" >&2
  exit 1
fi

run_step git branch -D "$branch"
echo "Force deleted $branch because it has no content diff from main and git cherry reports no unapplied commits."
rebuild_local_binary
