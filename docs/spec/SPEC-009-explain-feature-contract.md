# SPEC-009: Explain Feature Contract

- Status: Proposed
- Owner: @aattard
- Created: 2026-03-13
- Last updated: 2026-03-20

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
- `sw explain --output-format=skill` prints a `SKILL.md`-compatible
  Markdown document to stdout.
- `sw explain --output-format=skill --output-file` writes the skill document
  to the default Codex skill location for `sw`.
- `sw explain --output-format=skill --output-file=<path>` writes the skill
  document to the provided path.
- When `--output-file` or `--output-file=<path>` is used and the target
  already exists, the command exits with `1` and leaves the existing file
  unchanged unless `--force` is also provided.
- `sw explain` without a topic and without `--all` exits with `1` and prints a
  clear usage error, except when `--output-format=skill` is selected.
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
- `sw explain --output-format=text|skill`
- `sw explain --output-file[=<path>]`
- `sw explain --force`

## Outputs

- Human-readable plain text to stdout by default.
- `skill` output is Markdown intended to be saved as `SKILL.md`.
- When `--output-file` is present without a value, `skill` output is written to
  the default Codex skill location for `sw`.
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

When a topic includes authoring guidance for structured runbook fields, the
output should prefer the most semantic documented mechanism over brittle text
substitution. For example, run-oriented guidance should tell agents to prefer
`output.rewrite` with `type: datetime_shift` for real dates and times, and to
use `replace` only for non-semantic text or unsupported formats.

When `--output-format=skill` is selected:

- The command emits one aggregate skill document for the current `sw` build
  rather than a per-topic explanation.
- The default file destination for `--output-file` without a value is the
  standard Codex skill path for the `sw` skill, such as
  `~/.codex/skills/sw/SKILL.md`.
- The skill content should direct agents toward `sw help`, `sw example`, and
  `sw explain` for authoritative command, snippet, and contract discovery.
- The skill content must make implemented versus planned commands clear enough
  that an agent does not treat planned commands as available by default.
- The skill content should preserve important authoring guidance from explain
  topics, including preferring `datetime_shift` over `replace` for semantic
  dates and times in runbook rewrites.
- The output should remain deterministic so repeated exports produce stable
  skill content aside from intentional contract changes.

## Exit Codes

- `0` when the requested explanation is produced successfully
- `1` for operational or usage errors, including unknown topics or missing
  required arguments

## Acceptance Criteria

- [ ] Given `sw explain run`, the CLI prints a concise explanation of the run
      contract derived from `SPEC-003`.
- [ ] Given `sw explain run`, the CLI tells agents to prefer
      `datetime_shift` over `replace` for semantic dates and times, using
      `replace` only for non-semantic text or unsupported formats.
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
- [ ] Given `sw explain --output-format=skill`, the CLI exits with `0` and
      prints a deterministic `SKILL.md`-compatible document to stdout.
- [ ] Given `sw explain --output-format=skill --output-file`, the CLI exits
      with `0` and writes the skill document to the default Codex skill
      location for `sw`.
- [ ] Given `sw explain --output-format=skill --output-file=<path>`, the CLI
      exits with `0` and writes the skill document to the provided path.
- [ ] Given `sw explain --output-format=skill --output-file` when the default
      target file already exists, the CLI exits with `1` and does not
      overwrite the file.
- [ ] Given `sw explain --output-format=skill --output-file=<path>` when the
      target file already exists, the CLI exits with `1` and does not
      overwrite the file.
- [ ] Given `sw explain --output-format=skill --output-file --force` or
      `sw explain --output-format=skill --output-file=<path> --force`, the CLI
      overwrites the target file and exits with `0`.
- [ ] Given `sw explain --output-format=skill`, the generated skill content
      clearly distinguishes implemented commands from planned commands.
- [ ] Given `sw explain --output-format=skill`, the generated skill content
      preserves the documented preference for `datetime_shift` over `replace`
      for semantic dates and times.
- [ ] Help output documents `--output-format=<format>`,
      `--output-file[=<path>]`, and `--force` for `explain`.
- [ ] Help output documents the `explain` command and the `--all` option.

## Edge Cases

- Topic names with different letter casing.
- `--output-format=skill` used without a topic or `--all`.
- `--output-format=skill` combined with a topic or with `--all`.
- `--force` used without `--output-file`.
- `--output-file` resolves to a default Codex skill directory that does not
  exist yet.
- `--output-file=<path>` points to a parent directory that does not exist.
- `--output-file` or `--output-file=<path>` resolves to an unwritable target.
- A future topic exists in the specs but is not yet exposed by `explain`.
- `--all` output remains stable in ordering across runs.
- Explanations stay concise and do not dump raw spec Markdown by default.
- An agent needs to answer "how do I check for Java 21?".
- An agent needs to answer "how do I use JAVA_17_HOME?".
- An agent needs to decide whether to call `help`, `example`, or `explain`
  first.
