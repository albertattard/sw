# ADR-0001: Adopt Specification-Driven Development with AI Assistance

- Status: Accepted
- Date: 2026-03-01

## Context

Sociable Weaver is being developed as a learning-oriented project to evaluate
how effective specification-driven development is when combined with AI coding
agents.

The intent is to keep all product knowledge in repository-managed specification
files instead of external documentation systems.

In this model:
- Specifications in this repository are the source of truth.
- Code is generated and evolved from those specifications.
- Human effort is focused primarily on writing and refining specifications,
  with minimal direct coding.

The motivation is to gain first-hand experience and evaluate:
- where this approach is effective,
- where it is less effective,
- when it should be used, and
- when it should be avoided.

## Decision

Use specification-driven development with AI assistance as the default workflow
for this project.

## Alternatives Considered

### Conventional Code-First Development

Pros:
- Familiar workflow with direct control over implementation details.
- Lower dependency on specification quality and AI interpretation.

Cons:
- Product intent can drift from implementation over time.
- Harder to reuse requirements as structured knowledge for search/RAG systems.

### Specs Stored Outside the Repository

Pros:
- Can integrate with dedicated documentation tools.
- Non-code collaborators may already be familiar with external systems.

Cons:
- Source of truth becomes fragmented.
- Harder to version specs and code together atomically.

## Consequences

Positive:
- Requirements, rationale, and implementation intent stay in version control.
- Better alignment with AI-assisted generation from structured text inputs.
- Creates reusable, machine-readable project knowledge for future automation.
- Provides concrete learning data on the practical strengths and limits of the approach.

Negative:
- Specification quality becomes a critical dependency.
- Additional discipline is required to keep specs complete and current.
- AI-generated output still requires review and validation.

## Follow-up

- Treat `docs/spec/` as authoritative when code and docs diverge.
- Track workflow findings (what worked, what failed, why) as the project evolves.
- Revisit this decision after meaningful project milestones to decide whether to continue,
  refine, or reduce this approach.
