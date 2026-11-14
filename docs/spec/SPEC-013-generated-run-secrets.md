---
id: SPEC-013
title: Generated Run Secrets
status: implemented
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

The initial source is generated values only. Secret names and environment
variable names are case-sensitive and match the ASCII pattern
`[A-Za-z_][A-Za-z0-9_]*`. Each secret requires a `generate` mapping;
`generate: {}` selects the defaults. An optional `length` must be an integer
from 16 through 256 inclusive, defaulting to 32. The generation alphabet is
exactly `ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`;
each character is selected uniformly. Configurable additional characters are
outside this increment.

Empty `secrets` and `environment` mappings are valid no-ops. Secret
declarations must contain `generate`, and each environment binding must contain
a `secret` reference. Validation rejects unknown fields, malformed mappings,
invalid names or lengths, and references to undeclared secrets. Generation
failures fail the run without falling back to a weaker random source.

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
Masking matches complete known secret values; a substring printed on its own
is not necessarily masked.

Secrets are generated before run prerequisite checks. An explicit
`--reveal-secrets` run option prints a dedicated list of secret names and
values to stderr once, after all secrets have been generated successfully and
before those checks begin. The list is therefore printed even if a subsequent
prerequisite fails. The option works independently: it neither requires nor
enables `--debug`. If generation fails, no partial list is printed and
prerequisite checks do not run. All other output remains masked even with this
option, including generated Markdown, progress, errors, and every existing
debug section such as raw command stdout, stderr, and cleanup output. Help
documents that the reveal list contains secrets and stderr may be captured in
logs.

For one or more secrets, the reveal list has the heading `Generated secrets:`
followed by one line per secret in declaration order. Each line consists of
two spaces, the secret name, a colon and a space, and the generated value:

```text
Generated secrets:
  mysql_root_password: ExampleSecret123
  mysql_user_password: AnotherExample456
```

If no secrets are declared, `--reveal-secrets` prints no list or heading.
Standalone `sw check` validates secret declarations and command bindings, but
does not generate or reveal secrets. Secret bindings apply to `Command`
entries and their cleanup, not prerequisite checks.

Masking protects output controlled by `sw`; commands and external services can
expose secrets independently, including through process arguments, environment,
container configuration, files, and transformed output. The example's shell
expansion passes the password to Docker as an argument and Docker stores it in
the container configuration.

## Acceptance Criteria

- [x] A secret is generated once per invocation and shared by explicitly bound
      command entries and their cleanup; a separate invocation generates anew.
- [x] Generated Markdown, verbose output, debug output, streamed output, and
      failures mask exact values, including values split across stream chunks.
- [x] Assertions and captures operate on the appropriate real command output;
      subsequent output cannot disclose a captured known secret.
- [x] A captured secret interpolated into later output is masked, and a secret
      split across streamed chunks is masked without leaking its fragments.
- [x] `--reveal-secrets` prints only the dedicated list to stderr, independently
      of debug, while all other output remains masked; failed generation prints
      no partial list.
- [x] Generation and the optional reveal list precede prerequisite checks;
      a failed prerequisite still follows a successfully printed reveal list,
      and failed generation prevents prerequisite checks.
- [x] The reveal list follows the specified heading, line format, declaration
      order, and empty case. Standalone `sw check` validates without generating
      or revealing secrets or injecting bindings into prerequisite checks.
- [x] Validation enforces the specified names, integer length bounds, mandatory
      `generate` mapping, empty mapping behavior, and binding constraints.
- [x] Generation uses the exact specified alphabet and uniform selection.
- [x] Formatting places `environment` immediately before `commands` in JSON
      and YAML, regardless of input order.
- [x] Help, examples, and run documentation explain binding, redaction limits,
      external exposure, and the `--start-at` lifetime limitation.
- [x] CLI integration tests cover execution, reuse, cleanup, masking, failure,
      streamed output, reveal behavior, validation, and help.
