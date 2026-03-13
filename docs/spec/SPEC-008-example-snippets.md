# SPEC-008: Example Snippets Contract

- Status: Draft
- Owner: `@aattard`
- Created: `2026-03-13`
- Updated: `2026-03-13`

## Goal

Provide a simple way for users to request focused JSON examples for supported
runbook entities and nested features without generating a full starter runbook.

## User-facing Behavior

The CLI provides an explicit `example` command:

```bash
sw example <topic>
```

This command prints a JSON snippet to stdout for the requested topic.

The snippet is intended as a copyable starting point, not as a complete
runbook file.

## Inputs

- Required topic argument: `<topic>`

In this increment, topics may refer to:
- a runbook entry type such as `Command`, `DisplayFile`, or `Prerequisite`
- a nested feature such as `rewrite.keep_between`

The example catalog may grow over time to cover additional rewrite-rule topics
without changing the `example` command shape.

## Outputs

- JSON snippet written to stdout
- Human-readable error on stderr for unsupported topics

### Exit Codes

- `0`: example was found and printed successfully
- `1`: operational error or unknown topic

## Example Contract

- Example output should be valid JSON.
- Example output should be minimal but realistic.
- A topic-specific example should focus on the requested entity or feature
  rather than printing a full sample runbook.
- Entry-type examples should print a single JSON object representing that
  entry.
- Nested-feature examples should print the smallest meaningful JSON fragment
  that shows where the feature belongs.
- Example output is documentation-oriented and does not need to be executable
  without further user editing.

## Acceptance Criteria

- [ ] Given `sw example Command`, the CLI prints a valid JSON example of a
      `Command` entry.
- [ ] Given `sw example DisplayFile`, the CLI prints a valid JSON example of a
      `DisplayFile` entry.
- [ ] Given `sw example rewrite.keep_between`, the CLI prints a valid JSON
      example of that rewrite rule fragment.
- [ ] Given `sw example rewrite.replace`, the CLI prints a valid JSON example
      of a `replace` rewrite rule fragment.
- [ ] Given `sw example rewrite.datetime_shift`, the CLI prints a valid JSON
      example of a `datetime_shift` rewrite rule fragment.
- [ ] Given a rewrite example that demonstrates captured-variable usage, the
      CLI prints a valid JSON example showing how rewrite rules can reference
      captured values.
- [ ] Given an unsupported topic, the CLI exits with `1` and reports that the
      topic is unknown.
- [ ] The help output documents the `example` command and how to request a
      topic.

## Non-goals

- Printing every supported example in one command in this increment.
- Generating a full `sw-runbook.json`; that remains the responsibility of
  `sw init`.
- Detecting project context to personalize examples.

## Edge Cases

- Topic name differs only by case from a supported topic.
- Topic refers to a known family but not a supported leaf such as
  `rewrite.unknown`.
- Example output drifts from the currently supported schema.
- Rewrite examples drift from the currently supported capture and rewrite
  syntax.
