# `sw example`

Use `sw example` to print a focused runbook snippet.

```shell
sw example Command
```

Examples default to YAML because YAML is the preferred file-backed authoring
format.

## Common Usage

Print a command entry example:

```shell
sw example Command
```

Print a display-file entry example:

```shell
sw example DisplayFile
```

Print JSON instead of YAML:

```shell
sw example Command --output-format json
```

## Notes

- Supported topics currently include `Command`, `DisplayFile`, `Patch`, and
  `Prerequisite`.
- Topic matching is case-insensitive.
- Example output is a starting point and may need editing before use.
- Use `sw explain <topic>` for behavior and defaults, not `sw example`.

For exact syntax, run `sw help example`. For the behavior contract, run
`sw explain example` or read [SPEC-008](../spec/SPEC-008-example-snippets.md).
