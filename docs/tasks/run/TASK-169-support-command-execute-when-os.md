---
id: TASK-169
title: Support Command Execute When OS
status: done
category: run
related_features:
  - SPEC-003
owner: albertattard
created: 2026-06-29
updated: 2026-06-29
---

## Summary

Add `Command.execute_when` so runbooks can render multiple platform-specific
command alternatives while executing only the command that matches the current
operating system.

## Scope

- Add `Command.execute_when`
- Support `execute_when.fact: os`
- Support `execute_when.equals: macos|linux|windows`
- Keep conditional command blocks rendered even when their command bodies are
  skipped
- Do not add automatic skipped/executed markers to generated Markdown
- Validate the condition shape and reject unsupported facts, values, and
  properties
- Keep the condition model structured so later increments can add additional
  facts or boolean composition

## Assumptions

- The first increment supports only a single fact comparison, not `all`, `any`,
  `not`, environment checks, command existence checks, or shell expressions.
- A skipped command does not evaluate preconditions, assertions, captures,
  output rendering, or cleanup registration.
- Runbook authors who want the generated document to explain platform choices
  can do so with explicit `Markdown` entries around the conditional commands.
- Later commands should not depend on captures that can only come from a
  conditionally skipped command.

## Acceptance Criteria

- [x] Given a `Command` entry with `execute_when.fact: os` and
      `execute_when.equals` matching the current OS, `sw run` renders and
      executes the command normally.
- [x] Given a `Command` entry with `execute_when.fact: os` and
      `execute_when.equals` not matching the current OS, `sw run` renders the
      command block and skips command execution.
- [x] Given a skipped command with `output`, generated Markdown does not
      include command output for that skipped command.
- [x] Given a skipped command with `capture`, later commands cannot rely on
      values that would only be produced by that skipped command.
- [x] Given a skipped command with `cleanup`, no cleanup block is registered
      for that command.
- [x] Given `execute_when` with an unsupported `fact`, validation rejects the
      runbook.
- [x] Given `execute_when.fact: os` with an unsupported `equals` value,
      validation rejects the runbook.
- [x] Given `execute_when` with an unsupported property, validation rejects the
      runbook.
- [x] Given `sw example Command`, the example surfaces the `execute_when`
      shape.
- [x] Given `sw explain run`, the explanation documents conditional command
      execution and the no-automatic-marker rendering contract.

## Notes

This is intentionally not a shell-expression condition system. The runbook
format should keep conditions structured and validateable so future facts, such
as command availability or environment variables, can be added without making
execution dependent on platform-specific shell syntax.
