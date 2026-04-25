# Guides

Human-oriented guides for using Sociable Weaver (`sw`).

These pages explain common workflows and practical usage. They are not the
source of truth for the command contract. For exact command-line syntax, use
`sw help <subcommand>`. For behavior, defaults, and agent-facing discovery, use
`sw explain <topic>` or `sw explain --all`.

## Start Here

- [Quick workflow](#quick-workflow)
- [Run a runbook](./run.md)
- [Validate a runbook](./validate.md)
- [Create a starter runbook](./init.md)
- [Import an existing README](./import.md)

## Subcommand Guides

- [`sw init`](./init.md): create a starter runbook.
- [`sw import`](./import.md): convert an existing `README.md` into a starter
  runbook.
- [`sw validate`](./validate.md): check runbook structure without execution.
- [`sw check`](./check.md): validate prerequisites without running normal
  workflow commands.
- [`sw run`](./run.md): execute a runbook and generate Markdown output.
- [`sw format`](./format.md): rewrite a runbook file using the canonical
  formatting for its current format.
- [`sw convert`](./convert.md): convert a runbook between YAML and JSON.
- [`sw example`](./example.md): print focused runbook entry snippets.
- [`sw explain`](./explain.md): discover behavior, defaults, and feature
  contracts from the installed binary.
- [`sw help`](./help.md): print exact command-line syntax.
- [`sw version`](./version.md): print the current build identity.

## Quick Workflow

Create a starter YAML runbook:

```shell
sw init
```

Validate the runbook:

```shell
sw validate
```

Check prerequisites without running the whole workflow:

```shell
sw check
```

Execute the runbook and generate `README.md`:

```shell
sw run
```

## Agent Guidance

AI agents should treat the installed `sw` binary as the discoverable interface:

```shell
sw explain --all
sw example Command
sw help run
```

Use these guides for human context and workflow examples. Use `sw explain` for
the current product contract.
