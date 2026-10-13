---
id: TASK-199
title: Evaluate AI Agent Use Of SW
status: pending
category: discovery
related_features:
  - SPEC-001
  - SPEC-002
  - SPEC-003
  - SPEC-008
  - SPEC-009
owner: albertattard
created: 2026-10-08
updated: 2026-10-08
---

## Summary

Establish an evidence-based baseline for how effectively and efficiently AI
agents use `sw` to author, repair, and execute documentation runbooks. Evaluate
real task outcomes rather than treating the presence of guidance in CLI output
as proof that models can apply it.

The unit under evaluation is the combination of model, reasoning settings,
agent tools and instructions, environment, and `sw` version. This task concerns
agents using the product, rather than developing its implementation.

## Scope

- Define a repeatable evaluation protocol and a bounded benchmark of realistic
  user tasks, initially targeting 12 tasks.
- Establish a baseline using the generated `sw` skill, initially proposing
  three attempts per task across two representative models (72 attempts).
- Compare discovery approaches on the same tasks: CLI only, CLI plus generated
  skill, and CLI plus the full contract supplied upfront.
- Record outcomes, efficiency measurements, transcripts, and failure causes.
- Recommend bounded follow-up improvements supported by the results.

## Non-goals

- Changing CLI behavior, discovery output, or generated skill guidance as part
  of establishing the baseline.
- Building a general-purpose agent evaluation platform.
- Ranking models independently of their agent environments.
- Treating pilot results as precise estimates of production reliability.

## Evaluation Dimensions

| Dimension | Measurement |
| --- | --- |
| Correctness | Requested behavior and generated documentation verified against independent checks |
| Reliability | Success rate across repeated attempts, reported by task family |
| Discovery | Ability to find current syntax and behavior without inventing fields or flags |
| Recovery | Ability to repair mistakes using validation and execution diagnostics |
| Scope control | Preservation of unrelated files and adherence to permitted execution scope |
| Efficiency | Tokens, elapsed time, tool calls, failed commands, and human interventions |

Correctness and required scope constraints are success gates. Report cost per
successful completion while accounting for failed attempts; do not reward cheap
incorrect results. Distinguish first-attempt validity from eventual success
after repair.

## Candidate Scenarios

The final benchmark should include common workflows and known discovery traps:

- Author a Java 21 prerequisite and verify behavior against controlled Java
  versions.
- Import a README while preserving required prose and commands and reviewing
  inferred executable entries.
- Repair an invalid runbook without changing its intended behavior.
- Capture command output and interpolate the value into generated prose.
- Normalize related timestamps while preserving their relative intervals.
- Run YAML through stdin with the correct input format.
- Resolve ambiguous default runbook discovery using an explicit input file.
- Generate Markdown to a requested destination while preserving an existing
  README.
- Diagnose a missing prerequisite without executing the full workflow.
- Execute a workflow with a background process and verify owned-process cleanup.
- Convert between YAML and JSON without changing runbook semantics.
- Update documentation through its runbook source rather than editing generated
  Markdown directly.

Prompts should describe user intent without revealing the implementation or
required discovery commands. Accept alternative valid solutions.

## Protocol And Grading

- Define initial files, controlled environment, permitted actions, expected
  final state, forbidden side effects, and time/token budgets for each case.
- Use a fresh agent session and reset isolated workspace for every attempt.
  Avoid dependencies on changing external services where practical.
- Pin and record the `sw` build, model identity, reasoning settings, agent
  instructions, tool configuration, and environment. Record the generated
  skill or supplied contract used in each condition.
- Keep configurations and budgets fixed within discovery comparisons; vary
  only the guidance condition and balance trial order.
- Prefer automated checks of validation, execution behavior, generated output,
  filesystem changes, exit codes, and process cleanup. Keep grading checks
  outside the agent's editable workspace.
- Do not treat structural validation or the agent's completion claim as proof
  that the requested workflow is correct.
- Use explicit human-review criteria for documentation quality and intent
  preservation. Calibrate any model-based grader against human judgments.
- Retain observable transcripts and intermediate artifacts for diagnosis.
  Separate model failures from environment and grading failures.
- Hold back unseen scenario variants for checking whether later improvements
  generalize beyond the development benchmark.

The targeted-discovery guidance introduced by TASK-175 is a hypothesis to test:
does it reduce unnecessary context without lowering task success? Do not grade
agents on a prescribed discovery sequence unless a specific step is required
for correctness or scope control.

## Deliverables

- Evaluation protocol, task fixtures, prompts, and independently reviewable
  success checks.
- Baseline report with per-task and per-model outcomes, repeated-attempt
  reliability, efficiency measurements, and reproducible run metadata.
- Discovery comparison showing the value and cost of each guidance condition.
- Failure analysis distinguishing missing guidance, misunderstood guidance,
  insufficient diagnostics, awkward CLI behavior, model/agent limitations, and
  evaluation defects.
- Prioritized follow-up tasks, with proposed success thresholds informed by
  the baseline. Any user-visible changes must update the relevant spec before
  implementation.

## Acceptance Criteria

- [ ] The protocol defines task success, scope constraints, budgets, repeated
      attempts, environment reset, grading, and reproducibility metadata.
- [ ] A benchmark of at least 12 realistic tasks has reviewed outcome checks
      covering discovery, authoring, repair, execution, and scope control.
- [ ] A baseline has been run across at least two model configurations with
      at least three attempts per task, or a documented rationale explains an
      intentionally revised pilot size.
- [ ] CLI-only, generated-skill, and full-contract conditions have been
      compared under otherwise equivalent configurations on a documented set
      of tasks.
- [ ] Results report correctness separately from efficiency, include failures
      in cost accounting, and explain the limits of the sample size.
- [ ] Failed attempts have been inspected and classified, with environment or
      grading defects identified separately from agent mistakes.
- [ ] Unseen variants are reserved for evaluating subsequent improvements.
- [ ] Evidence-backed recommendations and proposed follow-up thresholds are
      recorded without making unmeasured product changes in this task.

## References

- [SPEC-009: Explain Feature Contract](../../spec/SPEC-009-explain-feature-contract.md)
- [TASK-175: Use Targeted Skill Discovery](../explain/TASK-175-use-targeted-skill-discovery.md)
- [Anthropic: Demystifying evals for AI agents](https://www.anthropic.com/engineering/demystifying-evals-for-ai-agents)

## Notes

Creating this record does not execute the evaluation. Model selection, tooling,
pilot budgets, and report location remain decisions for task execution. Current
CLI integration tests remain the deterministic product regression suite; agent
evaluations provide a separate measure of usability and task completion.
