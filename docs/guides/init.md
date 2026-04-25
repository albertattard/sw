# `sw init`

Use `sw init` to create a starter runbook.

```shell
sw init
```

By default, this writes `sw-runbook.yaml`. Existing files are not overwritten
unless you pass `--force`.

## Common Usage

Create the default YAML runbook:

```shell
sw init
```

Write to an explicit path:

```shell
sw init --output-file ./examples/demo/sw-runbook.yaml
```

Overwrite an existing generated starter:

```shell
sw init --force
```

## Notes

- YAML is the default starter format.
- JSON can be selected by using a `.json` output path.
- The generated sample is intended to be edited, not treated as a final
  runbook.

For exact syntax, run `sw help init`. For the behavior contract, run
`sw explain init` or read
[SPEC-004](../spec/SPEC-004-init-runbook-sample.md).
