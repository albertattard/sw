---
id: TASK-198
title: Enforce Datetime Anchor Uniqueness At Runtime
status: done
category: rewrite
related_features:
  - SPEC-003
owner: albertattard
created: 2026-10-07
updated: 2026-10-08
---

## Summary

Allow alternative commands to declare the same `datetime_shift.id`, and enforce
uniqueness among executed rewrite declarations at runtime. This supports macOS
and Linux alternatives sharing one logical timeline without allowing an active
rule to replace another rule's anchor.

## Scope

- Remove global duplicate-ID rejection from validation, while preserving rule
  structure checks and validation of earlier declarations for `use`
- Track executed ID declarations separately from established shift anchors
- Reserve each executed declaration's ID even when no timestamps match
- Fail on a second executed declaration of the same ID, within one output
  block or across commands, without overwriting the first anchor
- Keep skipped commands from reserving IDs or establishing anchors
- Require `use` to resolve an anchor established earlier in the current run
- Report runtime duplicate and unavailable-anchor errors with exit code `1`,
  the ID, and the offending entry and rewrite rule
- Keep internal output rendering and capture reprocessing from counting the
  same executed rule more than once
- Update CLI guidance, help text, and help-focused tests where this contract
  is exposed, and add integration tests for validation and runtime behavior

## Assumptions

- Static validation remains independent of the current host's OS and does not
  attempt to prove condition exclusivity.
- Each run starts with fresh declaration and anchor state. Entries skipped by
  `--start-at` do not contribute either kind of state.
- An executed declaration reserves its ID; only a timestamp match establishes
  the shift delta that `use` can consume.
- Runtime conflicts may be discovered after commands have performed work;
  this change does not add rollback or execution preflight.

## Acceptance Criteria

- [x] Validation accepts otherwise valid duplicate IDs across commands and
      within one output block, independently of the validating host's OS.
- [x] Mutually exclusive macOS/Linux declarations run successfully, and later
      `use` follows the anchor established by the executed alternative.
- [x] Two executed declarations with the same ID fail on the second with exit
      code `1`, identifying the ID, entry, and rewrite rule.
- [x] Duplicate declarations fail within the same output block and across
      commands, including when either or both rules match no timestamps.
- [x] A duplicate declaration does not overwrite or reset the first anchor.
- [x] Skipped declarations do not reserve IDs or establish anchors.
- [x] A declaration skipped by `--start-at` does not conflict with a later
      executed declaration of the same ID.
- [x] An executed `use` fails with exit code `1` and a clear unavailable-anchor
      diagnostic when earlier declarations were skipped or matched nothing,
      including when the consuming rule itself matches nothing.
- [x] Forward references and undeclared `use` IDs still fail validation.
- [x] Output capture generation does not cause an executed rule to conflict
      with its own declaration during internal reprocessing.
- [x] Existing timeline reuse, datetime formatting, and rewrite-generated
      capture tests pass, with guidance and help-focused tests aligned.

## Notes

This increment supersedes the global uniqueness behavior delivered by TASK-018
and retained by TASK-019. Those completed tasks remain historical records.
Implemented runtime declaration tracking separately from established anchors.
Capture reprocessing uses local state, so it neither reserves IDs again in the
run nor overwrites the shared anchor. Diagnostics include the offending entry
and the zero-based `output.rewrite` rule index.

Verified with `./tools/verify.sh` on 2026-10-08: formatting, Clippy, the full test
suite, release build, and Homebrew formula rendering all passed. Integration
coverage includes conditional alternatives, no-match declarations and consumers,
partial runs, and stdout/stderr capture reprocessing; a focused state test checks
that rejected duplicate declarations preserve the original anchor.
