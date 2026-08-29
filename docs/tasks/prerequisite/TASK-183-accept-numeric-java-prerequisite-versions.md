---
id: TASK-183
title: Accept Numeric Java Prerequisite Versions
status: done
category: prerequisite
related_features:
  - SPEC-002
  - SPEC-003
  - SPEC-005
  - SPEC-010
owner: albertattard
created: 2026-08-29
updated: 2026-08-29
---

## Summary

Allow authors to write an exact Java prerequisite major version as a YAML or
JSON integer without weakening the existing version-rule syntax.

## Scope

- Accept non-negative integer exact Java versions alongside the existing string
  form.
- Keep minimum rules such as `"25+"` string-only.
- Evaluate numeric exact versions identically to their string equivalents in
  `sw check` and `sw run`.
- Canonicalize integer exact versions to quoted strings when `sw format`
  rewrites YAML or JSON.
- Document and cover the input and formatting contract.

## Acceptance Criteria

- [x] `version: 25` validates and checks Java 25 exactly.
- [x] `version: "25"` keeps its existing behavior.
- [x] A minimum string value such as `version: "25+"` remains valid.
- [x] Decimal, negative, boolean, and array version values are rejected.
- [x] `sw format` writes numeric exact versions as strings for YAML and JSON.
