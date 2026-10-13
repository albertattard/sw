---
id: TASK-203
title: Place DisplayFile Content Type After Path
status: done
category: format
related_features:
  - SPEC-010
owner: albertattard
created: 2026-10-13
updated: 2026-10-13
---

## Summary

Place `DisplayFile.content_type` immediately after `path` in canonical format
output. Preserve the relative order of all other fields and omit absent fields.

## Acceptance Criteria

- [x] JSON, YAML, and YML place `content_type` immediately after `path`.
- [x] Absent fields remain absent; nested values and other entry orders are preserved.
- [x] Repeated formatting remains byte-identical.
- [x] Formatter guidance, CLI help, and tests reflect the revised order.
- [x] Verification and release build pass.

## Verification

- `./tools/verify.sh` passed: formatting, Clippy, all tests, release build,
  and Homebrew formula rendering.
- Formatter integration coverage checks JSON, YAML, and YML field order,
  omitted fields, value preservation, and repeat-run stability.
- Help coverage checks the documented DisplayFile ordering.
