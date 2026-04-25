# `sw convert`

Use `sw convert` to convert a runbook between YAML and JSON.

```shell
sw convert
```

When only `sw-runbook.json` exists, the default conversion writes YAML. When
only `sw-runbook.yaml` or `sw-runbook.yml` exists, the default conversion
writes JSON.

## Common Usage

Convert an explicit JSON runbook to YAML:

```shell
sw convert --input-file ./sw-runbook.json
```

Convert an explicit YAML runbook to JSON:

```shell
sw convert --input-file ./sw-runbook.yaml
```

Overwrite an existing output file:

```shell
sw convert --input-file ./sw-runbook.json --force
```

## Notes

- `sw convert` refuses ambiguous default input when multiple default runbooks
  exist.
- The command validates input before writing output.
- It refuses same-format conversion.
- It refuses to overwrite existing output unless `--force` is provided.

For exact syntax, run `sw help convert`. For the behavior contract, run
`sw explain convert` or read
[SPEC-012](../spec/SPEC-012-convert-runbook-formats.md).
