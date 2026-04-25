# SPEC-009: Explain Feature Contract

- Status: Proposed
- Owner: @aattard
- Created: 2026-03-13
- Last updated: 2026-04-25

## Problem

Users, agents, and other models can discover CLI usage through `sw help` and
request runbook snippets through `sw example`, but there is no command that
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
- `example` remains the runbook-snippet surface, with YAML as the default and
  JSON available explicitly.
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

When a topic supports stdin-backed runbook input, the output should explain
`--input-file=-`, the default JSON parsing behavior for stdin, and when
`--input-format=yaml` is required for piped YAML input.

When a topic uses file-backed runbook input by default, the output should make
it clear that `--input-format` does not replace the existing default file
lookup behavior unless stdin is explicitly selected with `--input-file=-`.

When a topic is part of a file-based authoring workflow, the output should make
it clear that YAML is the default format for that workflow and that JSON is an
explicit opt-in when a machine-oriented shape is needed.

When a topic includes authoring guidance for structured runbook fields, the
output should prefer the most semantic documented mechanism over brittle text
substitution. For example, run-oriented guidance should tell agents to prefer
`output.rewrite` with `type: datetime_shift` for real dates and times, and to
use `replace` only for non-semantic text or unsupported formats.

When a topic covers command output rewrite behavior, the output should make
shared datetime timelines discoverable by explaining that `datetime_shift.id`
establishes a reusable shift anchor and `datetime_shift.use` reuses an earlier
anchor. The explanation should also distinguish timeline reuse from value
reuse by documenting rewrite `capture_as`, which creates
`@{<capture_as>_original}` and `@{<capture_as>_rewritten}` variables.

When a topic covers runbook output cleanup behavior, the explanation should
describe current first-class output fields such as `trim_empty_lines` and make
their allowed values and intent discoverable without requiring the raw spec.

When a topic covers command teardown behavior, the explanation should describe
manual `cleanup` as a first-class `Command` field, explain that it may be
authored as a string or array, and make it clear that explicit `cleanup`
replaces the automatic process-cleanup fallback for that command entry.

When a topic covers run troubleshooting behavior, the explanation should
describe global `--debug` and entry-scoped `debug: true` for supported entry
types such as `Command` and `Patch`, and should make it clear that entry-scoped
debug enables diagnostics for only that entry when global debug is not enabled.

When a topic covers captured variables and Markdown authoring, the explanation
should describe the `@{name}` interpolation syntax, the `@@{name}` escape
syntax, and the current boundary that Markdown may interpolate values captured
earlier or later in the runbook.

When a topic covers command output captions, the explanation should state that
`output.caption` can interpolate captured variables, including captures
produced by the same `Command` entry before the caption is rendered.

When a topic covers command capture behavior, the explanation should document
the `capture` rule shape enough for agents to author a valid rule without
guessing enum names. In particular, `run` should explain that `capture.source`
is currently `stdout`, `capture.stage` is either `raw` or `rewritten`, `raw`
captures before `output.rewrite`, `rewritten` captures after `output.rewrite`,
and the first regex capture group is stored when present.

When a topic covers runbook output stream selection, the explanation should
describe `output.stream`, its supported values, and the boundary between
rendered output selection versus the narrower existing `capture.source` and
assertion-check source contracts. For `run`, the explanation should also state
the current default when `output.stream` is omitted.

When `--output-format=skill` is selected:

- The command emits one aggregate skill document for the current `sw` build
  rather than a per-topic explanation.
- The emitted document is a valid Codex `SKILL.md` file with YAML frontmatter
  delimited by `---`.
- The YAML frontmatter includes at least `name` and `description`.
- The default file destination for `--output-file` without a value is the
  standard Codex skill path for the `sw` skill, such as
  `~/.codex/skills/sw/SKILL.md`.
- The skill content is intentionally minimal and acts as a routing layer rather
  than an embedded contract dump.
- The skill content should direct agents to start with `sw explain --all`.
- The skill content should treat `sw` output as authoritative over cached
  assumptions.
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
- [ ] Given `sw explain run`, the CLI documents that `datetime_shift.id`
      establishes a shared shift anchor and `datetime_shift.use` reuses an
      earlier anchor.
- [ ] Given `sw explain run`, the CLI documents rewrite `capture_as` generated
      variables and makes it clear that this reuses rewritten values rather
      than a shared timeline.
- [ ] Given `sw explain run`, the CLI documents `output.trim_empty_lines` and
      its supported values for trimming leading and trailing empty output
      lines.
- [ ] Given `sw explain run`, the CLI documents `cleanup` as a `Command`
      field for manual teardown and makes it clear that explicit `cleanup`
      replaces automatic process cleanup for that entry.
- [ ] Given `sw explain run`, the CLI documents entry-scoped `debug: true` for
      supported entries such as `Command` and `Patch`, and explains the
      boundary between that setting and global `--debug`.
- [ ] Given `sw explain run`, the CLI documents Markdown interpolation with
      `@{name}` and the `@@{name}` escape syntax for captured variables.
- [ ] Given `sw explain run`, the CLI makes it clear that Markdown entries may
      interpolate values captured earlier or later in the runbook.
- [ ] Given `sw explain run`, the CLI makes it clear that `output.caption` may
      interpolate captures produced by the same `Command` entry.
- [ ] Given `sw explain run`, the CLI documents the supported `capture.stage`
      values `raw` and `rewritten`.
- [ ] Given `sw explain run`, the CLI documents that `capture.source` is
      currently limited to `stdout`.
- [ ] Given `sw explain run`, the CLI documents that capture stores the first
      regex capture group when present, otherwise the full regex match.
- [ ] Given `sw explain run`, the CLI documents `output.stream` and its
      supported values `stdout`, `stderr`, and `combined`.
- [ ] Given `sw explain run`, the CLI documents that omitted `output.stream`
      defaults to `combined`.
- [ ] Given `sw explain run`, the CLI makes it clear that `output.stream`
      changes rendered output only and does not broaden `capture.source` or
      assertion-check sources beyond their current contracts.
- [ ] Given `sw explain validate`, the CLI prints a concise explanation of the
      validate contract derived from `SPEC-002`.
- [ ] Given `sw explain validate`, the CLI documents `--input-file=-` for
      stdin input, JSON as the default stdin format, and `--input-format=yaml`
      for YAML stdin input.
- [ ] Given `sw explain --all`, the CLI prints explanations for all supported
      topics.
- [ ] Given an agent-oriented user question such as "how do I check for Java
      21?", the documented `explain` contract makes it clear that `explain` is
      the correct discovery command rather than `help`.
- [ ] Given an agent-oriented user question about configuration shape, the
      documented `explain` contract makes it clear when the next step should be
      `sw example <topic>`.
- [ ] Given `sw explain example`, the CLI makes it clear that the `Command`
      example includes current nested fields such as `trim_empty_lines`,
      `stream`, `cleanup`, and `debug`.
- [ ] Given `sw explain example`, the CLI makes it clear that the `DisplayFile`
      example includes the Java `collapse_method_body` transform for
      collapsing method bodies.
- [ ] Given `sw explain run` or `sw explain check`, the CLI documents
      `--input-file=-` for stdin input, JSON as the default stdin format, and
      `--input-format=yaml` for YAML stdin input.
- [ ] Given `sw explain run`, `sw explain check`, or `sw explain validate`,
      the CLI makes it clear that `--input-format=json|yaml` without
      `--input-file=-` keeps the existing file-backed default behavior.
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
- [ ] Given `sw explain --output-format=skill`, the generated document starts
      with YAML frontmatter delimited by `---`.
- [ ] Given `sw explain --output-format=skill`, the generated YAML frontmatter
      includes `name: sw` and a non-empty `description`.
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
- [ ] Given `sw explain --output-format=skill`, the generated skill content is
      concise and routes agents to `sw explain --all` instead of embedding the
      full command map.
- [ ] Given `sw explain --output-format=skill`, the generated skill content
      tells agents to treat `sw` output as authoritative over cached
      assumptions.
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
