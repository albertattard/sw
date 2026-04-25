# Runbook Entity Editing Guide

This guide explains the runbook entry types you use when editing
`sw-runbook.yaml` by hand.

Runbooks are ordered documents. Each item under `entries` is rendered,
executed, or both, in the order it appears.

```yaml
entries:
  - type: Heading
    level: H1
    title: Example

  - type: Markdown
    contents: |
      Explain what the reader is about to do.
```

Use this guide for authoring choices. For the current exact contract, use:

```shell
sw explain run
sw example Command
```

## Choosing An Entity

- Use `Heading` to structure the generated Markdown.
- Use `Markdown` for prose, lists, links, and explanatory text.
- Use `DisplayFile` to show source code or configuration from the repository.
- Use `Prerequisite` to document and verify tools that must exist before the
  workflow runs.
- Use `Patch` to temporarily modify a file during a run and restore it
  afterward.
- Use `Command` to run shell commands and render verified output.

## Heading

Use `Heading` when the generated document needs a Markdown heading.

```yaml
- type: Heading
  level: H2
  title: Build The Application
```

Use headings to keep generated output navigable. Avoid using `Markdown`
contents such as `## Build The Application` when the intent is a structural
heading.

## Markdown

Use `Markdown` for normal explanatory text.

```yaml
- type: Markdown
  contents: |
    This example starts the application and verifies that the health endpoint
    responds successfully.
```

Markdown can interpolate captured variables:

```yaml
- type: Markdown
  contents: |
    The generated image is `@{image_name}`.
```

Use `@@{name}` when you want the literal text `@{name}` in the generated
Markdown.

Use `indent` when this entry should be nested inside a larger Markdown
structure:

```yaml
- type: Markdown
  indent: 2
  contents: |
    This line is rendered with two leading spaces.
```

## DisplayFile

Use `DisplayFile` to show file contents without executing them.

```yaml
- type: DisplayFile
  path: ./src/main/java/demo/Example.java
  start_line: 1
  line_count: 12
```

Use line ranges when only part of a file is relevant. Use `indent` to indent
the whole fenced block in the generated Markdown.

```yaml
- type: DisplayFile
  path: ./src/main/resources/db/migration/V1__create_database.sql
  start_line: 18
  line_count: 18
  indent: 2
```

`DisplayFile` currently recognizes common file extensions such as `.java`,
`.sql`, and `.xml` for fenced-code labels. Unknown extensions render as text.

For Java examples, `transform` can collapse method bodies when the surrounding
code is more important than the implementation details:

```yaml
- type: DisplayFile
  path: ./src/main/java/demo/Example.java
  transform:
    language: java
    operations:
      - type: collapse_method_body
        name: initialize
```

## Prerequisite

Use `Prerequisite` to document and verify tools required by the runbook.

```yaml
- type: Prerequisite
  checks:
    - kind: java
      name: Java 25+
      version: 25+
      contents:
        - "- [Java downloads](https://www.oracle.com/java/technologies/downloads/)"
      help: Install Java 25 or newer and make sure `java` is available on the PATH.
```

Prefer built-in prerequisite kinds when they exist. Use command-based checks
only when there is no built-in check for the tool.

`sw check` runs prerequisites without running normal workflow commands.

## Patch

Use `Patch` when the runbook needs to temporarily alter a file to demonstrate a
change.

```yaml
- type: Patch
  path: ./src/main/java/demo/Main.java
  patch: |
    @@ -10,3 +10,3 @@
    -        return oldValue;
    +        return newValue;
```

Patch entries are applied during the run and restored afterward by default.
Use them for controlled demonstrations, not for permanent project changes.

## Command

Use `Command` to run shell commands as part of the executable documentation.

```yaml
- type: Command
  commands: |
    ./mvnw test
  assert:
    exit_code: 0
```

Use `assert.exit_code` when a command is expected to fail:

```yaml
- type: Command
  commands: |
    ./mvnw org.owasp:dependency-check-maven:check
  assert:
    exit_code: 1
```

Use `output` when command output should appear in the generated Markdown:

```yaml
- type: Command
  commands: |
    curl --silent --show-error http://localhost:8080/health
  output:
    caption: Health response
    content_type: json
    stream: stdout
```

Use `working_dir` when the command should run from a runbook-relative
directory:

```yaml
- type: Command
  working_dir: reverse-proxy
  commands: |
    docker build \
      --file Dockerfile \
      --tag reverse-proxy:local \
      .
```

Use `cleanup` when the command starts resources that must be released:

```yaml
- type: Command
  commands: |
    java -jar ./target/app.jar > ./target/app.log 2>&1 &
    echo "$!" > ./target/app.pid
  cleanup: |
    if [ -f ./target/app.pid ]; then
      kill "$(cat ./target/app.pid)" 2>/dev/null || true
      rm -f ./target/app.pid
    fi
```

Use `capture` when later entries need a value from command output:

```yaml
- type: Command
  commands: |
    printf 'image=demo:1.2.3\n'
  capture:
    - name: image_name
      source: stdout
      stage: raw
      pattern: 'image=(.+)'
```

`capture.source` is currently limited to `stdout`. Use `stage: raw` to match
the original stdout before `output.rewrite`, or `stage: rewritten` to match the
stdout after rewrite rules have been applied. When `pattern` contains a regex
capture group, `sw` stores the first captured group; otherwise it stores the
full match.

For example, this captures `109` from `Computed in 109 ms`:

```yaml
- type: Command
  commands: |
    echo 'Computed in 109 ms'
  capture:
    - name: elapsed_ms
      source: stdout
      stage: raw
      pattern: 'Computed in (\d+) ms'

- type: Markdown
  contents: |
    The command took @{elapsed_ms} ms.
```

Use `output.rewrite` for stable generated output. For dates and times, prefer
`datetime_shift` over literal replacement:

```yaml
- type: Command
  commands: |
    curl --silent --show-error http://localhost:11434/api/tags | jq
  output:
    rewrite:
      - type: datetime_shift
        format: rfc3339
        id: api_timeline
        capture_as: model_timestamp
```

`datetime_shift.id` and `datetime_shift.use` reuse a shared timeline.
`capture_as` creates `@{model_timestamp_original}` and
`@{model_timestamp_rewritten}` for later entries.

Use `debug: true` when a single command entry needs rewrite and capture
diagnostics:

```yaml
- type: Command
  debug: true
  commands: |
    echo "hello"
```

## Editing Workflow

1. Start from `sw init`, `sw import`, or `sw example <topic>`.
2. Prefer YAML block scalars with `|` for multiline Markdown, commands, and
   cleanup scripts.
3. Run `sw validate` after editing structure.
4. Run `sw check` before full execution when prerequisites matter.
5. Run `sw run --verbose-mode plain` when you want line-oriented progress.

## Agent Notes

AI agents should not treat this guide as the complete schema. Use the installed
binary for current discovery:

```shell
sw explain --all
sw example Command
sw example DisplayFile
sw example Prerequisite
sw example Patch
```

Use the specs for the full product contract when source access is available.
