---
id: TASK-207
title: Temporarily Except Chrono Maintenance Advisory
status: done
category: repo-process
related_features: []
owner: albertattard
created: 2026-11-17
updated: 2026-11-17
---

## Summary

Unblock the v0.1.13 dependency hygiene gate with a narrowly scoped, documented
exception for RUSTSEC-2026-0543 while retaining the existing dependency policy.

## Decision

The advisory marks every chrono version unmaintained and supplies no patched
version. It does not itself report an exploitable vulnerability. A version bump
cannot resolve it. Migrating the datetime implementation during release
preparation risks changing the SPEC-003 parsing and formatting contract.

Accept this maintenance risk temporarily; ignore only RUSTSEC-2026-0543.
Keep all other advisory checks, yanked-crate denial, and bans unchanged.
No CLI behavior changes, so no product spec or help changes are required.

## Acceptance Criteria

- [x] Add only this advisory ID to `deny.toml`, with rationale and removal task.
- [x] Document ownership, review deadline, risk, and conditions for early removal.
- [x] Track migration separately in TASK-208 without marking it complete.
- [x] Reproduce the failure and verify advisories and bans pass with the exception.
- [x] Run the repository verification suite and preserve unrelated TASK-205 work.

## Exception Lifecycle

- Owner: albertattard.
- Review/removal deadline: 2026-12-17; review before the next release after
  v0.1.13 if sooner. Continued acceptance requires a new explicit assessment.
- Remove the exception when TASK-208 removes chrono, or earlier if the advisory
  is withdrawn. Reassess immediately if new security or soundness evidence
  changes the risk; this is not a blanket claim that chrono is safe.
- cargo-deny 0.20.2 supports advisory-specific reasons but not `expiry`.
  The deadline is a maintainer obligation, not an automatically enforced timer.
- Policy rationale and source evidence: [dependency hygiene guide](../../guides/dependency-hygiene.md).

## Verification

- `cargo deny check advisories bans` with cargo-deny 0.20.2 reproduced the
  original failure against the current advisory database (exit 1;
  RUSTSEC-2026-0543, advisories failed, bans passed).
- The same command passed after adding the exception (exit 0; advisories and
  bans passed). Existing `syn` and `windows-sys` duplicate-version warnings
  remain warnings under the unchanged policy.
- `./tools/verify.sh` passed: formatting, Clippy with warnings denied, all
  tests, release build, and Homebrew formula rendering.
- `git diff --check` passed; changed documentation links resolve locally.
- Compared TASK-205's original spec diff and untracked task file with saved
  originals and confirmed its index entry remains present.
- No runtime or CLI changes were made; existing CLI tests provide regression
  verification without adding tests that duplicate cargo-deny's implementation.
