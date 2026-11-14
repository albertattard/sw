---
id: TASK-204
title: Support Generated Run Secrets
status: pending
category: run
related_features:
  - SPEC-013
  - SPEC-003
  - SPEC-010
owner: albertattard
created: 2026-11-14
updated: 2026-11-14
---

## Summary

Implement generated, run-scoped secrets and explicit command environment
bindings according to SPEC-013. Preserve real values for execution while
masking output controlled by `sw`.

## Scope

- Validate declarations, bindings, and generation constraints.
- Use the specified alphanumeric alphabet and uniform secure generation; leave
  configurable additional characters for a future increment.
- Generate independent secrets once per invocation and inject bindings into
  commands and cleanup, including across entries.
- Generate secrets and print the optional reveal list before prerequisite
  checks, including when a prerequisite subsequently fails.
- Redact known values in rendered output and diagnostics, including streaming
  and failure paths; implement the dedicated stderr reveal list independently
  of debug, without unmasking other output. Follow the specified list format,
  declaration order, and empty case.
- Keep standalone `sw check` validation-only for secrets: do not generate or
  reveal them or inject command bindings into prerequisite checks.
- Place `environment` immediately before `commands` in canonical JSON and YAML
  formatting, and update format help and help-focused tests.
- Update CLI help, examples, and guides, including lifetime and exposure limits.
- Add integration tests for the behavior and output contracts in SPEC-013.
- Explicitly test masking a captured secret after interpolation and a secret
  split across streamed output chunks.

## Acceptance Criteria

- [ ] SPEC-013 acceptance criteria are implemented and verified.
- [ ] Existing runbooks and CLI output contracts continue to work unchanged.
