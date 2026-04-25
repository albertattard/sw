# `sw validate`

Use `sw validate` to check that a runbook is structurally valid without running
its workflow commands.

```shell
sw validate
```

## Common Usage

Validate the default runbook:

```shell
sw validate
```

Validate an explicit file:

```shell
sw validate --input-file ./sw-runbook.yaml
```

Return machine-readable validation output:

```shell
sw validate --output-format json
```

Validate YAML from stdin:

```shell
sw validate --input-file=- --input-format yaml
```

## Notes

- File-backed validation accepts JSON, YAML, and YML runbooks.
- Stdin input defaults to JSON unless `--input-format yaml` is provided.
- Validation checks documented structure and field rules; it does not execute
  normal workflow commands.

For exact syntax, run `sw help validate`. For the behavior contract, run
`sw explain validate` or read [SPEC-002](../spec/SPEC-002-validate-runbook.md).
