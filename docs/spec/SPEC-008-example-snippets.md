# SPEC-008: Example Snippets Contract

- Status: Draft
- Owner: `@aattard`
- Created: `2026-03-13`
- Updated: `2026-03-22`

## Goal

Provide a simple way for users to request focused JSON examples for supported
runbook entry types without generating a full starter runbook.

## User-facing Behavior

The CLI provides an explicit `example` command:

```bash
sw example <entity-type>
```

This command prints a JSON snippet to stdout for the requested entity type.

The snippet is intended as a copyable starting point.

## Inputs

- Required entity-type argument: `<entity-type>`

In this increment, supported entity types include:
- `Command`
- `DisplayFile`
- `Patch`
- `Prerequisite`

## Outputs

- JSON snippet written to stdout
- Human-readable error on stderr for unsupported entity types

### Exit Codes

- `0`: example was found and printed successfully
- `1`: operational error or unknown entity type

## Example Contract

- Example output should be valid JSON.
- Entity-type matching is case-insensitive.
- Entry-type examples should print a single JSON object representing that
  entry and should include the commonly used nested properties for that entry
  type so users can remove what they do not need.
- The `Command` example should reflect the current output contract, including
  supported output fields such as `stream` and `trim_empty_lines` when
  available.
- The `DisplayFile` example should reflect the current transform contract,
  including Java `collapse_method_body` when implemented.
- Example output is documentation-oriented and does not need to be executable
  without further user editing.

## Acceptance Criteria

- [ ] Given `sw example Command`, the CLI prints a valid JSON example of a
      `Command` entry with its commonly used nested properties.
- [ ] Given `sw example Command`, the example includes the implemented
      `trim_empty_lines` output field.
- [ ] Given `sw example Command`, the example includes the implemented
      `stream` output field.
- [ ] Given `sw example DisplayFile`, the CLI prints a valid JSON example of a
      `DisplayFile` entry.
- [ ] Given `sw example DisplayFile`, the example includes the implemented
      Java `collapse_method_body` transform shape.
- [ ] Given `sw example Patch`, the CLI prints a valid JSON example of a
      `Patch` entry that reflects the current automatic-restore contract.
- [ ] Given `sw example command`, the CLI behaves the same as
      `sw example Command`.
- [ ] Given an unsupported entity type, the CLI exits with `1` and reports
      that the entity type is unknown.
- [ ] The help output documents the `example` command and how to request an
      entity type.

## Non-goals

- Generating a full `sw-runbook.json`; that remains the responsibility of
  `sw init`.
- Detecting project context to personalize examples.
- Accepting multiple entity types in one invocation in this increment.

## Edge Cases

- Entity type name differs only by case from a supported value.
- Entry example becomes so minimal that users must immediately drill down into
  other examples to do anything useful.
- Example output drifts from the currently supported schema.
- A newly supported runbook entry type is implemented in `run` but missing from
  `sw example`.
