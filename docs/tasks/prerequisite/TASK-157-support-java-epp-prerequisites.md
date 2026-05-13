---
id: TASK-157
title: Support Java EPP Prerequisites
status: done
category: prerequisite
related_features:
  - SPEC-003
  - SPEC-005
owner: @aattard
created: 2026-05-13
updated: 2026-05-13
---

## Summary

Extend built-in Java prerequisite checks so runbooks can verify Java EPP
without writing a command-based `java -version` parser.

## Scope

- Add an optional `distribution` field to `kind: "java"` prerequisite checks
- Support `distribution: "epp"` by requiring the resolved `java -version`
  output to contain the EPP `-perf` marker
- Keep ordinary Java prerequisites unchanged when `distribution` is omitted
- Reject unsupported Java distribution values during validation
- Cover EPP prerequisite behavior in `run`, `check`, validation, and authoring
  documentation

## Assumptions

- Java EPP is still checked with a Java major `version`, usually `"8"`.
- `distribution: "epp"` means the combined stdout/stderr from `java -version`
  must contain `-perf`.
- Distribution checks run after the Java executable is resolved and its major
  version satisfies the declared `version`.

## Acceptance Criteria

- [x] Given a `java` prerequisite check with `version: "8"` and
      `distribution: "epp"`, the prerequisite passes when the resolved
      `java -version` output contains `-perf`.
- [x] Given a `java` prerequisite check with `distribution: "epp"` whose
      resolved Java output does not contain `-perf`, the prerequisite fails
      with a clear message.
- [x] Given a `java` prerequisite check with an unsupported `distribution`,
      validation rejects the runbook.
- [x] Existing Java prerequisite checks without `distribution` keep their
      current behavior.
