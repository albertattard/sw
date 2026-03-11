# Repository Guidelines

## Git workflow
- Only create a commit when the user explicitly asks.
- Only push when the user explicitly asks.
- Write commit messages in this format:
  - Subject line: imperative verb + outcome
  - Example: `Add validate subcommand`
- After the subject, include a short business-oriented description explaining why the change was made.
- Focus the description on product intent, maintainability, usability, or future growth, not on low-level code mechanics.
- The description may mention structural decisions when they support future capabilities.
- Avoid code-centric summaries as the primary explanation.

Example:

```text
Add validate subcommand

Allow users and agents to verify that a runbook is structurally valid before
trying to execute it. This also reorganizes the CLI so new commands can be
added cleanly as the application grows, instead of concentrating behavior in
main.rs. An anonymised fixture is included to support realistic testing without
tying the test suite to a specific project.
```
