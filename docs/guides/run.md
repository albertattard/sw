# `sw run`

Use `sw run` to execute a runbook and generate Markdown output.

```shell
sw run
```

Running `sw` without a subcommand is equivalent to `sw run`.

## Five-Minute Workflow

Create a starter runbook:

```shell
sw init
```

Validate it:

```shell
sw validate
```

Check prerequisites:

```shell
sw check
```

Run it and write `README.md`:

```shell
sw run
```

## Common Usage

Run an explicit runbook:

```shell
sw run --input-file ./sw-runbook.yaml
```

Write output somewhere other than `README.md`:

```shell
sw run \
  --input-file ./sw-runbook.yaml \
  --output-file ./generated.md
```

Show progress while running:

```shell
sw run --verbose
```

Use plain progress output for SSH or wrapper-driven execution:

```shell
sw run --verbose --verbose-mode plain
```

Debug command rewrite and capture behavior:

```shell
sw run --debug
```

## Runbook Authoring

Use YAML for file-backed runbooks by default. Use `sw example` to get current
entry shapes:

```shell
sw example Command
sw example DisplayFile
sw example Prerequisite
sw example Patch
```

Useful `Command` fields include:

- `commands`: shell script to execute.
- `working_dir`: run commands from a runbook-relative directory.
- `timeout`: override the default command timeout.
- `cleanup`: release resources after the command finishes or the run fails.
- `assert.exit_code`: assert that a command succeeds or intentionally fails.
- `assert.checks`: assert expected output or file state.
- `output`: control rendered output, captions, streams, content type, trimming,
  and rewrite rules.
- `capture`: store command output for later interpolation.
- `debug`: enable diagnostics for a single supported entry, such as `Command`
  or `Patch`.

## Notes

- Default output file is `README.md`.
- Command output is rendered only when a `Command` entry declares `output`.
- Markdown can interpolate captured variables with `@{name}`.
- Use `@@{name}` when the generated Markdown should contain a literal
  `@{name}`.
- `datetime_shift.id` and `datetime_shift.use` reuse a shared timeline.
- Rewrite `capture_as` exposes original and rewritten values for later use.

For exact syntax, run `sw help run`. For the behavior contract, run
`sw explain run` or read
[SPEC-003](../spec/SPEC-003-run-runbook-to-markdown.md).
