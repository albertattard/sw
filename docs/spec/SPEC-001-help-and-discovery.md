---
id: SPEC-001
title: Help and Discovery Contract
status: Done
priority: High
owner: @aattard
last_updated: 2026-03-04
---

## Problem

Users and AI agents need a reliable way to discover how to use `sw` before
feature commands are fully implemented.

## User-facing Behavior

The CLI provides help entry points:

```bash
sw --help
sw help
sw [command] --help
```

Initial version behavior is intentionally minimal and non-interactive:
- Print usage/help text.
- Include a clear placeholder message that functionality is still in progress.

## Inputs/Outputs

Input:
- Top-level help flags/command.
- Command-specific help flags (for known commands).

Output:
- Human-readable help on stdout.
- Placeholder line indicating work in progress.

Exit codes:
- `0`: help printed successfully.
- `1`: unknown command or operational error.

## Acceptance Criteria

- [ ] `sw --help` prints top-level usage and exits with `0`.
- [ ] `sw help` prints top-level usage and exits with `0`.
- [ ] `sw [command] --help` is documented as the command-level help pattern.
- [ ] Help output includes a short in-progress placeholder line.

## Non-goals

- Full machine-readable help output in v1.
- Complete command implementation beyond help text.

## Notes for Reimplementation

This feature establishes the discoverability contract first; structured
machine-readable help can be added in a follow-up increment.
