---
id: TASK-168
title: Support Lowercase Dockerfile Fence Label
status: done
category: display-file
---

## Context

Markdown renderers and syntax highlighters more consistently recognize the
lowercase `dockerfile` fence label than the capitalized `Dockerfile` label.
The existing Dockerfile support used the capitalized label because it matched
the file name, but that made generated Markdown less portable than the other
lowercase content type labels.

## Scope

- Accept `DisplayFile.content_type: dockerfile` and
  `DisplayUrl.content_type: dockerfile` during validation
- Render `content_type: dockerfile` as a `dockerfile` fenced block
- Preserve `content_type: Dockerfile` as a compatible spelling that renders a
  `Dockerfile` fenced block
- Render auto-detected `Dockerfile` and `Dockerfile-*` file names as
  `dockerfile` fenced blocks
- Update user-facing help, explain, spec, guide, and automated tests

## Acceptance Criteria

- [x] Given a `DisplayFile` entry with `content_type: dockerfile`, validation
      accepts the runbook.
- [x] Given a `DisplayFile` entry with `content_type: dockerfile`, `sw run`
      renders a `dockerfile` fenced block.
- [x] Given a `DisplayFile` entry with `content_type: Dockerfile`, `sw run`
      continues to render a `Dockerfile` fenced block.
- [x] Given a `DisplayFile` entry that references `Dockerfile-Java8` without a
      `content_type`, `sw run` renders a `dockerfile` fenced block.
- [x] Help, explain, spec, and guide text describe both accepted Dockerfile
      content type spellings.
