---
id: TASK-160
title: Support DisplayUrl Entries
status: done
category: display-file
related_features:
  - SPEC-003
owner: albertattard
created: 2026-06-01
updated: 2026-06-01
---

## Summary

Add `DisplayUrl` entries so runbooks can render remote text content, such as a
raw GitHub Markdown file, without overloading local `DisplayFile.path`
semantics.

## Scope

- Add `DisplayUrl` as a supported runbook entry type
- Require `DisplayUrl.url`
- Accept absolute `http` and `https` URLs
- Fetch URL content during `sw run`
- Render fetched content in a fenced code block
- Support `timeout`, `start_line`, `line_count`, `indent`, `offset`, and
  `content_type`
- Reuse `DisplayFile` content type detection where the URL path has a known
  extension
- Add example, help, explain, validation, and run coverage

## Assumptions

- `DisplayUrl` is a rendering feature only and does not execute fetched
  content.
- Network availability is a runtime concern; validation checks URL shape and
  entry fields but does not fetch the URL.
- Remote content should stay explicit instead of being hidden behind
  `DisplayFile.path`.

## Acceptance Criteria

- [x] Given a valid `DisplayUrl` entry, `sw run` fetches the URL and renders
      the response body in a fenced block.
- [x] Given `DisplayUrl.content_type`, the generated fenced block uses that
      content type.
- [x] Given a recognized URL path extension and no `content_type`, the
      generated fenced block uses the detected content type.
- [x] Given `DisplayUrl.start_line` and `DisplayUrl.line_count`, only the
      requested slice is rendered.
- [x] Given `DisplayUrl.indent` and `DisplayUrl.offset`, rendering applies the
      same block and content shifts as `DisplayFile`.
- [x] Given an unsupported URL scheme, validation fails.
- [x] Given a missing or invalid `url`, validation fails.
- [x] Help, explain, and example output describe `DisplayUrl`.
- [x] Automated tests pass after the change.

## Notes

This keeps local file display and remote content display separate so path
resolution, network failure, and timeout semantics stay clear.
