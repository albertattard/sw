---
id: TASK-205
title: Add Secrets Example
status: done
category: example
related_features:
  - SPEC-008
  - SPEC-013
owner: albertattard
created: 2026-11-14
updated: 2026-11-18
---

## Summary

Make generated secrets discoverable through `sw example Secrets`, with a
copyable complete runbook showing declarations and bindings across commands.

## Scope

- Add case-insensitive `Secrets` topic support with YAML and JSON output.
- Show at least two named secrets: one with `generate: {}` and one with an
  explicit valid length. Bind a shared secret in at least two `Command` entries
  and demonstrate use of the bound environment variables. Example commands
  must not print secret values.
- Keep the example self-contained, without external services, credentials, or
  persistent side effects, so it can be run as well as validated.
- Update example help, help-focused tests, and the example guide.
- Test the declared secrets, bindings and reuse, both formats, structural
  equivalence, explicit YAML, case-insensitive lookup, and validation of the
  complete example as a runbook. Exercise the example with `sw run` in an
  isolated working directory.

## Boundaries

- This is a discovery example for the existing SPEC-013 behavior; it does not
  change generation, binding, redaction, or runbook validation.
- Avoid passing secret values as process arguments or writing them to files.
  The guide should point to the [run guide's secrets section](../../guides/run.md)
  for lifetime and redaction limitations, rather than implying that masking
  prevents all leaks.

## Acceptance Criteria

- [x] `sw example Secrets` prints a self-contained YAML runbook with both
      generation forms, multiple bindings, and shared-secret reuse as described
      above; its commands use the bindings without printing their values.
- [x] JSON represents the same runbook as default YAML; explicit YAML matches
      default output, and topic matching is case-insensitive.
- [x] Example help and the example guide advertise the topic.
- [x] Integration tests assert the secret declarations and binding references,
      validate both formats with `sw validate`, and run the example successfully
      in an isolated working directory without exposing generated values in
      normal output. A test may use `--reveal-secrets` to obtain values for
      comparison, excluding its intentional reveal list from that assertion.
