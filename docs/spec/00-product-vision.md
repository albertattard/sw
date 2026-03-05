# Sociable Weaver (`sw`) — Vision

## One-Line Thesis

**Sociable Weaver makes documentation examples executable so they stay correct over time.**

## Problem

Software engineers often learn by following examples in README files, tutorials, and blog posts. Over time, many of these examples stop working.

Commands fail because tools evolve, dependencies change, and environments drift. Even when readers follow instructions exactly, the result often differs from what the author originally experienced.

This creates a common and frustrating pattern:

1. You follow the guide exactly.
2. The command fails.
3. You spend time debugging the documentation instead of learning the topic.

The documentation was correct when written—but the ecosystem moved on. As examples silently decay, trust in technical documentation declines.

## Solution

Sociable Weaver (`sw`) is a CLI that keeps documentation examples executable and continuously verified.

Instead of embedding commands directly in Markdown and hoping they remain correct, authors define examples as **executable specifications** in version-controlled files (YAML or JSON).

These specifications describe:

- commands to run
- file changes to apply
- validations that confirm expected outcomes

The CLI executes these steps in a clean environment and verifies that they succeed. Documentation can then be generated or updated from the verified execution.

If an example breaks, the failure is detected immediately—during development or CI—rather than by readers months later.

In practice:

```
example specification
        ↓
   sw verify
        ↓
example executed and validated
        ↓
documentation generated or updated
```

The documentation always reflects steps that actually ran successfully.

## Workflow

**Today**

1. Write commands in a README
2. The ecosystem changes
3. Instructions silently break
4. Readers debug the documentation

**With `sw`**

1. Define the example as an executable specification
2. Run `sw verify` locally or in CI
3. The CLI executes and validates the workflow
4. Documentation is generated from verified results

Examples remain reproducible and trustworthy over time.

## Vision

Make **executable documentation the default for software projects**.

Sociable Weaver treats examples as living artifacts rather than static text. Each documented workflow becomes reproducible, executable, and continuously validated.

This creates a tight feedback loop:

- authors define examples as executable steps
- the CLI verifies those steps
- documentation reflects verified execution

When tools or dependencies change, failures surface immediately during development or CI instead of during a reader’s first attempt.

Because these examples are executable and validated, they also function as **behavioral specifications** of the system. Teams can use them to ensure that documented workflows remain aligned with the real implementation.

Over time, this approach enables documentation that is:

- reproducible
- continuously validated
- resistant to silent drift
- trusted by readers

## Why Now

Software ecosystems evolve rapidly, and documentation struggles to keep pace. At the same time, developers increasingly rely on example-driven learning—from open-source READMEs to AI-generated tutorials.

As the volume of generated examples grows, the risk of outdated or incorrect instructions increases. Ensuring that examples are **executable and verified** becomes essential for maintaining trust in documentation.

## Target Users

Primary users:

- open-source maintainers who want trustworthy README instructions
- engineers writing tutorials or developer guides
- teams maintaining internal onboarding or developer workflows

## Scope

**In scope**

- defining executable examples as version-controlled specifications
- executing commands and applying file modifications
- validating expected outcomes
- generating or updating documentation from verified runs

**Out of scope (for now)**

- replacing documentation platforms
- supporting every language or runtime from day one
- acting as a general workflow automation system

## Success Criteria

Sociable Weaver succeeds if:

- documentation examples remain executable over time
- broken instructions are detected automatically
- documentation verification becomes part of CI workflows
- developers trust that examples in documentation actually work

**Sociable Weaver ensures that what documentation says and what software actually does never silently diverge.**
