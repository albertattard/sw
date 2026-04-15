# SPEC-008: Example Snippets Contract

- Status: Draft
- Owner: `@aattard`
- Created: `2026-03-13`
- Updated: `2026-04-15`

## Goal

Provide a simple way for users to request focused examples for supported
runbook entry types without generating a full starter runbook, while making
YAML the default snippet format for file-based authoring and preserving JSON as
an explicit machine-oriented option.

## User-facing Behavior

The CLI provides an explicit `example` command:

```bash
sw example <entity-type>
sw example <entity-type> --output-format json
sw example <entity-type> --output-format yaml
```

This command prints a runbook snippet to stdout for the requested entity type.

The snippet is intended as a copyable starting point.

## Inputs

- Required entity-type argument: `<entity-type>`
- Optional output format argument: `--output-format yaml|json`

In this increment, supported entity types include:
- `Command`
- `DisplayFile`
- `Patch`
- `Prerequisite`

## Outputs

- YAML snippet written to stdout by default
- JSON snippet written to stdout when `--output-format json` is selected
- Human-readable error on stderr for unsupported entity types

### Exit Codes

- `0`: example was found and printed successfully
- `1`: operational error or unknown entity type

## Example Contract

- Default example output should be valid YAML.
- `--output-format json` should produce valid JSON.
- `--output-format yaml` should produce valid YAML and behave the same as the
  default output mode.
- Entity-type matching is case-insensitive.
- Entry-type examples should print a single runbook entry representing that
  entry and should include the commonly used nested properties for that entry
  type so users can remove what they do not need.
- YAML example output should remain hand-editable and should prefer the same
  editing-oriented conventions used elsewhere in this repository, including
  YAML mappings and sequences that read naturally in a runbook file.
- The `Command` example should reflect the current output contract, including
  supported output fields such as `stream` and `trim_empty_lines` when
  available.
- When the `Command` example includes `output.stream`, it should use the
  current default value so users and agents can start from the default
  behavior.
- The `DisplayFile` example should reflect the current transform contract,
  including Java `collapse_method_body` when implemented.
- The `DisplayFile` example should reflect the current block-indentation
  contract by including `indent` when that field is supported.
- Example output is documentation-oriented and does not need to be executable
  without further user editing.

## Acceptance Criteria

- [ ] Given `sw example Command`, the CLI prints a valid YAML example of a
      `Command` entry with its commonly used nested properties.
- [ ] Given `sw example Command`, the example includes the implemented
      `trim_empty_lines` output field.
- [ ] Given `sw example Command`, the example includes the implemented
      `stream` output field.
- [ ] Given `sw example DisplayFile`, the CLI prints a valid YAML example of a
      `DisplayFile` entry.
- [ ] Given `sw example DisplayFile`, the example includes the implemented
      `indent` field for whole-block indentation.
- [ ] Given `sw example DisplayFile`, the example includes the implemented
      Java `collapse_method_body` transform shape.
- [ ] Given `sw example Patch`, the CLI prints a valid YAML example of a
      `Patch` entry that reflects the current automatic-restore contract.
- [ ] Given `sw example Command --output-format yaml`, the CLI prints the same
      YAML shape as the default mode.
- [ ] Given `sw example Command --output-format json`, the CLI prints a valid
      JSON example of a `Command` entry.
- [ ] Given `sw example command`, the CLI behaves the same as
      `sw example Command`.
- [ ] Given an unsupported entity type, the CLI exits with `1` and reports
      that the entity type is unknown.
- [ ] Given an unsupported output format, the CLI exits with `1` and reports
      that the format is unknown.
- [ ] The help output documents the `example` command and how to request an
      entity type and output format.

## Non-goals

- Generating a full `sw-runbook.json`; that remains the responsibility of
  `sw init`.
- Supporting output formats other than YAML and JSON in this increment.
- Detecting project context to personalize examples.
- Accepting multiple entity types in one invocation in this increment.

## Edge Cases

- Entity type name differs only by case from a supported value.
- Entry example becomes so minimal that users must immediately drill down into
  other examples to do anything useful.
- Example output drifts from the currently supported schema.
- YAML output differs structurally from the JSON output for the same example in
  a way that changes the represented contract instead of only the syntax.
- A newly supported runbook entry type is implemented in `run` but missing from
  `sw example`.
