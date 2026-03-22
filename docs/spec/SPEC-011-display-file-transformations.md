---
id: SPEC-011
title: DisplayFile Transformations
status: proposed
priority: medium
owner: @aattard
last_updated: 2026-03-22
---

## Problem

Users and agents can already copy source files into generated Markdown with
`DisplayFile`, but they cannot reshape verbose or distracting parts of those
files for documentation. This makes it hard to present focused examples such
as collapsed Java methods, redacted JSON fields, or shortened XML sections
without mutating the source tree or relying on separate temporary patches.

## Goal

Extend `DisplayFile` so it can apply explicit, validated, documentation-first
transformations before rendering file content.

## User-facing Behavior

`DisplayFile` continues to copy file content into a fenced block, but it may
also declare a `transform` block:

```json
{
  "type": "DisplayFile",
  "path": "./src/main/java/demo/HolderOfUniqueValues.java",
  "transform": {
    "language": "java",
    "operations": [
      {
        "type": "collapse_method_body",
        "name": "initialize",
        "replacement": "/* Closed for brevity */"
      }
    ]
  }
}
```

The command reads the file, applies the declared transformations, and renders
the transformed content instead of the raw file bytes.

## Inputs

`DisplayFile` retains its existing fields:

- `path`
- `start_line`
- `line_count`
- `indent`
- `offset`

`DisplayFile` may also declare:

- `transform`

### Transform Shape

`transform` is an object with:

- required `language`
- required `operations`

`language` identifies which transformation rules apply.

`operations` is a non-empty array of transformation objects applied in order.

## Supported Languages

This feature is intended to grow over time. The initial contract should allow
language-specific behavior without overloading the top-level `DisplayFile`
shape.

Planned language families include:

- `java`
- `json`
- `xml`
- `text`

Unknown languages are rejected by validation unless and until the contract is
expanded intentionally.

## Output Behavior

- `DisplayFile` renders the transformed content into the fenced block.
- The output remains human-readable Markdown.
- Existing fence-language detection still applies based on the displayed file.
- Transformations affect only rendered output; they do not write back to the
  source file.

## Validation Rules

- `transform.language` must be a known language.
- `transform.operations` must be a non-empty array.
- Each operation must declare a supported `type`.
- Each operation type must validate its required fields strictly.
- Unknown operation fields should be rejected by default unless the contract is
  explicitly expanded.
- If a transformation cannot be applied deterministically, rendering fails with
  a clear operational error rather than silently producing partial output.

## Execution Order

`DisplayFile` rendering should follow this sequence:

1. Read the file content.
2. Parse and transform the content according to `transform`.
3. Apply `start_line` and `line_count` to the transformed display content.
4. Apply `offset` to copied content lines.
5. Apply `indent` to the fenced block.
6. Render the final fenced block.

This keeps transformation semantics separate from block layout concerns.

## Initial Increment

The first implementation increment should support:

- `language: java`
- one operation type: `collapse_method_body`

### Java `collapse_method_body`

Operation shape:

```json
{
  "type": "collapse_method_body",
  "name": "initialize"
}
```

Behavior:

- Find the named Java method in the displayed file.
- Preserve the original method signature.
- Replace only the method body with a single-line body containing the
  replacement comment.
- `replacement` is optional.
- If `replacement` is omitted, the renderer uses
  `/* Closed for brevity */`.
- Render the result as a valid-looking Java snippet suitable for
  documentation.

Example output:

```java
public void initialize(final int moduloDivisor) { /* Closed for brevity */ }
```

## Non-goals

- Mutating source files as part of `DisplayFile`.
- Inferring transformations implicitly from file extensions without explicit
  `transform` instructions.
- Supporting every language or code-structure rewrite in the first increment.
- Replacing the existing `Patch` entry for arbitrary source edits.

## Edge Cases

- Requested Java method does not exist.
- Multiple methods with the same name exist and the operation is ambiguous.
- The target file is not valid enough for deterministic transformation.
- `start_line` and `line_count` are applied after a transformation changes the
  displayed line structure.
- A future runbook requests `json`, `xml`, or `text` transforms before those
  operation families are implemented.

## Acceptance Criteria

- [ ] Given a `DisplayFile` entry with `transform.language: "java"` and a
      `collapse_method_body` operation, the rendered snippet preserves the
      method signature and replaces the body with the requested replacement or
      the default replacement when omitted.
- [ ] Given a `DisplayFile` entry without `transform`, rendering behaves the
      same as today.
- [ ] Validation rejects unknown `transform.language` values.
- [ ] Validation rejects unknown transform operation types.
- [ ] Given a named Java method that does not exist, rendering fails with a
      clear operational error.
- [ ] Help, explain, and examples are updated when the first transform
      increment is implemented so users and agents can discover
      `collapse_method_body`.

## Notes for Reimplementation

The first implementation should prefer a conservative, deterministic approach
over a broad but fragile one. The contract should leave room for later
`json`, `xml`, and `text` operations without forcing those formats into a
Java-specific model.
