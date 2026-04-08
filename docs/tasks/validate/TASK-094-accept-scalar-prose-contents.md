---
id: TASK-094
title: Accept Scalar Prose Contents
status: completed
category: validate
related_features:
  - SPEC-002
  - SPEC-003
  - SPEC-005
owner: @aattard
created: 2026-04-07
updated: 2026-04-08
---

## Summary

Accept `Markdown.contents` and prerequisite `contents` as either a single
string or an array of strings so YAML runbooks can use block scalars without
rewriting prose into explicit line arrays.

## Scope

- Allow `Markdown.contents` to be either a string or an array of strings
- Allow `Prerequisite.checks[*].contents` to be either a string or an array of
  strings
- Normalize the accepted scalar form into the existing internal line-array
  model
- Drop the implicit terminal blank line that YAML literal scalars add by
  default so scalar rendering matches the explicit array form
- Preserve existing array-based behavior for JSON and YAML runbooks
- Update help or discovery text for the new shorthand
- Add or update automated tests for `validate`, `run`, and `check`

## Assumptions

- This increment is limited to prose-like content fields and does not expand
  scalar shorthand to `Command.commands`, `Patch.patch`, `cleanup`, or other
  execution-oriented arrays.
- Accepting the scalar form for these fields applies across supported input
  formats because the current parser pipeline does not retain source-format
  distinctions after loading.
- Existing array-based runbooks remain valid and unchanged.

## Acceptance Criteria

- [x] Given a runbook whose `Markdown.contents` is a single string, `sw
      validate` accepts that runbook as valid input.
- [x] Given a runbook whose `Prerequisite.checks[*].contents` is a single
      string, `sw validate` accepts that runbook as valid input.
- [x] Given `sw run` with scalar `Markdown.contents`, the generated Markdown
      matches the existing line-array rendering contract.
- [x] Given `sw run` with scalar prerequisite `contents`, the generated
      Markdown matches the existing line-array rendering contract.
- [x] Given `sw check` with scalar prerequisite `contents`, prerequisite
      execution still follows the current contract and exit codes.
- [x] Existing array-based runbooks continue to pass automated tests.

## Notes

This change improves authoring ergonomics, especially for YAML block scalars,
without redesigning the broader runbook schema. Scalar normalization also
ignores the terminal line break that YAML literal scalars add by default so
run output stays aligned with the explicit array form.
