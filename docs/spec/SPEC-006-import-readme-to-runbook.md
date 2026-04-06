# SPEC-006: Import README to Runbook

- Status: Implemented
- Owner: `@aattard`
- Created: `2026-03-13`
- Updated: `2026-04-02`

## Goal

Provide a command that imports an existing Markdown README file into a starter
`sw-runbook.json` so teams can convert existing documentation into an editable
runbook instead of authoring one from scratch.

## User-facing Behavior

The new command is:

```bash
sw import
```

It reads a Markdown document and writes a best-effort runbook JSON file.

## Inputs

- Optional input file parameter: `--input-file <README.md>`.
- Optional output file parameter: `--output-file <sw-runbook.json>`.
- Optional force flag: `--force`.

## CLI Defaults

- If `--input-file` is not provided, default to `./README.md`.
- If `--output-file` is not provided, default to `./sw-runbook.json`.
- If `--force` is not provided, existing output files are not overwritten.

## Outputs

- A runbook JSON file written to the target path.
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
- The generated runbook must be valid JSON and acceptable to `sw validate`.
- Heading blocks are imported as `Heading` entries where possible.
- Plain Markdown prose is imported as `Markdown` entries.
- Fenced shell code blocks are imported as `Command` entries.
- Markdown content that cannot be mapped to a richer runbook entry type is kept
  as `Markdown`.
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
      `./sw-runbook.json` and exits with `0`.
- [x] Given `sw import --input-file <path> --output-file <path>`, the command
      reads the provided Markdown file and writes the runbook to the provided
      output path.
- [x] Given `sw import` when the target output file already exists, the command
      exits with `1` and does not overwrite the file.
- [x] Given `sw import --force` when the target output file already exists, the
      command overwrites the file and exits with `0`.
- [x] Given a README with Markdown headings, prose, and fenced shell blocks,
      the generated runbook contains corresponding `Heading`, `Markdown`, and
      `Command` entries.
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
