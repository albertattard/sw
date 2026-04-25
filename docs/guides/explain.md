# `sw explain`

Use `sw explain` to discover behavior, defaults, and feature contracts from the
installed `sw` binary.

```shell
sw explain run
```

This is the primary interface for AI agents that do not have source access.

## Common Usage

Explain one topic:

```shell
sw explain run
```

Print all supported explanations:

```shell
sw explain --all
```

Export a Codex skill routing file:

```shell
sw explain --output-format skill
```

Write that skill to the default Codex skill path:

```shell
sw explain --output-format skill --output-file
```

## Notes

- Use `sw explain` for behavior, defaults, constraints, and discovery paths.
- Use `sw help` for exact command-line syntax.
- Use `sw example` for copyable runbook snippets.
- The generated skill file should stay small and route agents back to
  `sw explain`.

For exact syntax, run `sw help explain`. For the behavior contract, read
[SPEC-009](../spec/SPEC-009-explain-feature-contract.md).
