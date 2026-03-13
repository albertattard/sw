---
id: TASK-048
title: Improve Human Validation Guidance
status: done
related_features:
  - SPEC-002
  - SPEC-003
  - SPEC-005
owner: @aattard
created: 2026-03-13
updated: 2026-03-13
---

## Summary

Make human-readable validation failures easier to fix by using clearer
field-specific messages and printing the nearby offending block for each error.

## Scope

- Improve selected validation messages with field-specific guidance
- Print a nearby offending block for each human-readable validation error
- Prefer the nearest useful object block over the whole top-level entry when
  the error points into nested structures
- Preserve the machine-readable JSON validation output

## Assumptions

- Human validation output can be more verbose than JSON output because it is
  intended for interactive diagnosis.
- Printing the nearest object block is more useful than printing the whole entry
  when the invalid field belongs to a nested prerequisite or assertion check.

## Acceptance Criteria

- [x] Given a prerequisite `help` array in human validation output, the message
      explains that `help` must be a single string and suggests removing the
      surrounding brackets.
- [x] Given a human-readable validation error inside a prerequisite check, the
      output prints that prerequisite check block.
- [x] Given `sw run` with an invalid runbook, the shared human validation output
      includes the improved message and nearby block.
- [x] Given JSON validation output, the structured validation result remains
      unchanged.

## Notes

Implemented in the shared human validation formatter and targeted validation
messages so all commands present clearer interactive guidance without changing
the JSON contract.
