# SPEC-004: Init Runbook Sample

- Status: Draft
- Owner: `@aattard`
- Created: `2026-03-12`
- Updated: `2026-03-12`

## Goal

Provide a command that generates a starter `sw-runbook.json` file so users can
begin authoring runbooks from a realistic example instead of starting from an
empty document.

## User-facing Behavior

The new command is:

```bash
sw init
```

It creates a sample runbook in the current directory.

## Inputs

- Optional output file parameter: `--output-file <path>`.
- Optional force flag: `--force`.

## CLI Defaults

- If `--output-file` is not provided, default to `./sw-runbook.json`.
- If `--force` is not provided, existing files are not overwritten.

## Outputs

- A sample runbook JSON file written to the target path.
- Human-readable status on stdout.

### Exit Codes

- `0` when the sample file is created successfully.
- `1` when the command cannot write the file or the target already exists
  without `--force`.

## Generated Sample Contract

- The generated file is valid JSON.
- The generated file is intended as a realistic authoring example, not as the
  smallest possible document.
- The generated file includes one example of each supported entry type in this
  increment:
  - `Heading`
  - `Markdown`
  - `DisplayFile`
  - `Prerequisites`
  - `Command`
- The sample demonstrates common options and workflow features, including:
  - command output rendering
  - assertions
  - cleanup
  - timeout
  - output rewrite
  - captured variables
- The sample uses safe placeholder values and does not depend on a specific
  user machine.

## Acceptance Criteria

- [ ] Given `sw init` in a directory without `sw-runbook.json`, the command
      writes `./sw-runbook.json` and exits with `0`.
- [ ] Given `sw init --output-file <path>`, the command writes the sample to
      the provided path.
- [ ] Given `sw init` when the target file already exists, the command exits
      with `1` and does not overwrite the file.
- [ ] Given `sw init --force` when the target file already exists, the command
      overwrites the file and exits with `0`.
- [ ] The generated sample file is valid according to `sw validate`.
- [ ] The generated sample includes one example of each supported entry type in
      this increment.

## Non-goals

- Generating multiple template families in the first increment.
- Detecting the current project type and tailoring the sample automatically.
- Creating additional files beyond the requested runbook.

## Edge Cases

- Target output file already exists.
- Parent directory of the output path does not exist.
- Output path is unwritable.
- User requests a custom output file path.
- User requests overwrite with `--force`.
