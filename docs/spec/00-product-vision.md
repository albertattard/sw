# Product Vision

## Problem

Software engineers often learn from online examples, but many examples stop
working over time.

Most examples were likely correct when published, but platforms, tools, and
dependencies evolve. As a result, copied commands may fail, instructions may
drift, and readers lose trust in the documentation.

This creates a recurring pain:
- You follow the guide exactly.
- The command does not work.
- You spend time debugging the example instead of learning the topic.

## Solution

Sociable Weaver (`sw`) is a specification-driven CLI and workflow that treats
examples as executable assets, not static text.

Instead of writing a README first and hoping the examples stay valid, this
project keeps a repository-backed runbook (typically defined in structured
files such as JSON) that can:
- run commands,
- apply file edits (for example via patches), and
- validate outcomes.

Documentation is then generated or updated from verified steps, so instructions
in README files reflect what was actually executed successfully.

## Vision

Build a practical system where trustworthy examples are the default:
- Specs define what should happen.
- Runbooks execute and verify those specs.
- README content is derived from tested behavior.

In short: documentation should be reproducible, testable, and continuously
validated.

## Target Users

- Primary: software engineers who publish tutorials, snippets, and CLI guides.
- Secondary: teams maintaining internal runbooks or onboarding documentation.

## Scope

In scope:
- Defining examples and workflows as version-controlled specifications.
- Executing and validating commands from those specs.
- Producing documentation from validated runs.

Out of scope (for now):
- Replacing full documentation platforms.
- Supporting every language/runtime from day one.

## Success Criteria

- Examples in project documentation are executable and reproducible.
- Drift between instructions and reality is detected early.
- Updating docs becomes part of a verified workflow, not a manual copy/paste task.
- The approach produces clear learning outcomes on when spec-driven,
  AI-assisted development is effective and when it is not.
