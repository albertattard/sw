# Sociable Weaver (SW)

Sociable Weaver (`sw`) is a Rust CLI for executable documentation. It lets
authors define documentation examples as version-controlled runbooks in YAML or
JSON, validate and execute those workflows, and generate README-style output
from verified runs.

The goal is to stop example-driven documentation from silently drifting out of
date. Instead of copying commands into Markdown and hoping they still work
later, `sw` treats those examples as executable specifications that can be
checked locally and in CI.

The broader product direction is described in the
[`Product-Vision.md`](./docs/spec/Product-Vision.md).

## What You Can Do With `sw`

- [`sw validate`](./docs/spec/SPEC-002-validate-runbook.md) checks that a
  runbook is structurally valid without executing it.
- [`sw check`](./docs/spec/SPEC-005-check-prerequisites.md) validates the
  runbook and runs only prerequisite checks.
- [`sw run`](./docs/spec/SPEC-003-run-runbook-to-markdown.md) executes the
  runbook and writes generated Markdown output.
- [`sw import`](./docs/spec/SPEC-006-import-readme-to-runbook.md) turns an
  existing `README.md` into a starter runbook.
- [`sw help`](./docs/spec/SPEC-001-help-and-discovery.md),
  [`sw explain`](./docs/spec/SPEC-009-explain-feature-contract.md), and
  [`sw example`](./docs/spec/SPEC-008-example-snippets.md) support discovery
  and authoring.

## Typical Workflow

1. Write or import a runbook in YAML or JSON.
2. Run `sw validate` or `sw check`.
3. Run `sw run` to execute the workflow and generate the documentation output.

## Engineering Workflow

This repository is developed with AI coding agents under a specification-first
engineering workflow.

The working model is:

1. Define or update the specification first.
2. Implement code that satisfies the spec.
3. Mark specification status only after acceptance criteria pass.

The main project records are:

- [`docs/spec/README.md`](./docs/spec/README.md): executable specifications and
  the product contract.
- [`docs/spec/Product-Vision.md`](./docs/spec/Product-Vision.md): the product
  thesis, problem, and long-term direction.
- [`docs/tasks/README.md`](./docs/tasks/README.md): file-based task tracking
  for pending, in-progress, and completed work.
- [`docs/decisions/README.md`](./docs/decisions/README.md): architecture
  decision records and technical tradeoffs.
- [`ADR-0001`](./docs/decisions/ADR-0001-specification-driven-development-with-ai.md):
  why this project uses specification-driven development with AI assistance.

The main implementation areas are:

- [`src/main.rs`](./src/main.rs): CLI entrypoint and command dispatch.
- [`src/commands/`](./src/commands/): top-level command implementations.
- [`src/runbook/`](./src/runbook/): runbook loading, validation, execution, and
  rendering.

## Quality Checks

Use these commands from the repository root:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

This is also the verification sequence used before the `commit changes`
workflow creates and pushes a commit.

The repository pins Rust `1.94.1` through `rust-toolchain.toml` so local
development and CI use the same compiler, formatter, and linter versions.

## Dependency Hygiene

Dependency freshness is handled through weekly Dependabot updates for Cargo
crates and GitHub Actions.

Dependency advisories and duplicate-version drift are checked in CI with
`cargo-deny`. To mirror that check locally, install it once and then run:

```bash
cargo install cargo-deny --locked
cargo deny check advisories bans
```

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
