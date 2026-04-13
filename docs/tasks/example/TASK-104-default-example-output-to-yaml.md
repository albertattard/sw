---
id: TASK-104
title: Default Example Output To YAML
status: pending
category: example
related_features:
  - SPEC-008
owner: @aattard
created: 2026-04-13
updated: 2026-04-13
---

## Summary

Change `sw example` so it emits YAML by default, while still allowing users and
agents to request JSON explicitly when they need the machine-oriented shape.

## Scope

- Add `--output-format yaml|json` to `sw example`
- Make YAML the default output format
- Preserve JSON output through `--output-format json`
- Update help text and integration tests for both output modes

## Assumptions

- YAML now better matches how users are expected to author and edit runbooks in
  this repository.
- JSON examples still matter for users and agents that want the explicit object
  shape without YAML serialization choices.
- The same logical example should be representable in both output formats.

## Acceptance Criteria

- [ ] Given `sw example Command`, the CLI exits with `0` and prints valid YAML.
- [ ] Given `sw example Command --output-format yaml`, the CLI exits with `0`
      and prints the same YAML shape as the default mode.
- [ ] Given `sw example Command --output-format json`, the CLI exits with `0`
      and prints valid JSON.
- [ ] Given `sw example DisplayFile`, the CLI exits with `0` and prints valid
      YAML, including supported fields such as `indent` and transforms.
- [ ] Help output documents `--output-format yaml|json` and YAML as the
      default.

## Notes

This is a user-visible CLI behavior change and must stay aligned with
`SPEC-008`.
