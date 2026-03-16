# Tasks

File-based task tracker for Sociable Weaver.

## Task Metadata

New task files must include these front matter fields:

- `id`
- `title`
- `status`
- `category`
- `related_features`
- `owner`
- `created`
- `updated`

## Task Directories

- New task files live under `docs/tasks/<category>/`.
- Task filenames stay unchanged when tasks are moved into category directories.
- [README.md](./README.md) remains the root entry point and master index for all task files.

## Task Categories

Use one of these controlled `category` values for new tasks:

- `discovery`
- `validate`
- `format`
- `run`
- `rewrite`
- `prerequisite`
- `display-file`
- `example`
- `explain`
- `init`
- `import`
- `release`
- `repo-process`

These categories group tasks by feature or workstream without changing the
task history. Existing task files may be backfilled incrementally into
category-specific directories.

Expected task directories:

- `docs/tasks/discovery/`
- `docs/tasks/validate/`
- `docs/tasks/format/`
- `docs/tasks/run/`
- `docs/tasks/rewrite/`
- `docs/tasks/prerequisite/`
- `docs/tasks/display-file/`
- `docs/tasks/example/`
- `docs/tasks/explain/`
- `docs/tasks/init/`
- `docs/tasks/import/`
- `docs/tasks/release/`
- `docs/tasks/repo-process/`

## Task Files

- [TASK-001: Implement SPEC-001 Help Placeholder](discovery/TASK-001-implement-help-placeholder.md)
- [TASK-002: Implement SPEC-002 Validate Command](validate/TASK-002-implement-validate-command.md)
- [TASK-003: Implement SPEC-003 Run Command](run/TASK-003-implement-run-command.md)
- [TASK-004: Support Command Exit Assertions](run/TASK-004-support-command-exit-assertions.md)
- [TASK-005: Support Stdout Contains Assertions](run/TASK-005-support-stdout-contains-assertions.md)
- [TASK-006: Support Command Timeouts](run/TASK-006-support-command-timeouts.md)
- [TASK-007: Separate Runbook Execution and Rendering Modules](run/TASK-007-separate-runbook-execution-and-rendering-modules.md)
- [TASK-008: Support Command Cleanup](run/TASK-008-support-command-cleanup.md)
- [TASK-009: Support Output Content Types](run/TASK-009-support-output-content-types.md)
- [TASK-010: Support DisplayFile Entries](display-file/TASK-010-support-display-file-entries.md)
- [TASK-011: Add Generated File Marker](run/TASK-011-add-generated-file-marker.md)
- [TASK-012: Apply Command Indentation](run/TASK-012-apply-command-indentation.md)
- [TASK-013: Trim Command Output Trailing Whitespace](run/TASK-013-trim-command-output-trailing-whitespace.md)
- [TASK-014: Use Unlabeled Fences For Plain Output](run/TASK-014-use-unlabeled-fences-for-plain-output.md)
- [TASK-015: Support Output Rewrite Rules](rewrite/TASK-015-support-output-rewrite-rules.md)
- [TASK-016: Extend Datetime Shift Formats And Default Base](rewrite/TASK-016-extend-datetime-shift-formats-and-default-base.md)
- [TASK-017: Support Shared Datetime Shift Anchors And Custom Formats](rewrite/TASK-017-support-shared-datetime-shift-anchors-and-custom-formats.md)
- [TASK-018: Enforce Global Datetime Shift Anchor Uniqueness](rewrite/TASK-018-enforce-global-datetime-shift-anchor-uniqueness.md)
- [TASK-019: Support Cross-Command Datetime Anchor Reuse](rewrite/TASK-019-support-cross-command-datetime-anchor-reuse.md)
- [TASK-020: Support Captured Output Variables](run/TASK-020-support-captured-output-variables.md)
- [TASK-021: Support Deferred Markdown Interpolation](run/TASK-021-support-deferred-markdown-interpolation.md)
- [TASK-022: Support Markdown Interpolation For Earlier Captures](run/TASK-022-support-markdown-interpolation-for-earlier-captures.md)
- [TASK-023: Support Captured Variables In Rewrite Replacements](rewrite/TASK-023-support-captured-variables-in-rewrite-replacements.md)
- [TASK-024: Support Captured Variables In Rewrite Patterns](rewrite/TASK-024-support-captured-variables-in-rewrite-patterns.md)
- [TASK-025: Support Rewrite Generated Capture Pairs](rewrite/TASK-025-support-rewrite-generated-capture-pairs.md)
- [TASK-026: Change Default Output File To README Uppercase](run/TASK-026-change-default-output-file-to-readme-uppercase.md)
- [TASK-027: Support Prerequisites Entry Type](prerequisite/TASK-027-support-prerequisites-entry-type.md)
- [TASK-028: Implement Init Command](init/TASK-028-implement-init-command.md)
- [TASK-029: Support Keep Between Rewrite Rule](rewrite/TASK-029-support-keep-between-rewrite-rule.md)
- [TASK-030: Support Keep Between Trim Markers](rewrite/TASK-030-support-keep-between-trim-markers.md)
- [TASK-031: Rename Prerequisites Entry To Singular](prerequisite/TASK-031-rename-prerequisites-entry-to-singular.md)
- [TASK-032: Implement Check Command](prerequisite/TASK-032-implement-check-command.md)
- [TASK-033: Implement Import Command](import/TASK-033-implement-import-command.md)
- [TASK-034: Expand Help Command Coverage](discovery/TASK-034-expand-help-command-coverage.md)
- [TASK-035: Publish Official Release Assets](release/TASK-035-publish-official-release-assets.md)
- [TASK-036: Include Commit Subject In Release README](release/TASK-036-include-commit-subject-in-release-readme.md)
- [TASK-037: Support DisplayFile Line Ranges](display-file/TASK-037-support-display-file-line-ranges.md)
- [TASK-038: Include Release Commit History In README](release/TASK-038-include-release-commit-history-in-readme.md)
- [TASK-039: Implement Example Command](example/TASK-039-implement-example-command.md)
- [TASK-040: Expand Rewrite Example Coverage](rewrite/TASK-040-expand-rewrite-example-coverage.md)
- [TASK-041: Make Example Topics Case-Insensitive And Richer](example/TASK-041-make-example-topics-case-insensitive-and-richer.md)
- [TASK-042: Support Time-Only Datetime Shift](rewrite/TASK-042-support-time-only-datetime-shift.md)
- [TASK-043: Implement Explain Command](explain/TASK-043-implement-explain-command.md)
- [TASK-043: Include Failing Command Entry in Assertion Errors](run/TASK-043-include-failing-command-entry-in-assertion-errors.md)
- [TASK-044: Include Command Output In Assertion Errors](run/TASK-044-include-command-output-in-assertion-errors.md)
- [TASK-045: Support DisplayFile Indent Control](display-file/TASK-045-support-display-file-indent-control.md)
- [TASK-046: Support File Assertion Checks](run/TASK-046-support-file-assertion-checks.md)
- [TASK-047: Include Offending Entry In Human Validation Errors](validate/TASK-047-include-offending-entry-in-human-validation-errors.md)
- [TASK-048: Improve Human Validation Guidance](validate/TASK-048-improve-human-validation-guidance.md)
- [TASK-049: Make Keep Between End Optional](rewrite/TASK-049-make-keep-between-end-optional.md)
- [TASK-050: Show Keep Between Markers Only Where Trimmed](rewrite/TASK-050-show-keep-between-markers-only-where-trimmed.md)
- [TASK-051: Default To Automatic Command Process Cleanup](run/TASK-051-support-automatic-command-process-cleanup.md)
- [TASK-052: Support Built-In Java Prerequisite Checks](prerequisite/TASK-052-support-built-in-java-prerequisite-checks.md)
- [TASK-053: Treat Missing Process As No-op In Automatic Cleanup](run/TASK-053-treat-missing-process-as-no-op-in-automatic-cleanup.md)
- [TASK-054: Add Verbose Run Progress Output](run/TASK-054-add-verbose-run-progress-output.md)
- [TASK-055: Refine Verbose Run Time Formatting](run/TASK-055-refine-verbose-run-time-formatting.md)
- [TASK-056: Warn About Background Commands Holding Command Pipes Open](validate/TASK-056-warn-about-background-commands-holding-command-pipes-open.md)
- [TASK-057: Support Multiline Cleanup Control Structures](run/TASK-057-support-multiline-cleanup-control-structures.md)
- [TASK-058: Migrate Tasks Into Category Directories](repo-process/TASK-058-migrate-tasks-into-category-directories.md)
- [TASK-059: Implement Format Command](format/TASK-059-implement-format-command.md)
- [TASK-060: Improve Rewrite Capture Failure Diagnostics](rewrite/TASK-060-improve-rewrite-capture-failure-diagnostics.md)
- [TASK-061: Add Debug Run Diagnostics](run/TASK-061-add-debug-run-diagnostics.md)
- [TASK-062: Support Command-Scoped Debug Diagnostics](run/TASK-062-support-command-scoped-debug-diagnostics.md)
- [TASK-063: Separate DisplayFile Indent And Offset](display-file/TASK-063-separate-display-file-indent-and-offset.md)
- [TASK-064: Support Variable RFC3339 Fractional Seconds](rewrite/TASK-064-support-variable-rfc3339-fractional-seconds.md)
- [TASK-065: Add Patch Entry With Automatic Restore](run/TASK-065-add-patch-entry-with-automatic-restore.md)

## Pending

- [ ] [TASK-021: Support Deferred Markdown Interpolation](run/TASK-021-support-deferred-markdown-interpolation.md)
- [ ] [TASK-028: Implement Init Command](init/TASK-028-implement-init-command.md)
- [ ] [TASK-033: Implement Import Command](import/TASK-033-implement-import-command.md)
- [ ] [TASK-034: Expand Help Command Coverage](discovery/TASK-034-expand-help-command-coverage.md)
- [ ] [TASK-038: Include Release Commit History In README](release/TASK-038-include-release-commit-history-in-readme.md)
- [ ] [TASK-043: Implement Explain Command](explain/TASK-043-implement-explain-command.md)
- [ ] [TASK-059: Implement Format Command](format/TASK-059-implement-format-command.md)

## In Progress

- (none)

## Done

- [x] Define `SPEC-001` in `docs/spec/` (help and discovery).
- [x] Define `SPEC-002` in `docs/spec/` (validate runbook input).
- [x] Define `SPEC-003` in `docs/spec/` (run runbook to markdown).
- [x] [TASK-001: Implement SPEC-001 Help Placeholder](discovery/TASK-001-implement-help-placeholder.md)
- [x] [TASK-056: Warn About Background Commands Holding Command Pipes Open](validate/TASK-056-warn-about-background-commands-holding-command-pipes-open.md)
- [x] [TASK-002: Implement SPEC-002 Validate Command](validate/TASK-002-implement-validate-command.md)
- [x] [TASK-003: Implement SPEC-003 Run Command](run/TASK-003-implement-run-command.md)
- [x] [TASK-004: Support Command Exit Assertions](run/TASK-004-support-command-exit-assertions.md)
- [x] [TASK-005: Support Stdout Contains Assertions](run/TASK-005-support-stdout-contains-assertions.md)
- [x] [TASK-006: Support Command Timeouts](run/TASK-006-support-command-timeouts.md)
- [x] [TASK-007: Separate Runbook Execution and Rendering Modules](run/TASK-007-separate-runbook-execution-and-rendering-modules.md)
- [x] [TASK-008: Support Command Cleanup](run/TASK-008-support-command-cleanup.md)
- [x] [TASK-009: Support Output Content Types](run/TASK-009-support-output-content-types.md)
- [x] [TASK-010: Support DisplayFile Entries](display-file/TASK-010-support-display-file-entries.md)
- [x] [TASK-011: Add Generated File Marker](run/TASK-011-add-generated-file-marker.md)
- [x] [TASK-012: Apply Command Indentation](run/TASK-012-apply-command-indentation.md)
- [x] [TASK-013: Trim Command Output Trailing Whitespace](run/TASK-013-trim-command-output-trailing-whitespace.md)
- [x] [TASK-014: Use Unlabeled Fences For Plain Output](run/TASK-014-use-unlabeled-fences-for-plain-output.md)
- [x] [TASK-015: Support Output Rewrite Rules](rewrite/TASK-015-support-output-rewrite-rules.md)
- [x] [TASK-016: Extend Datetime Shift Formats And Default Base](rewrite/TASK-016-extend-datetime-shift-formats-and-default-base.md)
- [x] [TASK-017: Support Shared Datetime Shift Anchors And Custom Formats](rewrite/TASK-017-support-shared-datetime-shift-anchors-and-custom-formats.md)
- [x] [TASK-018: Enforce Global Datetime Shift Anchor Uniqueness](rewrite/TASK-018-enforce-global-datetime-shift-anchor-uniqueness.md)
- [x] [TASK-019: Support Cross-Command Datetime Anchor Reuse](rewrite/TASK-019-support-cross-command-datetime-anchor-reuse.md)
- [x] [TASK-020: Support Captured Output Variables](run/TASK-020-support-captured-output-variables.md)
- [x] [TASK-022: Support Markdown Interpolation For Earlier Captures](run/TASK-022-support-markdown-interpolation-for-earlier-captures.md)
- [x] [TASK-023: Support Captured Variables In Rewrite Replacements](rewrite/TASK-023-support-captured-variables-in-rewrite-replacements.md)
- [x] [TASK-024: Support Captured Variables In Rewrite Patterns](rewrite/TASK-024-support-captured-variables-in-rewrite-patterns.md)
- [x] [TASK-025: Support Rewrite Generated Capture Pairs](rewrite/TASK-025-support-rewrite-generated-capture-pairs.md)
- [x] [TASK-026: Change Default Output File To README Uppercase](run/TASK-026-change-default-output-file-to-readme-uppercase.md)
- [x] [TASK-027: Support Prerequisites Entry Type](prerequisite/TASK-027-support-prerequisites-entry-type.md)
- [x] [TASK-029: Support Keep Between Rewrite Rule](rewrite/TASK-029-support-keep-between-rewrite-rule.md)
- [x] [TASK-030: Support Keep Between Trim Markers](rewrite/TASK-030-support-keep-between-trim-markers.md)
- [x] [TASK-031: Rename Prerequisites Entry To Singular](prerequisite/TASK-031-rename-prerequisites-entry-to-singular.md)
- [x] [TASK-032: Implement Check Command](prerequisite/TASK-032-implement-check-command.md)
- [x] [TASK-035: Publish Official Release Assets](release/TASK-035-publish-official-release-assets.md)
- [x] [TASK-036: Include Commit Subject In Release README](release/TASK-036-include-commit-subject-in-release-readme.md)
- [x] [TASK-037: Support DisplayFile Line Ranges](display-file/TASK-037-support-display-file-line-ranges.md)
- [x] [TASK-039: Implement Example Command](example/TASK-039-implement-example-command.md)
- [x] [TASK-040: Expand Rewrite Example Coverage](rewrite/TASK-040-expand-rewrite-example-coverage.md)
- [x] [TASK-041: Make Example Topics Case-Insensitive And Richer](example/TASK-041-make-example-topics-case-insensitive-and-richer.md)
- [x] [TASK-042: Support Time-Only Datetime Shift](rewrite/TASK-042-support-time-only-datetime-shift.md)
- [x] [TASK-043: Include Failing Command Entry in Assertion Errors](run/TASK-043-include-failing-command-entry-in-assertion-errors.md)
- [x] [TASK-044: Include Command Output In Assertion Errors](run/TASK-044-include-command-output-in-assertion-errors.md)
- [x] [TASK-045: Support DisplayFile Indent Control](display-file/TASK-045-support-display-file-indent-control.md)
- [x] [TASK-046: Support File Assertion Checks](run/TASK-046-support-file-assertion-checks.md)
- [x] [TASK-047: Include Offending Entry In Human Validation Errors](validate/TASK-047-include-offending-entry-in-human-validation-errors.md)
- [x] [TASK-048: Improve Human Validation Guidance](validate/TASK-048-improve-human-validation-guidance.md)
- [x] [TASK-049: Make Keep Between End Optional](rewrite/TASK-049-make-keep-between-end-optional.md)
- [x] [TASK-050: Show Keep Between Markers Only Where Trimmed](rewrite/TASK-050-show-keep-between-markers-only-where-trimmed.md)
- [x] [TASK-051: Default To Automatic Command Process Cleanup](run/TASK-051-support-automatic-command-process-cleanup.md)
- [x] [TASK-052: Support Built-In Java Prerequisite Checks](prerequisite/TASK-052-support-built-in-java-prerequisite-checks.md)
- [x] [TASK-053: Treat Missing Process As No-op In Automatic Cleanup](run/TASK-053-treat-missing-process-as-no-op-in-automatic-cleanup.md)
- [x] [TASK-054: Add Verbose Run Progress Output](run/TASK-054-add-verbose-run-progress-output.md)
- [x] [TASK-055: Refine Verbose Run Time Formatting](run/TASK-055-refine-verbose-run-time-formatting.md)
- [x] [TASK-057: Support Multiline Cleanup Control Structures](run/TASK-057-support-multiline-cleanup-control-structures.md)
- [x] [TASK-058: Migrate Tasks Into Category Directories](repo-process/TASK-058-migrate-tasks-into-category-directories.md)
- [x] [TASK-060: Improve Rewrite Capture Failure Diagnostics](rewrite/TASK-060-improve-rewrite-capture-failure-diagnostics.md)
- [x] [TASK-061: Add Debug Run Diagnostics](run/TASK-061-add-debug-run-diagnostics.md)
- [x] [TASK-062: Support Command-Scoped Debug Diagnostics](run/TASK-062-support-command-scoped-debug-diagnostics.md)
- [x] [TASK-063: Separate DisplayFile Indent And Offset](display-file/TASK-063-separate-display-file-indent-and-offset.md)
- [x] [TASK-064: Support Variable RFC3339 Fractional Seconds](rewrite/TASK-064-support-variable-rfc3339-fractional-seconds.md)
- [x] [TASK-065: Add Patch Entry With Automatic Restore](run/TASK-065-add-patch-entry-with-automatic-restore.md)

## Blocked

- (none)
