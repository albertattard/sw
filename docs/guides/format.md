# `sw format`

Use `sw format` to rewrite a runbook file in place using the canonical
formatting for its current format.

```shell
sw format
```

## Common Usage

Format the default runbook:

```shell
sw format
```

Format an explicit file:

```shell
sw format --input-file ./sw-runbook.yaml
```

## Notes

- `sw format` validates before writing changes.
- YAML formatting keeps runbooks editing-friendly, including blank lines
  between entries and indented sequences under keys.
- Stdin formatting is not supported; format works on files.

For exact syntax, run `sw help format`. For the behavior contract, run
`sw explain format` or read
[SPEC-010](../spec/SPEC-010-format-runbook-json.md).
