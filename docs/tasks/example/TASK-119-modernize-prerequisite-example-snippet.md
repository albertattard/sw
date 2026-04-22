---
id: TASK-119
title: Modernize Prerequisite Example Snippet
status: done
category: example
related_features:
  - SPEC-005
  - SPEC-008
owner: @aattard
created: 2026-04-22
updated: 2026-04-22
---

## Summary

Update `sw example Prerequisite` so it teaches the built-in Java prerequisite
check instead of a shell-based version probe, and so the example does not
claim stronger validation than the engine actually performs.

## Scope

- Replace the command-based Java version example with a built-in `kind: java`
  example
- Show a Java version rule in the example snippet
- Keep the example documentation-oriented and copyable in both YAML and JSON
- Add automated coverage for the updated example output

## Assumptions

- The built-in Java prerequisite is the preferred authoring pattern for Java
  version checks in this project.
- The built-in Java prerequisite validates Java major version, not vendor
  identity.
- The example should demonstrate a PATH-based Java check unless there is a
  reason to teach `java_home` or `java_home_env`.

## Acceptance Criteria

- [x] Given `sw example Prerequisite`, the CLI prints a valid YAML
      `Prerequisite` entry.
- [x] Given `sw example Prerequisite`, the example uses `kind: java` and a
      `version` field instead of `kind: command` and `commands`.
- [x] Given `sw example Prerequisite --output-format json`, the CLI prints a
      valid JSON `Prerequisite` entry with the same represented contract.
- [x] Given the generated example, the snippet does not imply vendor-specific
      validation that the built-in Java prerequisite does not perform.

## Notes

This is an example-contract update, not a change to prerequisite execution.
The engine already supports built-in Java prerequisite checks.
