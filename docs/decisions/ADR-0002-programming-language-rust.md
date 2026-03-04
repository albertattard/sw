# ADR-0002: Use Rust as the Primary Implementation Language

- Status: Accepted
- Date: 2026-03-02

## Context

Sociable Weaver is a cross-platform CLI application (`sw`) targeting Linux,
macOS, and optionally Windows.

The project needs a language that supports native binaries, good CLI ergonomics,
and fast developer iteration.

The main candidates considered were Rust, Go, and Java.

- Rust is a language I am already familiar with.
- Go has a stronger and easier cross-compilation story, but I am not familiar
  with this language.
- Java is my preferred language for general backend work, but native-image
  options are less attractive for this CLI use case than before, given that
  GraalVM will not be part of the language
  ([reference](https://blogs.oracle.com/java/detaching-graalvm-from-the-java-ecosystem-train)).

## Decision

Use Rust as the primary implementation language for Sociable Weaver.

## Alternatives Considered

### Go

Pros:
- Excellent cross-compilation workflow.
- Simple deployment model for CLI binaries.

Cons:
- Lower personal familiarity at this stage.
- Higher near-term delivery risk due to onboarding cost.

### Java

Pros:
- Strong personal preference and deep experience.
- Mature ecosystem and tooling.

Cons:
- Current native-image path is less aligned with project needs.
- Less attractive for this CLI scenario compared with Rust.

## Consequences

Positive:
- Faster initial delivery by using existing language familiarity.
- Strong performance and native binary support for CLI workloads.
- Lower cognitive overhead for day-to-day development.

Negative:
- Cross-compilation, especially across all target OSes, may require additional
  setup versus Go.
- Some contributors may face a steeper Rust learning curve.

## Follow-up

- Keep CI optimized for Linux-first builds.
- Revisit this decision if delivery speed, hiring needs, or cross-platform
  packaging complexity change materially.
