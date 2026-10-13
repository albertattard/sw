---
id: TASK-200
title: Order Command Fields During Format
status: done
category: format
related_features:
  - SPEC-010
owner: albertattard
created: 2026-10-12
updated: 2026-10-13
---

## Summary

Make `sw format` write direct `Command` fields in the canonical order defined
in SPEC-010 for both JSON and YAML.

## Scope

- Order fields as `type`, `debug`, `execute_when`, `working_directory`,
  `working_dir`, `commands`, `assert`, `capture`, `output`, `cleanup`, `timeout`,
  and `indent`.
- Retain the legacy `working_dir` spelling when present; do not infer defaults.
- Preserve order in other entry types, nested objects, and all arrays.
- Update formatter guidance, CLI help, and help-focused tests.
- Add integration-style CLI tests for ordering and repeat-run stability.

## Acceptance Criteria

- [x] Differently ordered JSON and YAML Command entries format into the
      specified field order.
- [x] Missing optional fields are not added, and legacy `working_dir` is
      retained immediately after the `working_directory` position.
- [x] Formatting preserves values, nested field order, other entry field
      order, and array item order.
- [x] A second format run produces byte-identical output.
- [x] Invalid runbooks remain unchanged, retaining existing exit codes.
- [x] Formatter guidance and CLI help describe the ordering behavior, with
      updated help-focused tests.
- [x] Relevant automated tests pass.

## Verification

- `cargo test --test cli_format --test cli_help` passed.
- `./tools/verify.sh` passed (format, Clippy, full test suite, release build,
  and Homebrew formula rendering).
- Formatting a temporary copy of the continuous-recording workshop runbook
  placed `capture` before `output` in its jcmd entry and produced identical
  bytes on a second run.
