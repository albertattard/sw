---
id: TASK-034
title: Expand Help Command Coverage
status: pending
related_features:
  - SPEC-001
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Expand `sw help` so users can request help for a specific subcommand or print
the full help set for all known subcommands from one entry point.

## Scope

- Support `sw help <subcommand>`
- Support `sw help --all`
- Keep existing `sw --help`, `sw help`, and `sw [command] --help` behavior
- Return a clear error for unknown help targets
- Add or update help-focused integration tests

## Assumptions

- Help remains human-readable in this increment.
- `--all` prints top-level help followed by each known subcommand help.
- The placeholder line remains until help output is considered complete.

## Acceptance Criteria

- [ ] `sw help <subcommand>` prints help for a known subcommand and exits with `0`.
- [ ] `sw help <subcommand>` with an unknown subcommand exits with `1`.
- [ ] `sw help --all` prints help for all known subcommands and exits with `0`.
- [ ] Existing help entry points keep working.

## Notes

This keeps the `help` command itself useful as the CLI grows instead of
requiring users to remember a mix of top-level and command-specific help forms.
