# `sw help`

Use `sw help` to print exact command-line syntax.

```shell
sw help
```

## Common Usage

Print top-level help:

```shell
sw help
```

Print help for one subcommand:

```shell
sw help run
```

Print all implemented help text:

```shell
sw help --all
```

## Notes

- Use help for syntax, flags, and command names.
- Use `sw explain` when the question is about behavior or defaults.
- Use `sw example` when the question is about runbook entry shape.

For the behavior contract, run `sw explain help` or read
[SPEC-001](../spec/SPEC-001-help-and-discovery.md).
