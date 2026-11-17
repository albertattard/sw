---
id: TASK-206
title: Order Nested Schema Fields During Formatting
status: done
category: format
related_features:
  - SPEC-010
owner: albertattard
created: 2026-11-17
updated: 2026-11-17
---

## Summary

Extend `sw format` to order all schema-defined objects, including prerequisite
checks, according to SPEC-010.

## Acceptance Criteria

- [x] Apply the documented nested field orders in JSON, YAML, and YML.
- [x] Preserve user-defined map order, array order, values, and absent fields.
- [x] Preserve validation behavior and ensure formatting is idempotent.
- [x] Update CLI help and help-focused tests.
- [x] Add integration tests covering nested object families and both formats.
- [x] Run the appropriate verification.

## Verification

- `cargo test --test cli_format --test cli_help` passed.
- `./tools/verify.sh` passed: formatting, Clippy, all tests, release build,
  and Homebrew formula rendering.
- Integration coverage checks each nested object family in JSON, YAML, and
  YML, user-defined map order, accepted extra fields, array and value
  preservation, omitted fields, and repeat-run stability.
- Invalid nested fields retain the validation exit code and leave input files
  unchanged.
