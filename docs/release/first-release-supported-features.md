## Supported Features

Sociable Weaver supports the core executable-documentation workflow:

- Author runbooks in YAML or JSON so documentation examples stay versioned and
  reviewable.
- Validate runbook structure before execution with `sw validate`.
- Check prerequisites separately with `sw check`.
- Execute verified workflows and generate README-style Markdown output with
  `sw run`.
- Import an existing `README.md` into a starter runbook with `sw import`.
- Discover the CLI contract through built-in help, explain, and example
  commands.

The first release also includes the current runbook capabilities that make
these examples practical to maintain:

- command assertions, timeouts, and cleanup
- prerequisite checks for environment validation
- patch application with automatic restore
- output capture, interpolation, and rewrite rules
- display-file rendering with language-aware fenced blocks
- verbose and debug output to support authoring and troubleshooting
