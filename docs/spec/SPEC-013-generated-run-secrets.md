---
id: SPEC-013
title: Generated Run Secrets
status: planned
priority: medium
owner: albertattard
last_updated: 2026-11-14
---

## Problem

Runbooks need temporary credentials for resources such as databases, and later
commands may need the same credentials to connect. Hardcoding them exposes the
values in the runbook and generated documentation.

## Behavior

A runbook can declare named generated secrets at the document level. Each
invocation generates each declared value once using a cryptographically secure
random source. Values are independent, live in memory for that invocation,
remain available through cleanup, and are not persisted by `sw`. A new run,
including one using `--start-at`, generates new values. Consequently a
partial run cannot reuse credentials from a previous run, including a run
whose resources were preserved on failure.

The initial source is generated values only. Generation defaults to 32
characters from a documented alphanumeric alphabet. Length can be specified;
invalid declarations, names, lengths, environment variable names, and unknown
secret references fail validation. Generation failures fail the run without
falling back to a weaker random source.

Commands explicitly bind named secrets to environment variables. The same
secret may be bound in multiple entries or under different environment names.
Bindings are available to the command and its cleanup, and override inherited
environment variables of the same name for those processes. Other commands do
not receive those bindings. `sw format` places `environment` immediately before
`commands` in JSON and YAML. For example:

```yaml
secrets:
  mysql_root_password:
    generate:
      length: 32
entries:
  - type: Command
    environment:
      MYSQL_ROOT_PASSWORD:
        secret: mysql_root_password
    commands: |
      docker run --env MYSQL_ROOT_PASSWORD="$MYSQL_ROOT_PASSWORD" mysql
```

The displayed command retains the environment variable reference. Generated
values are masked with the fixed marker `******` in generated Markdown,
progress, diagnostics, error messages, and command stdout and stderr that `sw`
emits. Masking covers exact known values, including occurrences split across
stream chunks; it does not promise to detect transformed values. Command
execution, assertions, captures, and cleanup must receive the actual values
where needed rather than the masked output. Normal `--verbose`, `--debug`, and
entry-scoped debug do not reveal secrets.

An explicit `--reveal-secrets` run option permits diagnostic output to show
generated values for troubleshooting. It must not silently change generated
Markdown into a secret-bearing document; ordinary command output and errors
remain masked. The option and its disclosure risk are documented in help.

Masking protects output controlled by `sw`; commands and external services can
expose secrets independently, including through process arguments, environment,
container configuration, files, and transformed output. The example's shell
expansion passes the password to Docker as an argument and Docker stores it in
the container configuration.

## Acceptance Criteria

- [ ] A secret is generated once per invocation and shared by explicitly bound
      command entries and their cleanup; a separate invocation generates anew.
- [ ] Generated Markdown, verbose output, debug output, streamed output, and
      failures mask exact values, including values split across stream chunks.
- [ ] Assertions and captures operate on the appropriate real command output;
      subsequent output cannot disclose a captured known secret.
- [ ] `--reveal-secrets` deliberately reveals values in diagnostics only.
- [ ] Validation rejects invalid declarations and bindings and documents the
      supported name, length, and environment variable constraints.
- [ ] Formatting places `environment` immediately before `commands` in JSON
      and YAML, regardless of input order.
- [ ] Help, examples, and run documentation explain binding, redaction limits,
      external exposure, and the `--start-at` lifetime limitation.
- [ ] CLI integration tests cover execution, reuse, cleanup, masking, failure,
      streamed output, reveal behavior, validation, and help.
