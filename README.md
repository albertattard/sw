# Sociable Weaver (SW)

Sociable Weaver is a Rust CLI project built using a specification-first workflow.

Primary command name: `sw` (short name: `SW`).

## Source of Truth

The product specification in `docs/spec/` is the authoritative source for behavior and requirements.
Code is an implementation of those specs.

## Working Model

1. Define or update the feature spec first.
2. Implement code that satisfies the spec.
3. Mark feature status only after acceptance criteria pass.

## Documentation Structure

```text
docs/
  spec/
    00-product-vision.md
    features/
      FEAT-001-*.md
  decisions/
    ADR-001-*.md
  tasks/
    TASK-001-*.md
```

- `docs/spec/features/`: feature specifications, expected behavior, and acceptance criteria.
- `docs/decisions/`: architecture decision records (ADRs) and technical tradeoff rationale.
- `docs/tasks/`: file-based task tracking for pending, in-progress, blocked, and completed work.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
