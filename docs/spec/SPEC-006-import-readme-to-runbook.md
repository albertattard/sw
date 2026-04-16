# SPEC-006: Import README to Runbook

- Status: Implemented
- Owner: `@aattard`
- Created: `2026-03-13`
- Updated: `2026-04-16`

## Goal

Provide a command that imports an existing Markdown README file into a starter
runbook file so teams can convert existing documentation into an editable
runbook instead of authoring one from scratch.

## User-facing Behavior

The new command is:

```bash
sw import
```

It reads a Markdown document and writes a best-effort runbook YAML or JSON
file.

## Inputs

- Optional input file parameter: `--input-file <README.md>`.
- Optional output file parameter: `--output-file <sw-runbook.{yaml|yml|json}>`.
- Optional output format parameter: `--output-format json|yaml`.
- Optional force flag: `--force`.

## CLI Defaults

- If `--input-file` is not provided, default to `./README.md`.
- If `--output-format` is not provided, default to `yaml`.
- `sw import` establishes YAML as the default file-based import format.
- If `--output-file` is not provided, default to `./sw-runbook.yaml` for YAML
  output and `./sw-runbook.json` for JSON output.
- If `--output-file` has a `.json`, `.yaml`, or `.yml` extension and
  `--output-format` is not provided, infer the output format from the file
  extension.
- If `--output-format` is provided and `--output-file` has a recognized
  extension that conflicts with it, the command exits with `1` and reports a
  clear mismatch error.
- If `--force` is not provided, existing output files are not overwritten.

## Outputs

- A runbook YAML or JSON file written to the target path.
- Human-readable status on stdout.

### Exit Codes

- `0` when the import succeeds and writes a runbook file.
- `1` when the command cannot read the input file, cannot write the output
  file, the target already exists without `--force`, or an operational error
  occurs.

## Import Contract

- Import is intentionally lossy.
- The generated runbook is a starting point for further editing, not a perfect
  round-trip reconstruction.
- The generated runbook must be valid YAML or JSON and acceptable to
  `sw validate`.
- Heading blocks are imported as `Heading` entries where possible.
- Plain Markdown prose is imported as `Markdown` entries.
- Fenced shell code blocks are imported as `Command` entries.
- Markdown content that cannot be mapped to a richer runbook entry type is kept
  as `Markdown`.
- Imported entry objects serialize `type` before the other entry-specific
  fields to keep the generated runbook easier to scan and edit.
- This ordering rule applies to all runbook entry types that `sw import`
  emits, including future imported entry types added in later increments.
- This field-ordering rule applies to `sw import` output only; other machine-
  readable output remains governed by their own contracts.
- YAML output from `sw import` indents sequence items by two spaces beneath the
  owning key, including the top-level `entries` list as `entries:\n  - ...`.
- YAML output from `sw import` inserts a blank line between adjacent items in
  the top-level `entries` list so individual imported entries are easier to
  scan and edit.
- YAML output from `sw import` emits imported multi-line prose fields such as
  `Markdown.contents` as literal block scalars using `|` instead of explicit
  line arrays.
- This YAML-friendly formatting contract aligns with the repository's
  canonical YAML authoring style rather than using a one-off import-specific
  layout.
- Information not recoverable from the README is omitted and left to defaults
  or later manual enhancement, including:
  - `assert`
  - `cleanup`
  - `timeout`
  - `capture`
  - `rewrite`
  - prerequisite execution metadata unless explicitly inferable
- The importer should favor stable, editable output over aggressive inference.

## Acceptance Criteria

- [x] Given `sw import` with a `./README.md` present, the command writes
      `./sw-runbook.yaml` and exits with `0`.
- [x] Given `sw import --input-file <path> --output-file <path>`, the command
      reads the provided Markdown file and writes the runbook to the provided
      output path.
- [ ] Given `sw import --output-format json`, the command writes
      `./sw-runbook.json` and exits with `0`.
- [ ] Given `sw import --output-file <path>.json` without `--output-format`,
      the command writes JSON to that output path.
- [ ] Given `sw import --output-file <path>.yaml` without `--output-format`,
      the command writes YAML to that output path.
- [ ] Given `sw import --output-format json --output-file <path>.yaml`, the
      command exits with `1` and reports the format mismatch.
- [x] Given `sw import` when the target output file already exists, the command
      exits with `1` and does not overwrite the file.
- [x] Given `sw import --force` when the target output file already exists, the
      command overwrites the file and exits with `0`.
- [x] Given a README with Markdown headings, prose, and fenced shell blocks,
      the generated runbook contains corresponding `Heading`, `Markdown`, and
      `Command` entries.
- [x] Given an imported runbook entry of any type that `sw import` emits, the
      serialized output places `type` before the other entry fields.
- [x] Given `sw import` YAML output with multiple imported entries, the
      serialized `entries` list uses `entries:\n  - ...` indentation.
- [x] Given `sw import` YAML output with multiple imported entries, the
      serialized `entries` list includes a blank line between adjacent entry
      items.
- [x] Given imported multi-line Markdown prose in YAML output, the serialized
      `contents` field uses a literal block scalar introduced with `|`.
- [x] The generated runbook is valid according to `sw validate`.

## Non-goals

- Perfect round-trip reconstruction from README back to runbook.
- Recovering runbook-only execution semantics from rendered Markdown.
- Detecting project-specific conventions beyond the supported Markdown import
  rules in this increment.

## Edge Cases

- Input README file does not exist.
- Output file already exists.
- README contains fenced code blocks without a recognized shell language tag.
- README contains non-shell fenced code blocks.
- README contains mixed prose and lists that must remain as `Markdown`.
- README contains formatting that cannot be mapped cleanly to richer runbook
  entries.
