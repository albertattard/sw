---
id: SPEC-001
title: Help and Discovery Contract
status: in_progress
priority: high
owner: @aattard
last_updated: 2026-04-23
---

## Problem

Users and AI agents need a reliable way to discover how to use `sw` before
feature commands are fully implemented.

## User-facing Behavior

The CLI provides help entry points:

```bash
sw --help
sw --version
sw help
sw help <subcommand>
sw help --all
sw [command] --help
sw version
```

Current help behavior is human-readable and non-interactive:
- Print usage/help text.
- Print version/build identity on demand.
- Support targeted help for a known subcommand via `sw help <subcommand>`.
- Support aggregated help for all known subcommands via `sw help --all`.
- Keep the placeholder line while help remains incomplete.

## Inputs/Outputs

Input:
- Top-level help flags/command.
- Top-level version flag/command.
- Optional help target subcommand name.
- Optional `--all` flag for expanded command coverage.
- Command-specific help flags (for known commands).

Output:
- Human-readable help on stdout.
- Human-readable version text on stdout.
- Placeholder line indicating work in progress while applicable.
- `sw help <subcommand>` prints the help for that subcommand only.
- `sw help --all` prints top-level help plus help for each known subcommand.
- `sw --version` and `sw version` print the same version string.
- The version string includes the package version and build identity metadata.
- Build identity metadata includes the source commit when available.
- If the binary was built from a working tree with uncommitted changes, the
  version string includes a `-dirty` marker.
- Subcommand help may point users to `example` or `explain` when a question is
  about runbook-authored fields rather than CLI flags.
- Subcommand help should document `--input-file=-` as the stdin convention for
  runbook input where supported, and should explain when `--input-format` is
  needed for piped YAML input.
- Subcommand help should make the format split explicit where relevant:
  file-based runbook workflows default to YAML, while stdin-backed runbook
  input via `--input-file=-` defaults to JSON unless the caller provides
  `--input-format=yaml`.
- Subcommand help for file-generating or snippet-printing commands should make
  the default YAML output clear and should point to explicit JSON flags when
  users or agents need the machine-oriented shape instead.

Exit codes:
- `0`: help printed successfully.
- `1`: unknown command or operational error.

## Acceptance Criteria

- [x] `sw --help` prints top-level usage and exits with `0`.
- [x] `sw --version` prints version/build identity and exits with `0`.
- [x] `sw help` prints top-level usage and exits with `0`.
- [x] `sw version` prints the same version/build identity as `sw --version`
      and exits with `0`.
- [ ] `sw help <subcommand>` prints help for a known subcommand and exits with `0`.
- [ ] `sw help run` documents the CLI flags for `run` and directs users to
      `sw example Command` and `sw explain run` for runbook-authored output
      fields such as `trim_empty_lines`, `stream`, and `cleanup`.
- [ ] `sw help run` makes it clear that captured variables can be referenced
      from Markdown using `@{name}` and escaped with `@@{name}`, while
      directing users to `sw explain run` for the wider interpolation
      contract.
- [ ] `sw help run`, `sw help check`, and `sw help validate` document
      `--input-file=-` for stdin-backed runbook input and `--input-format` for
      explicit YAML stdin input.
- [ ] `sw help run`, `sw help check`, and `sw help validate` make it clear
      that stdin defaults to JSON while file-based authoring elsewhere in the
      CLI defaults to YAML.
- [ ] `sw help example` makes it clear that `sw example DisplayFile` includes
      the Java `collapse_method_body` transform for collapsing method bodies.
- [ ] `sw help example`, `sw help init`, and `sw help import` make it clear
      that file-based snippet and starter-runbook workflows default to YAML.
- [ ] `sw help <subcommand>` with an unknown subcommand exits with `1` and reports a clear error.
- [ ] `sw help --all` prints top-level help plus help for each known subcommand and exits with `0`.
- [x] `sw [command] --help` is documented as the command-level help pattern.
- [x] Help output includes a short in-progress placeholder line.
- [x] The version output contains the package version from `Cargo.toml`.
- [x] The version output includes the source commit identifier when available.
- [x] A build from a dirty working tree appends `-dirty` to the build identity.

## Non-goals

- Full machine-readable help output in v1.
- Complete command implementation beyond help text.
- Auto-incrementing the product version on every local build.

## Notes for Reimplementation

This feature establishes the discoverability contract first; structured
machine-readable help can be added in a follow-up increment.
