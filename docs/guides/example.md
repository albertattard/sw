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

Print a display-url entry example:

```shell
sw example DisplayUrl
```

Print a breakpoint entry example:

```shell
sw example Breakpoint
```

Print a complete, runnable generated-secrets example:

```shell
sw example Secrets > secrets.yaml
sw validate --input-file secrets.yaml
sw run --input-file secrets.yaml --output-file secrets.md
```

This runbook declares a default-length secret and a 48-character secret, binds
both to command environment variables, and reuses the default-length secret in
a second command. Its shell builtins check the bound lengths and print only
fixed confirmation messages. The commands need no external services or
credentials and leave no persistent side effects; `sw run` writes the requested
Markdown output. See [Generated Run Secrets](run.md#generated-run-secrets) for
secret lifetime and redaction limitations.

Print JSON instead of YAML:

```shell
sw example Command --output-format json
```

## Notes

- Supported topics currently include `Breakpoint`, `Command`, `DisplayFile`,
  `DisplayUrl`, `ChangeDirectory`, `Patch`, `Prerequisite`, and `Secrets`.
- Topic matching is case-insensitive.
- Entry examples are starting points and may need editing before use.
  `Secrets` is a complete runbook that can be validated and run as printed.
- Use `sw explain <topic>` for behavior and defaults, not `sw example`.

For exact syntax, run `sw help example`. For the behavior contract, run
`sw explain example` or read [SPEC-008](../spec/SPEC-008-example-snippets.md).
