---
id: SPEC-001
title: Help and Discovery Contract
status: in_progress
priority: high
owner: @aattard
last_updated: 2026-03-22
---

## Problem

Users and AI agents need a reliable way to discover how to use `sw` before
feature commands are fully implemented.

## User-facing Behavior

The CLI provides help entry points:

```bash
sw --help
sw help
sw help <subcommand>
sw help --all
sw [command] --help
```

Current help behavior is human-readable and non-interactive:
- Print usage/help text.
- Support targeted help for a known subcommand via `sw help <subcommand>`.
- Support aggregated help for all known subcommands via `sw help --all`.
- Keep the placeholder line while help remains incomplete.

## Inputs/Outputs

Input:
- Top-level help flags/command.
- Optional help target subcommand name.
- Optional `--all` flag for expanded command coverage.
- Command-specific help flags (for known commands).

Output:
- Human-readable help on stdout.
- Placeholder line indicating work in progress while applicable.
- `sw help <subcommand>` prints the help for that subcommand only.
- `sw help --all` prints top-level help plus help for each known subcommand.
- Subcommand help may point users to `example` or `explain` when a question is
  about runbook-authored fields rather than CLI flags.
- Subcommand help should document `--input-file=-` as the stdin convention for
  runbook input where supported, and should explain when `--input-format` is
  needed for piped YAML input.

Exit codes:
- `0`: help printed successfully.
- `1`: unknown command or operational error.

## Acceptance Criteria

- [x] `sw --help` prints top-level usage and exits with `0`.
- [x] `sw help` prints top-level usage and exits with `0`.
- [ ] `sw help <subcommand>` prints help for a known subcommand and exits with `0`.
- [ ] `sw help run` documents the CLI flags for `run` and directs users to
      `sw example Command` and `sw explain run` for runbook-authored output
      fields such as `trim_empty_lines`.
- [ ] `sw help run`, `sw help check`, and `sw help validate` document
      `--input-file=-` for stdin-backed runbook input and `--input-format` for
      explicit YAML stdin input.
- [ ] `sw help example` makes it clear that `sw example DisplayFile` includes
      the Java `collapse_method_body` transform for collapsing method bodies.
- [ ] `sw help <subcommand>` with an unknown subcommand exits with `1` and reports a clear error.
- [ ] `sw help --all` prints top-level help plus help for each known subcommand and exits with `0`.
- [x] `sw [command] --help` is documented as the command-level help pattern.
- [x] Help output includes a short in-progress placeholder line.

## Non-goals

- Full machine-readable help output in v1.
- Complete command implementation beyond help text.

## Notes for Reimplementation

This feature establishes the discoverability contract first; structured
machine-readable help can be added in a follow-up increment.
