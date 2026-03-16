---
id: TASK-052
title: Support Built-In Java Prerequisite Checks
status: done
related_features:
  - SPEC-003
  - SPEC-005
owner: @aattard
created: 2026-03-15
updated: 2026-03-15
---

## Summary

Add a built-in `java` prerequisite check kind so runbooks can validate Java
versions and Java home locations without shell-based parsing.

## Scope

- Support `kind: "java"` in prerequisite checks
- Support exact major versions like `17`
- Support minimum major versions like `24+`
- Support Java resolution from `PATH`, `java_home`, and `java_home_env`
- Reject invalid `java` prerequisite shapes during validation
- Cover the new prerequisite behavior in `run` and `check`

## Assumptions

- `version: "17"` means exactly Java 17.
- `version: "24+"` means Java 24 or higher.
- `java_home` and `java_home_env` are mutually exclusive.

## Acceptance Criteria

- [x] Given a `java` prerequisite check with `version: "24+"`, the prerequisite
      passes when the resolved Java runtime is Java 24 or higher.
- [x] Given a `java` prerequisite check with `version: "17"`, the prerequisite
      passes only when the resolved Java runtime is exactly Java 17.
- [x] Given a `java` prerequisite check with `java_home_env`, the runtime
      resolves Java from that environment variable.
- [x] Given a `java` prerequisite check with an unset `java_home_env`, the
      prerequisite fails with a clear message.
- [x] Given a `java` prerequisite check with both `java_home` and
      `java_home_env`, the runbook is rejected during validation.

## Notes

This keeps Java-focused runbooks readable and stable, especially when they need
to compare behavior across Java releases or depend on named JDK home
environment variables.
