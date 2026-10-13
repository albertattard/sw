---
id: TASK-202
title: Place Output Before Capture
status: done
category: format
related_features:
  - SPEC-010
owner: albertattard
created: 2026-10-13
updated: 2026-10-13
---

## Summary

Adjust canonical Command field order so output precedes capture. This changes
formatting only; capture stage and execution behavior remain unchanged.

## Acceptance Criteria

- [x] JSON, YAML, and YML format output before capture regardless of input order.
- [x] Other field orders, values, nested fields, and array items remain unchanged.
- [x] Repeated formatting remains byte-identical.
- [x] Formatter guidance, help, and tests reflect the revised order.
- [x] Full verification and release build pass.

## Verification

- `./tools/verify.sh` passed: formatting, Clippy, all tests, release build,
  and Homebrew formula rendering.
- Integration tests cover capture-before-output input, canonical output,
  preservation of nested fields and arrays, and repeat-run stability.
