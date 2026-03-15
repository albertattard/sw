# SPEC-009: Explain Feature Contract

- Status: Proposed
- Owner: @aattard
- Created: 2026-03-13
- Last updated: 2026-03-13

## Problem

Users, agents, and other models can discover CLI usage through `sw help` and
request JSON snippets through `sw example`, but there is no command that
explains the documented product contract itself. A caller without repository
access cannot easily ask `sw` how a feature works, what defaults apply, or
which major constraints and exit codes matter.

## Goals

- Add an `explain` subcommand that exposes product-contract knowledge through
  the CLI.
- Make `explain` useful for both humans and agents, with output optimized for
  concise machine-assisted discovery.
- Make `explain` agent-first so an LLM or other agent can learn how to
  interact with `sw` without repository access.
- Support targeted explanation for one topic and a full aggregate mode for all
  supported topics.

## Non-goals

- Dumping raw Markdown spec files by default.
- Replacing `help` or `example`.
- Exposing repository-internal implementation details that are not part of the
  user-facing contract.

## User-facing Behavior

- `sw explain <topic>` prints a concise explanation of the requested feature.
- `sw explain --all` prints explanations for all supported topics.
- `sw explain` without a topic and without `--all` exits with `1` and prints a
  clear usage error.
- Unknown topics exit with `1` and print a clear error.
- `sw explain` output is readable to humans and stable enough for programmatic
  or agent use.
- `sw explain` may prefer an agent-friendlier structure over a more
  conversational human style when those goals conflict.

## Agent-first Positioning

- `explain` is the primary CLI surface for agent-first product guidance.
- An agent should be able to use `explain` to decide how to interact with `sw`
  for common user questions.
- `help` remains the command-reference surface.
- `example` remains the JSON-snippet surface.
- `explain` should make those boundaries clear enough that an agent can choose
  the next command reliably.

## Supported Topics

In this increment, `explain` supports at least:

- `help`
- `validate`
- `run`
- `check`
- `init`
- `import`
- `example`

Topic matching should be case-insensitive.

## Inputs

- `sw explain <topic>`
- `sw explain --all`

## Outputs

- Human-readable plain text to stdout.
- Concise by default.
- Structured in predictable sections so agents can parse it reliably.
- Optimized for deterministic interpretation over conversational phrasing.

## Output Content

For each explained topic, the output should include:

- purpose
- main defaults
- major inputs and outputs
- exit code behavior
- important constraints or assumptions
- when the agent should use `help`, `example`, or `explain` next

The output should be structured so an agent can infer:

- whether `explain` is the correct command for the current question
- which topic best matches the question
- whether a follow-up `example` call would be useful

The output should summarize the corresponding spec contract rather than print
the raw spec file verbatim.

## Exit Codes

- `0` when the requested explanation is produced successfully
- `1` for operational or usage errors, including unknown topics or missing
  required arguments

## Acceptance Criteria

- [ ] Given `sw explain run`, the CLI prints a concise explanation of the run
      contract derived from `SPEC-003`.
- [ ] Given `sw explain validate`, the CLI prints a concise explanation of the
      validate contract derived from `SPEC-002`.
- [ ] Given `sw explain --all`, the CLI prints explanations for all supported
      topics.
- [ ] Given an agent-oriented user question such as "how do I check for Java
      21?", the documented `explain` contract makes it clear that `explain` is
      the correct discovery command rather than `help`.
- [ ] Given an agent-oriented user question about configuration shape, the
      documented `explain` contract makes it clear when the next step should be
      `sw example <topic>`.
- [ ] Given an agent choosing among `help`, `example`, and `explain`, the
      documented `explain` contract provides enough context to choose
      reliably.
- [ ] Given `sw explain RUN`, the CLI behaves the same as `sw explain run`.
- [ ] Given `sw explain unknown`, the CLI exits with `1` and prints a clear
      error.
- [ ] Given `sw explain` without a topic and without `--all`, the CLI exits
      with `1` and prints a clear usage error.
- [ ] Help output documents the `explain` command and the `--all` option.

## Edge Cases

- Topic names with different letter casing.
- A future topic exists in the specs but is not yet exposed by `explain`.
- `--all` output remains stable in ordering across runs.
- Explanations stay concise and do not dump raw spec Markdown by default.
- An agent needs to answer "how do I check for Java 21?".
- An agent needs to answer "how do I use JAVA_17_HOME?".
- An agent needs to decide whether to call `help`, `example`, or `explain`
  first.
