---
id: TASK-208
title: Replace Unmaintained Chrono Dependency
status: pending
category: repo-process
related_features:
  - SPEC-003
owner: albertattard
created: 2026-11-17
updated: 2026-11-17
---

## Summary

Replace chrono with a maintained datetime implementation, evaluating Jiff and
`time`, and remove the temporary RUSTSEC-2026-0543 exception from TASK-207.
Target completion before the exception review deadline of 2026-12-17.

## Scope

- Inventory chrono usage in `src/runbook/render.rs` and existing datetime tests.
- Compare Jiff and `time` for contract compatibility, supported Rust version,
  dependency graph, advisory status, and implementation complexity. Jiff is a
  candidate, not a preselected drop-in replacement.
- Establish regression coverage for the existing SPEC-003 datetime contract
  before replacement; intentionally update the spec first if any behavior
  must change, then update this task and CLI help/help-focused tests as needed.
- Preserve RFC 3339 numeric offsets and Z suffixes, fractional widths 1..9,
  RFC 1123 parsing through the current RFC 2822 parser and GMT output,
  custom formats and numeric offsets, time-only base inheritance and midnight
  crossing, default bases, relative distances, and shared anchors.
- Assess format-directive compatibility, invalid input, range/overflow and
  leap-second behavior explicitly; retain established error/exit contracts.
- Refactor datetime logic out of the large rendering module before adding
  migration complexity. Keep business logic outside `main.rs`.
- Remove chrono from the resolved graph and remove the exception and its active
  README notice; retain TASK-207 as a historical record.

## Acceptance Criteria

- [ ] Record the replacement decision and compatibility evidence.
- [ ] Preserve SPEC-003 behavior or intentionally update the relevant spec/task
      before implementing any user-visible differences.
- [ ] Add/update integration CLI tests for compatibility gaps and error cases.
- [ ] No chrono remains in `Cargo.toml` or the resolved dependency graph.
- [ ] Remove the RUSTSEC-2026-0543 exception and update active documentation.
- [ ] `cargo deny check advisories bans` passes against the current database.
- [ ] `./tools/verify.sh` passes.

## Notes

Jiff supports RFC and strftime-style interfaces, but similar directive names do
not guarantee identical parsing or formatting. For example, its documented
`%+` support and `%f` behavior differ from chrono conventions. Do not change
custom-format semantics silently to simplify the dependency replacement.

See the [dependency hygiene guide](../../guides/dependency-hygiene.md) for the
advisory and upstream documentation.
