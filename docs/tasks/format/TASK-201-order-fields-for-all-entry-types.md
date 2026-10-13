---
id: TASK-201
title: Order Fields For All Entry Types
status: done
category: format
related_features:
  - SPEC-010
owner: albertattard
created: 2026-10-13
updated: 2026-10-13
---

## Summary

Expand canonical direct-field ordering to every supported entry type using
SPEC-010, retaining the Command order delivered in TASK-200.

## Scope

- Apply explicit field orders to Heading, Markdown, ChangeDirectory,
  DisplayFile, DisplayUrl, Patch, Prerequisite, and Breakpoint entries.
- Preserve absent fields, accepted extra fields, nested field order, and arrays.
- Update help, guidance, and integration-style CLI tests.

## Acceptance Criteria

- [x] JSON, YAML, and YML entries follow their documented canonical orders.
- [x] Command ordering, including legacy working_dir, remains correct.
- [x] Accepted extra fields remain after known fields in their input order.
- [x] Values and nested object/array order are preserved.
- [x] A second formatting run produces byte-identical output.
- [x] Invalid runbooks remain unchanged with the existing exit codes.
- [x] Help, guidance, and automated tests reflect all-entry ordering.
- [x] Full verification passes.

## Verification

- `cargo test --test cli_format --test cli_help` passed (35 tests).
- `./tools/verify.sh` passed: formatting, Clippy, full test suite, release
  build, and Homebrew formula rendering.
