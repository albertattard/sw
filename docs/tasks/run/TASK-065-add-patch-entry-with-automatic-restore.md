---
id: TASK-065
title: Add Patch Entry With Automatic Restore
status: done
category: run
related_features:
  - SPEC-003
owner: @aattard
created: 2026-03-16
updated: 2026-03-16
---

## Summary

Add a first-class `Patch` runbook entry so examples can temporarily modify
files and reliably restore them afterward without requiring hand-written undo
commands.

## Scope

- Add a `Patch` entry type to the runbook model and validation
- Allow a `Patch` entry to target one file and declare patch lines in order
- Render `Patch` entries as fenced `diff` blocks in generated Markdown
- Apply patches during execution in runbook order
- Default patch restoration to automatic behavior
- Snapshot original file contents before the first patch touching a file is
  applied in a run
- Restore patched files after success, failure, or timeout
- Unwind patch restoration in reverse order and continue best-effort when one
  restore step fails
- Add CLI coverage for stacked patches and automatic restoration

## Assumptions

- Automatic restore is the default because there is no existing first-class
  `Patch` entry contract to preserve for backward compatibility.
- Snapshot-and-restore is safer than requiring reverse patch authoring and
  should restore original file bytes rather than intermediate patched states.

## Acceptance Criteria

- [x] Given a runbook with a `Patch` entry, the patch is applied during the run
      and rendered as a fenced `diff` block in the generated Markdown.
- [x] Given a successful run with a `Patch` entry, the target file is restored
      automatically at the end of the run.
- [x] Given a failed or timed-out run after a `Patch` entry, the target file is
      still restored automatically.
- [x] Given multiple `Patch` entries that modify the same file, later patches
      can build on the earlier patched state during the run.
- [x] Given multiple `Patch` entries that modify the same file, the final
      automatic restore leaves that file in its original pre-run state.
- [x] Given a failure in one patch restore step, later registered patch
      restores still run and the run reports restore failure afterward.

## Notes

This should remove the need for authors to pair every example patch with a
manual undo command just to keep an example rerunnable.
