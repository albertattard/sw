# `sw check`

Use `sw check` to validate a runbook and execute only prerequisite checks.

```shell
sw check
```

This is useful before running a full workflow because it confirms whether the
environment is ready.

## Common Usage

Check the default runbook:

```shell
sw check
```

Check an explicit file:

```shell
sw check --input-file ./sw-runbook.yaml
```

Check prerequisites against a different project directory:

```shell
sw check \
  --input-file /path/to/runbook/sw-runbook.yaml \
  --working-directory /path/to/project
```

Check YAML from stdin:

```shell
sw check --input-file=- --input-format yaml
```

## Notes

- `sw check` does not render or write `README.md`.
- `--working-directory` selects the execution root for runbook-relative paths
  used by prerequisite checks.
- Normal `Command` entries are not executed.
- Built-in prerequisite checks include Java version checks.
- Command-based prerequisite checks use a shorter default timeout than normal
  command entries.

For exact syntax, run `sw help check`. For the behavior contract, run
`sw explain check` or read
[SPEC-005](../spec/SPEC-005-check-prerequisites.md).
