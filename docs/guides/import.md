# `sw import`

Use `sw import` to turn an existing Markdown README into an editable runbook.

```shell
sw import
```

By default, this reads `README.md` and writes `sw-runbook.yaml`.

## Common Usage

Import the default README:

```shell
sw import
```

Import from and write to explicit paths:

```shell
sw import \
  --input-file ./README.md \
  --output-file ./sw-runbook.yaml
```

Write JSON instead of YAML:

```shell
sw import --output-format json
```

## Notes

- Import is intentionally best-effort and produces a starting point for manual
  editing.
- Headings become `Heading` entries.
- Prose becomes `Markdown` entries.
- Shell fenced blocks become `Command` entries.
- Non-shell fenced blocks remain Markdown so `sw` does not guess execution
  semantics.

For exact syntax, run `sw help import`. For the behavior contract, run
`sw explain import` or read
[SPEC-006](../spec/SPEC-006-import-readme-to-runbook.md).
