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
- [TASK-066: Add Patch Example Snippet](example/TASK-066-add-patch-example-snippet.md)
- [TASK-067: Add DisplayFile Transformations](display-file/TASK-067-add-display-file-transformations.md)
- [TASK-068: Add Skill Output Format To Explain Command](explain/TASK-068-add-skill-output-format-to-explain-command.md)
- [TASK-069: Support YAML Runbook Input](run/TASK-069-support-yaml-runbook-input.md)
- [TASK-070: Support Java Command Output Content Type](run/TASK-070-support-java-command-output-content-type.md)
- [TASK-071: Prefer Datetime Shift Guidance In Explain](explain/TASK-071-prefer-datetime-shift-guidance-in-explain.md)
- [TASK-072: Support Output Empty Line Trimming](run/TASK-072-support-output-empty-line-trimming.md)
- [TASK-073: Surface Runbook Output Field Discovery In Help](discovery/TASK-073-surface-runbook-output-field-discovery-in-help.md)
- [TASK-074: Document Output Empty Line Trimming In Explain](explain/TASK-074-document-output-empty-line-trimming-in-explain.md)
- [TASK-075: Make Output Empty Line Trimming Default](run/TASK-075-make-output-empty-line-trimming-default.md)
- [TASK-076: Surface DisplayFile Transform Discovery In Help](discovery/TASK-076-surface-display-file-transform-discovery-in-help.md)
- [TASK-077: Document DisplayFile Transform Discovery In Explain](explain/TASK-077-document-display-file-transform-discovery-in-explain.md)
- [TASK-078: Support Stdin Runbook Input](run/TASK-078-support-stdin-runbook-input.md)
- [TASK-079: Support Command Output Stream Selection](run/TASK-079-support-command-output-stream-selection.md)
- [TASK-080: Make Patch Application Non-Interactive](run/TASK-080-make-patch-application-non-interactive.md)
- [TASK-081: Add Skill Frontmatter To Explain Export](explain/TASK-081-add-skill-frontmatter-to-explain-export.md)
- [TASK-082: Minimize Skill Export To Discovery Entrypoint](explain/TASK-082-minimize-skill-export-to-discovery-entrypoint.md)
- [TASK-083: Default Command Prerequisite Timeouts To Five Seconds](prerequisite/TASK-083-default-command-prerequisite-timeouts-to-five-seconds.md)
- [TASK-084: Add Version Discovery Output](discovery/TASK-084-add-version-discovery-output.md)
- [TASK-085: Add Selectable Verbose Progress Modes](run/TASK-085-add-selectable-verbose-progress-modes.md)
- [TASK-086: Make Combined The Default Command Output Stream](run/TASK-086-make-combined-the-default-command-output-stream.md)
- [TASK-087: Support RFC3339 Zulu Datetime Shift](rewrite/TASK-087-support-rfc3339-zulu-datetime-shift.md)
- [TASK-088: Add Rust Quality And Dependency Hygiene Automation](repo-process/TASK-088-add-rust-quality-and-dependency-hygiene-automation.md)
- [TASK-089: Replace Deprecated Serde YAML Dependency](repo-process/TASK-089-replace-deprecated-serde-yaml-dependency.md)
- [TASK-090: Recognize SQL DisplayFile Fences](display-file/TASK-090-recognize-sql-display-file-fences.md)
- [TASK-091: Recognize XML DisplayFile Fences](display-file/TASK-091-recognize-xml-display-file-fences.md)
- [TASK-092: Place Imported Entry Types First](import/TASK-092-place-imported-entry-types-first.md)
- [TASK-093: Declare Package License Metadata](repo-process/TASK-093-declare-package-license-metadata.md)
- [TASK-094: Accept Scalar Prose Contents](validate/TASK-094-accept-scalar-prose-contents.md)
- [TASK-095: Surface DisplayFile Indent In Example Output](example/TASK-095-surface-display-file-indent-in-example.md)
- [TASK-096: Trim Scalar Output Caption Terminator Blank Lines](run/TASK-096-trim-scalar-output-caption-terminator-blank-lines.md)
- [TASK-097: Add YAML Import Output Format](import/TASK-097-add-yaml-import-output-format.md)
- [TASK-098: Format Imported YAML For Editing](import/TASK-098-format-imported-yaml-for-editing.md)
- [TASK-099: Accept Scalar Command Scripts](run/TASK-099-accept-scalar-command-scripts.md)
- [TASK-100: Add Local Verification Tool](repo-process/TASK-100-add-fail-fast-verification-script.md)
- [TASK-101: Upgrade sha2 To 0.11](repo-process/TASK-101-upgrade-sha2-to-0-11.md)
- [TASK-102: Review actions checkout v6 Upgrade](repo-process/TASK-102-review-actions-checkout-v6-upgrade.md)
- [TASK-103: Review upload artifact v7 Upgrade](repo-process/TASK-103-review-upload-artifact-v7-upgrade.md)
- [TASK-104: Default Example Output To YAML](example/TASK-104-default-example-output-to-yaml.md)
- [TASK-105: Accept Scalar Cleanup Scripts](run/TASK-105-accept-scalar-cleanup-scripts.md)
- [TASK-106: Move Release Matrix To Tagged Releases](release/TASK-106-move-release-matrix-to-tagged-releases.md)
- [TASK-107: Add Supported Features To First Release README](release/TASK-107-add-supported-features-to-first-release-readme.md)
- [TASK-108: Support Command Working Directory](run/TASK-108-support-command-working-directory.md)
- [TASK-109: Support HTML Command Output Content Type](run/TASK-109-support-html-command-output-content-type.md)
- [TASK-110: Render Working Dir As Copyable Subshell](run/TASK-110-render-working-dir-as-copyable-subshell.md)
- [TASK-111: Reject Ambiguous Default Runbook Selection](discovery/TASK-111-reject-ambiguous-default-runbook-selection.md)
- [TASK-112: Surface Cleanup Discovery In Help And Explain](discovery/TASK-112-surface-cleanup-discovery-in-help-and-explain.md)
- [TASK-113: Default Init Output To YAML](init/TASK-113-default-init-output-to-yaml.md)
- [TASK-114: Clarify Format Defaults By Workflow](discovery/TASK-114-clarify-format-defaults-by-workflow.md)
- [TASK-115: Support Markdown Indent](run/TASK-115-support-markdown-indent.md)
- [TASK-116: Add Blank Lines Between Formatted YAML Entries](format/TASK-116-add-blank-lines-between-formatted-yaml-entries.md)
- [TASK-117: Standardize Indented YAML Sequences](format/TASK-117-standardize-indented-yaml-sequences.md)
- [TASK-118: Implement Convert Command](format/TASK-118-implement-convert-command.md)
- [TASK-119: Modernize Prerequisite Example Snippet](example/TASK-119-modernize-prerequisite-example-snippet.md)
- [TASK-120: Link Generated Marker To Project Repository](run/TASK-120-link-generated-marker-to-project-repository.md)
- [TASK-121: Support Command Preconditions And Port Checks](run/TASK-121-support-command-preconditions-and-port-checks.md)
- [TASK-122: Shorten Default Command Timeout](run/TASK-122-shorten-default-command-timeout.md)
- [TASK-123: Surface Markdown Capture Interpolation In Help And Explain](discovery/TASK-123-surface-markdown-capture-interpolation-in-help-and-explain.md)
- [TASK-124: Apply Command Timeout To Full Entry Lifecycle](run/TASK-124-apply-command-timeout-to-full-entry-lifecycle.md)
- [TASK-125: Normalize Scalar-Capable Arrays During JSON To YAML Convert](format/TASK-125-normalize-scalar-capable-arrays-during-json-to-yaml-convert.md)
- [TASK-126: Surface Command Debug Discovery In Explain And Example](discovery/TASK-126-surface-command-debug-discovery-in-explain-and-example.md)
- [TASK-127: Support Numeric Captures And Markdown Arithmetic](run/TASK-127-support-numeric-captures-and-markdown-arithmetic.md)
- [TASK-128: Surface Datetime Rewrite Reuse Discovery](discovery/TASK-128-surface-datetime-rewrite-reuse-discovery.md)
- [TASK-129: Add Human Guide Documentation Structure](discovery/TASK-129-add-human-guide-documentation-structure.md)
- [TASK-130: Add Runbook Entity Editing Guide](discovery/TASK-130-add-runbook-entity-editing-guide.md)

## Pending

- [ ] [TASK-127: Support Numeric Captures And Markdown Arithmetic](run/TASK-127-support-numeric-captures-and-markdown-arithmetic.md)
- [ ] [TASK-121: Support Command Preconditions And Port Checks](run/TASK-121-support-command-preconditions-and-port-checks.md)

## In Progress

- (none)

## Done

- [x] [TASK-130: Add Runbook Entity Editing Guide](discovery/TASK-130-add-runbook-entity-editing-guide.md)
- [x] [TASK-129: Add Human Guide Documentation Structure](discovery/TASK-129-add-human-guide-documentation-structure.md)
- [x] [TASK-128: Surface Datetime Rewrite Reuse Discovery](discovery/TASK-128-surface-datetime-rewrite-reuse-discovery.md)
- [x] [TASK-126: Surface Command Debug Discovery In Explain And Example](discovery/TASK-126-surface-command-debug-discovery-in-explain-and-example.md)
- [x] [TASK-125: Normalize Scalar-Capable Arrays During JSON To YAML Convert](format/TASK-125-normalize-scalar-capable-arrays-during-json-to-yaml-convert.md)
- [x] [TASK-124: Apply Command Timeout To Full Entry Lifecycle](run/TASK-124-apply-command-timeout-to-full-entry-lifecycle.md)
- [x] [TASK-123: Surface Markdown Capture Interpolation In Help And Explain](discovery/TASK-123-surface-markdown-capture-interpolation-in-help-and-explain.md)
- [x] [TASK-122: Shorten Default Command Timeout](run/TASK-122-shorten-default-command-timeout.md)
- [x] [TASK-120: Link Generated Marker To Project Repository](run/TASK-120-link-generated-marker-to-project-repository.md)
- [x] [TASK-119: Modernize Prerequisite Example Snippet](example/TASK-119-modernize-prerequisite-example-snippet.md)
- [x] [TASK-118: Implement Convert Command](format/TASK-118-implement-convert-command.md)
- [x] [TASK-117: Standardize Indented YAML Sequences](format/TASK-117-standardize-indented-yaml-sequences.md)
- [x] [TASK-116: Add Blank Lines Between Formatted YAML Entries](format/TASK-116-add-blank-lines-between-formatted-yaml-entries.md)
- [x] [TASK-115: Support Markdown Indent](run/TASK-115-support-markdown-indent.md)
- [x] [TASK-059: Implement Format Command](format/TASK-059-implement-format-command.md)
- [x] [TASK-114: Clarify Format Defaults By Workflow](discovery/TASK-114-clarify-format-defaults-by-workflow.md)
- [x] [TASK-113: Default Init Output To YAML](init/TASK-113-default-init-output-to-yaml.md)
- [x] [TASK-028: Implement Init Command](init/TASK-028-implement-init-command.md)
- [x] [TASK-112: Surface Cleanup Discovery In Help And Explain](discovery/TASK-112-surface-cleanup-discovery-in-help-and-explain.md)
- [x] [TASK-111: Reject Ambiguous Default Runbook Selection](discovery/TASK-111-reject-ambiguous-default-runbook-selection.md)
- [x] [TASK-110: Render Working Dir As Copyable Subshell](run/TASK-110-render-working-dir-as-copyable-subshell.md)
- [x] [TASK-109: Support HTML Command Output Content Type](run/TASK-109-support-html-command-output-content-type.md)
- [x] [TASK-108: Support Command Working Directory](run/TASK-108-support-command-working-directory.md)
- [x] [TASK-107: Add Supported Features To First Release README](release/TASK-107-add-supported-features-to-first-release-readme.md)
- [x] [TASK-038: Include Release Commit History In README](release/TASK-038-include-release-commit-history-in-readme.md)
- [x] [TASK-106: Move Release Matrix To Tagged Releases](release/TASK-106-move-release-matrix-to-tagged-releases.md)
- [x] [TASK-105: Accept Scalar Cleanup Scripts](run/TASK-105-accept-scalar-cleanup-scripts.md)

- [x] Define `SPEC-001` in `docs/spec/` (help and discovery).
- [x] Define `SPEC-002` in `docs/spec/` (validate runbook input).
- [x] Define `SPEC-003` in `docs/spec/` (run runbook to markdown).
- [x] [TASK-021: Support Deferred Markdown Interpolation](run/TASK-021-support-deferred-markdown-interpolation.md)
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
- [x] [TASK-033: Implement Import Command](import/TASK-033-implement-import-command.md)
- [x] [TASK-034: Expand Help Command Coverage](discovery/TASK-034-expand-help-command-coverage.md)
- [x] [TASK-035: Publish Official Release Assets](release/TASK-035-publish-official-release-assets.md)
- [x] [TASK-036: Include Commit Subject In Release README](release/TASK-036-include-commit-subject-in-release-readme.md)
- [x] [TASK-037: Support DisplayFile Line Ranges](display-file/TASK-037-support-display-file-line-ranges.md)
- [x] [TASK-039: Implement Example Command](example/TASK-039-implement-example-command.md)
- [x] [TASK-040: Expand Rewrite Example Coverage](rewrite/TASK-040-expand-rewrite-example-coverage.md)
- [x] [TASK-041: Make Example Topics Case-Insensitive And Richer](example/TASK-041-make-example-topics-case-insensitive-and-richer.md)
- [x] [TASK-042: Support Time-Only Datetime Shift](rewrite/TASK-042-support-time-only-datetime-shift.md)
- [x] [TASK-043: Implement Explain Command](explain/TASK-043-implement-explain-command.md)
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
- [x] [TASK-083: Default Command Prerequisite Timeouts To Five Seconds](prerequisite/TASK-083-default-command-prerequisite-timeouts-to-five-seconds.md)
- [x] [TASK-084: Add Version Discovery Output](discovery/TASK-084-add-version-discovery-output.md)
- [x] [TASK-085: Add Selectable Verbose Progress Modes](run/TASK-085-add-selectable-verbose-progress-modes.md)
- [x] [TASK-086: Make Combined The Default Command Output Stream](run/TASK-086-make-combined-the-default-command-output-stream.md)
- [x] [TASK-087: Support RFC3339 Zulu Datetime Shift](rewrite/TASK-087-support-rfc3339-zulu-datetime-shift.md)
- [x] [TASK-088: Add Rust Quality And Dependency Hygiene Automation](repo-process/TASK-088-add-rust-quality-and-dependency-hygiene-automation.md)
- [x] [TASK-089: Replace Deprecated Serde YAML Dependency](repo-process/TASK-089-replace-deprecated-serde-yaml-dependency.md)
- [x] [TASK-090: Recognize SQL DisplayFile Fences](display-file/TASK-090-recognize-sql-display-file-fences.md)
- [x] [TASK-091: Recognize XML DisplayFile Fences](display-file/TASK-091-recognize-xml-display-file-fences.md)
- [x] [TASK-092: Place Imported Entry Types First](import/TASK-092-place-imported-entry-types-first.md)
- [x] [TASK-093: Declare Package License Metadata](repo-process/TASK-093-declare-package-license-metadata.md)
- [x] [TASK-094: Accept Scalar Prose Contents](validate/TASK-094-accept-scalar-prose-contents.md)
- [x] [TASK-095: Surface DisplayFile Indent In Example Output](example/TASK-095-surface-display-file-indent-in-example.md)
- [x] [TASK-096: Trim Scalar Output Caption Terminator Blank Lines](run/TASK-096-trim-scalar-output-caption-terminator-blank-lines.md)
- [x] [TASK-097: Add YAML Import Output Format](import/TASK-097-add-yaml-import-output-format.md)
- [x] [TASK-098: Format Imported YAML For Editing](import/TASK-098-format-imported-yaml-for-editing.md)
- [x] [TASK-099: Accept Scalar Command Scripts](run/TASK-099-accept-scalar-command-scripts.md)
- [x] [TASK-100: Add Local Verification Tool](repo-process/TASK-100-add-fail-fast-verification-script.md)
- [x] [TASK-101: Upgrade sha2 To 0.11](repo-process/TASK-101-upgrade-sha2-to-0-11.md)
- [x] [TASK-102: Review actions checkout v6 Upgrade](repo-process/TASK-102-review-actions-checkout-v6-upgrade.md)
- [x] [TASK-103: Review upload artifact v7 Upgrade](repo-process/TASK-103-review-upload-artifact-v7-upgrade.md)
- [x] [TASK-104: Default Example Output To YAML](example/TASK-104-default-example-output-to-yaml.md)
- [x] [TASK-061: Add Debug Run Diagnostics](run/TASK-061-add-debug-run-diagnostics.md)
- [x] [TASK-062: Support Command-Scoped Debug Diagnostics](run/TASK-062-support-command-scoped-debug-diagnostics.md)
- [x] [TASK-063: Separate DisplayFile Indent And Offset](display-file/TASK-063-separate-display-file-indent-and-offset.md)
- [x] [TASK-064: Support Variable RFC3339 Fractional Seconds](rewrite/TASK-064-support-variable-rfc3339-fractional-seconds.md)
- [x] [TASK-065: Add Patch Entry With Automatic Restore](run/TASK-065-add-patch-entry-with-automatic-restore.md)
- [x] [TASK-066: Add Patch Example Snippet](example/TASK-066-add-patch-example-snippet.md)
- [x] [TASK-067: Add DisplayFile Transformations](display-file/TASK-067-add-display-file-transformations.md)
- [x] [TASK-068: Add Skill Output Format To Explain Command](explain/TASK-068-add-skill-output-format-to-explain-command.md)
- [x] [TASK-069: Support YAML Runbook Input](run/TASK-069-support-yaml-runbook-input.md)
- [x] [TASK-070: Support Java Command Output Content Type](run/TASK-070-support-java-command-output-content-type.md)
- [x] [TASK-071: Prefer Datetime Shift Guidance In Explain](explain/TASK-071-prefer-datetime-shift-guidance-in-explain.md)
- [x] [TASK-072: Support Output Empty Line Trimming](run/TASK-072-support-output-empty-line-trimming.md)
- [x] [TASK-073: Surface Runbook Output Field Discovery In Help](discovery/TASK-073-surface-runbook-output-field-discovery-in-help.md)
- [x] [TASK-074: Document Output Empty Line Trimming In Explain](explain/TASK-074-document-output-empty-line-trimming-in-explain.md)
- [x] [TASK-075: Make Output Empty Line Trimming Default](run/TASK-075-make-output-empty-line-trimming-default.md)
- [x] [TASK-076: Surface DisplayFile Transform Discovery In Help](discovery/TASK-076-surface-display-file-transform-discovery-in-help.md)
- [x] [TASK-077: Document DisplayFile Transform Discovery In Explain](explain/TASK-077-document-display-file-transform-discovery-in-explain.md)
- [x] [TASK-078: Support Stdin Runbook Input](run/TASK-078-support-stdin-runbook-input.md)
- [x] [TASK-079: Support Command Output Stream Selection](run/TASK-079-support-command-output-stream-selection.md)
- [x] [TASK-080: Make Patch Application Non-Interactive](run/TASK-080-make-patch-application-non-interactive.md)
- [x] [TASK-081: Add Skill Frontmatter To Explain Export](explain/TASK-081-add-skill-frontmatter-to-explain-export.md)
- [x] [TASK-082: Minimize Skill Export To Discovery Entrypoint](explain/TASK-082-minimize-skill-export-to-discovery-entrypoint.md)

## Blocked

- (none)
